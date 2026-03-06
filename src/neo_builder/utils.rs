use serde::Serialize;
use serde_json::Value;

use crate::{
	builder::{
		BuilderError, ScriptBuilder, Signer, TransactionAttribute, TransactionSendToken,
		TransactionSigner,
	},
	crypto::Secp256r1PublicKey,
};
use neo3::prelude::*;
// pub type ScriptHash = H160;

/// Converts a list of public keys to a script hash using a given threshold.
///
/// # Arguments
///
/// * `public_keys` - A mutable slice of `Secp256r1PublicKey` instances.
/// * `threshold` - The minimum number of signatures required to validate the transaction.
///
/// # Returns
///
/// A `ScriptHash` instance representing the script hash of the MultiSig script.
pub fn public_keys_to_scripthash(
	public_keys: &mut [Secp256r1PublicKey],
	threshold: usize,
) -> ScriptHash {
	match try_public_keys_to_scripthash(public_keys, threshold) {
		Ok(script_hash) => script_hash,
		Err(err) => {
			tracing::warn!(error = %err, "Failed to build multi-sig script; returning zero script hash");
			ScriptHash::zero()
		},
	}
}

pub fn try_public_keys_to_scripthash(
	public_keys: &mut [Secp256r1PublicKey],
	threshold: usize,
) -> Result<ScriptHash, BuilderError> {
	let threshold_u8 =
		u8::try_from(threshold).ok().filter(|value| *value > 0).ok_or_else(|| {
			BuilderError::SignerConfiguration(
				"multi-sig threshold must be greater than zero".to_string(),
			)
		})?;

	if threshold > public_keys.len() {
		return Err(BuilderError::SignerConfiguration(format!(
			"multi-sig threshold {} exceeds public key count {}",
			threshold,
			public_keys.len()
		)));
	}

	let script = ScriptBuilder::build_multi_sig_script(public_keys, threshold_u8)?;
	Ok(ScriptHash::from_script(&script))
}

/// Converts a public key to a script hash.
///
/// # Arguments
///
/// * `public_key` - A `Secp256r1PublicKey` instance.
///
/// # Returns
///
/// A `ScriptHash` instance representing the script hash of the verification script.
pub fn pubkey_to_scripthash(public_key: &Secp256r1PublicKey) -> ScriptHash {
	let script = ScriptBuilder::build_verification_script(public_key);
	ScriptHash::from_script(&script)
}

pub trait VecValueExtension {
	fn to_value(&self) -> Value;
}

fn serialize_to_value<T: Serialize>(value: &T, type_name: &str) -> Value {
	serde_json::to_value(value).unwrap_or_else(|err| {
		tracing::warn!(error = %err, item_type = type_name, "Failed to serialize value to JSON");
		Value::Null
	})
}

impl ValueExtension for TransactionAttribute {
	fn to_value(&self) -> Value {
		serialize_to_value(self, "TransactionAttribute")
	}
}

impl ValueExtension for TransactionSendToken {
	fn to_value(&self) -> Value {
		serialize_to_value(self, "TransactionSendToken")
	}
}

impl VecValueExtension for Vec<TransactionSendToken> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

impl VecValueExtension for Vec<TransactionAttribute> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}
impl ValueExtension for Signer {
	fn to_value(&self) -> Value {
		serialize_to_value(self, "Signer")
	}
}

impl VecValueExtension for Vec<Signer> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

impl ValueExtension for TransactionSigner {
	fn to_value(&self) -> Value {
		serialize_to_value(self, "TransactionSigner")
	}
}

impl VecValueExtension for Vec<TransactionSigner> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{builder::WitnessScope, neo_crypto::KeyPair};
	use primitive_types::{H160, H256};

	#[test]
	fn test_try_public_keys_to_scripthash_rejects_zero_threshold() {
		let key_pair = KeyPair::new_random();
		let mut public_keys = vec![key_pair.public_key().clone()];

		let result = try_public_keys_to_scripthash(&mut public_keys, 0);
		assert!(result.is_err());
	}

	#[test]
	fn test_try_public_keys_to_scripthash_rejects_threshold_above_len() {
		let key_pair = KeyPair::new_random();
		let mut public_keys = vec![key_pair.public_key().clone()];

		let result = try_public_keys_to_scripthash(&mut public_keys, 2);
		assert!(result.is_err());
	}

	#[test]
	fn test_transaction_attribute_to_value_is_structured_json() {
		let attribute = TransactionAttribute::Conflicts { hash: H256::zero() };

		assert_eq!(attribute.to_value(), serde_json::to_value(&attribute).unwrap());
	}

	#[test]
	fn test_transaction_send_token_to_value_is_structured_json() {
		let token = TransactionSendToken::new(H160::zero(), 42, "NdzTestAddress".to_string());

		assert_eq!(token.to_value(), serde_json::to_value(&token).unwrap());
	}

	#[test]
	fn test_signer_to_value_is_structured_json() {
		let signer = Signer::TransactionSigner(
			TransactionSigner::new(H160::zero(), vec![WitnessScope::CalledByEntry]).unwrap(),
		);

		assert_eq!(signer.to_value(), serde_json::to_value(&signer).unwrap());
	}
}
