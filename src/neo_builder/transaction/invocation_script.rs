use std::hash::Hash;

use crate::{
	builder::{BuilderError, ScriptBuilder},
	codec::{Decoder, Encoder, NeoSerializable},
	crypto::{KeyPair, Secp256r1Signature},
	var_size, OpCode,
};
use getset::{Getters, Setters};
use serde_derive::{Deserialize, Serialize};

/// An invocation script is part of a witness and is simply a sequence of neo-vm instructions.
///
/// The invocation script usually is the input to the verification script.
///
/// In most cases it will contain a signature that is checked in the verification script.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters, Serialize, Deserialize)]
pub struct InvocationScript {
	/// This invocation script as a byte array
	#[getset(get = "pub", set = "pub")]
	script: Vec<u8>,
}

impl InvocationScript {
	/// Constructs an empty invocation script.
	pub fn new() -> Self {
		Self { script: Vec::new() }
	}

	/// Creates an invocation script with the given script.
	///
	/// It is recommended to use `InvocationScript::from_signature` or `InvocationScript::from_message_and_key_pair`
	/// when you need a signature invocation script.
	///
	/// # Arguments
	///
	/// * `script` - The script as a byte array
	pub fn new_with_script(script: Vec<u8>) -> Self {
		Self { script }
	}

	pub fn from_serialized_script(script: Vec<u8>) -> Self {
		let mut decoder = Decoder::new(&script);
		Self::decode(&mut decoder).unwrap()
	}

	/// Creates an invocation script from the given signature.
	///
	/// # Arguments
	///
	/// * `signature` - The signature to use in the script
	///
	/// # Returns
	///
	/// The constructed invocation script
	pub fn from_signature(signature: Secp256r1Signature) -> Self {
		let mut script = ScriptBuilder::new();
		let signature_bytes = signature.to_bytes();
		script.push_data(signature_bytes.to_vec());
		Self { script: script.to_bytes() }
	}

	/// Creates an invocation script from the signature of the given message signed with the given key pair.
	///
	/// # Arguments
	///
	/// * `message` - The message to sign
	/// * `key_pair` - The key to use for signing
	///
	/// # Returns
	///
	/// The constructed invocation script
	pub fn from_message_and_key_pair(
		message: Vec<u8>,
		key_pair: &KeyPair,
	) -> Result<Self, BuilderError> {
		let signature = key_pair.private_key.sign_tx(&message)?;
		Ok(Self::from_signature(signature))
	}

	/// Constructs an invocation script from the given signatures.
	///
	/// # Arguments
	///
	/// * `signatures` - The signatures
	///
	/// # Returns
	///
	/// The invocation script
	pub fn from_signatures(signatures: &[Secp256r1Signature]) -> Self {
		let mut builder = ScriptBuilder::new();
		for signature in signatures {
			let signature_bytes = signature.to_bytes();
			builder.push_data(signature_bytes.to_vec());
		}
		Self { script: builder.to_bytes() }
	}
}

impl InvocationScript {
	/// Unbundles the script into a list of signatures if this invocation script contains signatures.
	///
	/// # Returns
	///
	/// The list of signatures found in this script
	pub fn get_signatures(&self) -> Vec<Secp256r1Signature> {
		let mut reader = Decoder::new(&self.script);
		let mut sigs = Vec::new();
		while reader.available() > 0 && reader.read_u8() == OpCode::PushData1 as u8 {
			reader.read_u8(); // ignore opcode size
			if let Ok(signature) = Secp256r1Signature::from_bytes(&reader.read_bytes(64).unwrap()) {
				sigs.push(signature);
			}
		}
		sigs
	}
}

impl NeoSerializable for InvocationScript {
	type Error = BuilderError;

	fn size(&self) -> usize {
		return var_size(self.script.len()) + self.script.len();
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_var_bytes(&self.script);
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		let script = reader.read_var_bytes()?;
		Ok(Self { script })
	}
	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}

#[cfg(test)]
mod tests {
	use crate::neo_crypto::utils::ToHexString;

	use super::*;

	#[test]
	fn test_from_message_and_key_pair() {
		let message = vec![0u8; 10];
		let key_pair = KeyPair::new_random();
		let script =
			InvocationScript::from_message_and_key_pair(message.clone(), &key_pair).unwrap();
		let expected_signature = key_pair.private_key().sign_tx(&message).unwrap();
		let expected = format!(
			"{}40{}",
			OpCode::PushData1.to_hex_string(),
			hex::encode(expected_signature.to_bytes())
		);
		assert_eq!(hex::decode(&expected).unwrap(), script.script);
		assert_eq!(hex::decode(&format!("42{}", expected)).unwrap(), script.to_array());
	}

	#[test]
	fn test_serialize_random_invocation_script() {
		let message = vec![1; 10];
		let script = InvocationScript::new_with_script(message.clone());
		assert_eq!(message, script.script);
	}

	#[test]
	fn test_deserialize_custom_invocation_script() {
		let message = vec![1; 256];
		let script = format!("{}0001{}", OpCode::PushData2.to_hex_string(), hex::encode(&message));
		let serialized_script = format!("FD0301{}", script);
		let deserialized =
			InvocationScript::from_serialized_script(hex::decode(&serialized_script).unwrap());
		assert_eq!(deserialized.script, hex::decode(&script).unwrap());
	}

	#[test]
	fn test_deserialize_signature_invocation_script() {
		let message = vec![0u8; 10];
		let key_pair = KeyPair::new_random();
		let signature = key_pair.private_key().sign_tx(&message).unwrap();
		let script =
			format!("{}40{}", OpCode::PushData1.to_hex_string(), hex::encode(signature.to_bytes()));
		let deserialized = InvocationScript::from_serialized_script(
			hex::decode(&format!("42{}", script)).unwrap(),
		);
		assert_eq!(deserialized.script, hex::decode(&script).unwrap());
	}

	#[test]
	fn test_size() {
		let script = hex::decode("147e5f3c929dd830d961626551dbea6b70e4b2837ed2fe9089eed2072ab3a655523ae0fa8711eee4769f1913b180b9b3410bbb2cf770f529c85f6886f22cbaaf").unwrap();
		let s = InvocationScript::new_with_script(script);
		assert_eq!(s.size(), 65);
	}

	#[test]
	fn test_get_signatures() {
		let message = vec![0u8; 10];
		let key_pair = KeyPair::new_random();
		let signature = key_pair.private_key.sign_tx(&message).unwrap();
		let inv = InvocationScript::from_signatures(&vec![
			signature.clone(),
			signature.clone(),
			signature.clone(),
		]);
		inv.get_signatures().iter().for_each(|sig| assert_eq!(*sig, signature));
	}
}
