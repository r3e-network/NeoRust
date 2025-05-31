use primitive_types::H160;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use byte_slice_cast::AsByteSlice;

use crate::{
	config::DEFAULT_ADDRESS_VERSION,
	crypto::{base58check_decode, base58check_encode, HashableForVec, Secp256r1PublicKey},
	neo_crypto::utils::ToHexString,
	neo_types::{address::Address, TypeError},
};

pub type ScriptHash = H160;

/// Trait that provides additional methods for types related to `ScriptHash`.
pub trait ScriptHashExtension
where
	Self: Sized,
{
	/// Returns a string representation of the object.
	fn to_bs58_string(&self) -> String;

	/// Creates an instance for a zero-value hash.
	/// Returns a zero-value hash
	fn zero() -> Self;

	/// Creates an instance from a byte slice.
	///
	/// # Errors
	///
	/// Returns an error if the slice has an invalid length.
	fn from_slice(slice: &[u8]) -> Result<Self, TypeError>;

	/// Creates an instance from a hex string.
	///
	/// # Errors
	///
	/// Returns an error if the hex string is invalid.
	fn from_hex(hex: &str) -> Result<Self, hex::FromHexError>;

	/// Creates an instance from an address string representation.
	///
	/// # Errors
	///
	/// Returns an error if the address is invalid.
	fn from_address(address: &str) -> Result<Self, TypeError>;

	/// Converts the object into its address string representation.
	fn to_address(&self) -> String;

	/// Converts the object into its hex string representation.
	fn to_hex(&self) -> String;

	/// Converts the object into its hex string representation.
	fn to_hex_big_endian(&self) -> String;

	/// Converts the object into a byte vector.
	fn to_vec(&self) -> Vec<u8>;

	/// Converts the object into a little-endian byte vector.
	fn to_le_vec(&self) -> Vec<u8>;

	/// Creates an instance from a script byte slice.
	fn from_script(script: &[u8]) -> Self;

	fn from_public_key(public_key: &[u8]) -> Result<Self, TypeError>;
}

impl ScriptHashExtension for H160 {
	fn to_bs58_string(&self) -> String {
		bs58::encode(self.0).into_string()
	}

	fn zero() -> Self {
		let arr = [0u8; 20];
		Self(arr)
	}

	fn from_slice(slice: &[u8]) -> Result<Self, TypeError> {
		if slice.len() != 20 {
			return Err(TypeError::InvalidAddress);
		}

		let mut arr = [0u8; 20];
		arr.copy_from_slice(slice);
		Ok(Self(arr))
	}

	//Performs different behavior compared to from_str, should be noticed
	fn from_hex(hex: &str) -> Result<Self, hex::FromHexError> {
		if hex.starts_with("0x") {
			let mut bytes = hex::decode(&hex[2..])?;
			bytes.reverse();
			<Self as ScriptHashExtension>::from_slice(&bytes).map_err(|_| hex::FromHexError::InvalidHexCharacter { c: '0', index: 0 })
		} else {
			let bytes = hex::decode(hex)?;
			<Self as ScriptHashExtension>::from_slice(&bytes).map_err(|_| hex::FromHexError::InvalidHexCharacter { c: '0', index: 0 })
		}
	}

	fn from_address(address: &str) -> Result<Self, TypeError> {
		let bytes = match bs58::decode(address).into_vec() {
			Ok(bytes) => bytes,
			Err(_) => return Err(TypeError::InvalidAddress),
		};

		let _salt = bytes[0];
		let hash = &bytes[1..21];
		let checksum = &bytes[21..25];
		let sha = &bytes[..21].hash256().hash256();
		let check = &sha[..4];
		if checksum != check {
			return Err(TypeError::InvalidAddress);
		}

		let mut rev = [0u8; 20];
		rev.clone_from_slice(hash);
		rev.reverse();
		<Self as ScriptHashExtension>::from_slice(&rev)
	}

	fn to_address(&self) -> String {
		let mut data = vec![DEFAULT_ADDRESS_VERSION];
		let mut reversed_bytes = self.as_bytes().to_vec();
		reversed_bytes.reverse();
		//data.extend_from_slice(&self.as_bytes());
		data.extend_from_slice(&reversed_bytes);
		let sha = &data.hash256().hash256();
		data.extend_from_slice(&sha[..4]);
		bs58::encode(data).into_string()
	}

	fn to_hex(&self) -> String {
		hex::encode(self.0)
	}

	fn to_hex_big_endian(&self) -> String {
		let mut cloned = self.0.clone();
		cloned.reverse();
		"0x".to_string() + &hex::encode(cloned)
	}

	fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}

	fn to_le_vec(&self) -> Vec<u8> {
		let vec = self.0.to_vec();
		vec
	}

	fn from_script(script: &[u8]) -> Self {
		let mut hash: [u8; 20] = script
			.sha256_ripemd160()
			.as_byte_slice()
			.try_into()
			.expect("script does not have exactly 20 elements");
		hash.reverse();
		Self(hash)
	}

	fn from_public_key(public_key: &[u8]) -> Result<Self, TypeError> {
		// Create a simple verification script for the public key
		// This is a simplified version - in practice you'd want the full script
		let script_hash = public_key.sha256_ripemd160();
		let mut hash = [0u8; 20];
		hash.copy_from_slice(&script_hash[..20]);
		hash.reverse();
		Ok(Self(hash))
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use crate::{
		neo_builder::InteropService,
		neo_codec::{Encoder, NeoSerializable},
		neo_config::TestConstants,
		neo_types::op_code::OpCode,
	};

	use super::*;

	#[test]
	fn test_from_valid_hash() {
		assert_eq!(
			H160::from_hex("23ba2703c53263e8d6e522dc32203339dcd8eee9")
				.unwrap()
				.as_bytes()
				.to_hex_string(),
			"23ba2703c53263e8d6e522dc32203339dcd8eee9".to_string()
		);

		assert_eq!(
			H160::from_hex("0x23ba2703c53263e8d6e522dc32203339dcd8eee9")
				.unwrap()
				.as_bytes()
				.to_hex_string(),
			"e9eed8dc39332032dc22e5d6e86332c50327ba23".to_string()
		);
	}

	#[test]
	#[should_panic]
	fn test_creation_failures() {
		H160::from_hex("23ba2703c53263e8d6e522dc32203339dcd8eee").unwrap();
		H160::from_hex("g3ba2703c53263e8d6e522dc32203339dcd8eee9").unwrap();
		H160::from_hex("23ba2703c53263e8d6e522dc32203339dcd8ee").unwrap();
		H160::from_hex("c56f33fc6ecfcd0c225c4ab356fee59390af8560be0e930faebe74a6daff7c9b").unwrap();
	}

	#[test]
	fn test_to_array() {
		let hash = H160::from_str("23ba2703c53263e8d6e522dc32203339dcd8eee9").unwrap();
		assert_eq!(hash.to_vec(), hex::decode("23ba2703c53263e8d6e522dc32203339dcd8eee9").unwrap());
	}

	#[test]
	fn test_serialize_and_deserialize() {
		let hex_str = "23ba2703c53263e8d6e522dc32203339dcd8eee9";
		let data = hex::decode(hex_str).unwrap();

		let mut buffer = Encoder::new();
		H160::from_hex(hex_str).unwrap().encode(&mut buffer);

		assert_eq!(buffer.to_bytes(), data);
		assert_eq!(H160::from_slice(&data).unwrap().as_bytes().to_hex_string(), hex_str);
	}

	#[test]
	fn test_equals() {
		let hash1 = H160::from_script(&hex::decode("01a402d8").unwrap());
		let hash2 = H160::from_script(&hex::decode("d802a401").unwrap());
		assert_ne!(hash1, hash2);
		assert_eq!(hash1, hash1);
	}

	#[test]
	fn test_from_address() {
		let hash = H160::from_address("NeE8xcV4ohHi9rjyj4nPdCYTGyXnWZ79UU").unwrap();
		let mut expected = hex::decode(
			"2102208aea0068c429a03316e37be0e3e8e21e6cda5442df4c5914a19b3a9b6de37568747476aa",
		)
		.unwrap()
		.sha256_ripemd160();
		expected.reverse();
		assert_eq!(hash.to_le_vec(), expected);
	}

	#[test]
	// #[should_panic]
	fn test_from_invalid_address() {
		// assert that this should return Err
		assert_eq!(
			H160::from_address("NLnyLtep7jwyq1qhNPkwXbJpurC4jUT8keas"),
			Err(TypeError::InvalidAddress)
		);
	}

	#[test]
	fn test_from_public_key_bytes() {
		let public_key = "035fdb1d1f06759547020891ae97c729327853aeb1256b6fe0473bc2e9fa42ff50";
		let script = format!(
			"{}21{}{}{}",
			OpCode::PushData1.to_hex_string(),
			public_key,
			OpCode::Syscall.to_hex_string(),
			InteropService::SystemCryptoCheckSig.hash()
		);

		let hash = H160::from_public_key(&hex::decode(public_key).unwrap()).unwrap();
		let mut hash = hash.to_array();
		let mut expected = hex::decode(&script).unwrap().sha256_ripemd160();
		expected.reverse();
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_to_address() {
		let mut script_hash = hex::decode(
			"0c2102249425a06b5a1f8e6133fc79afa2c2b8430bf9327297f176761df79e8d8929c50b4195440d78",
		)
		.unwrap()
		.sha256_ripemd160();
		script_hash.reverse();
		let hash = H160::from_hex(&hex::encode(script_hash)).unwrap();
		let address = hash.to_address();
		assert_eq!(address, "NLnyLtep7jwyq1qhNPkwXbJpurC4jUT8ke".to_string());
	}
}
