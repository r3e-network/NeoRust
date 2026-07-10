use base64::Engine;
use bs58;
use hex;
use sha2::{Digest, Sha256};

use crate::{
	config::DEFAULT_ADDRESS_VERSION, neo_crypto::try_base58check_decode, neo_types::TypeError,
};
use neo3::prelude::ScriptHash;

pub trait TryStringExt {
	fn try_base58_decoded(&self) -> Result<Vec<u8>, TypeError>;

	fn try_base58_check_decoded(&self) -> Result<Vec<u8>, TypeError>;

	fn try_address_to_scripthash(&self) -> Result<ScriptHash, TypeError>;
}

pub trait StringExt {
	fn bytes_from_hex(&self) -> Result<Vec<u8>, hex::FromHexError>;

	fn base64_decoded(&self) -> Result<Vec<u8>, base64::DecodeError>;

	fn base64_encoded(&self) -> String;

	fn base58_decoded(&self) -> Option<Vec<u8>>;

	fn base58_check_decoded(&self) -> Option<Vec<u8>>;

	fn base58_encoded(&self) -> String;

	fn var_size(&self) -> usize;

	fn is_valid_address(&self) -> bool;

	fn is_valid_hex(&self) -> bool;

	fn address_to_scripthash(&self) -> Result<ScriptHash, &'static str>;

	fn try_reversed_hex(&self) -> Result<String, hex::FromHexError>;

	fn reversed_hex(&self) -> String;
}

impl TryStringExt for str {
	fn try_base58_decoded(&self) -> Result<Vec<u8>, TypeError> {
		bs58::decode(self)
			.into_vec()
			.map_err(|err| TypeError::InvalidFormat(format!("invalid base58 string: {err}")))
	}

	fn try_base58_check_decoded(&self) -> Result<Vec<u8>, TypeError> {
		try_base58check_decode(self)
			.map_err(|err| TypeError::InvalidFormat(format!("invalid base58check string: {err}")))
	}

	fn try_address_to_scripthash(&self) -> Result<ScriptHash, TypeError> {
		let data = self.try_base58_decoded().map_err(|_| TypeError::InvalidAddress)?;
		if data.len() != 25 || data[0] != DEFAULT_ADDRESS_VERSION {
			return Err(TypeError::InvalidAddress);
		}

		let checksum = &Sha256::digest(Sha256::digest(&data[..21]))[..4];
		if checksum != &data[21..] {
			return Err(TypeError::InvalidAddress);
		}

		let mut scripthash = data[1..21].to_vec();
		scripthash.reverse();
		Ok(ScriptHash::from_slice(&scripthash))
	}
}

impl TryStringExt for String {
	fn try_base58_decoded(&self) -> Result<Vec<u8>, TypeError> {
		self.as_str().try_base58_decoded()
	}

	fn try_base58_check_decoded(&self) -> Result<Vec<u8>, TypeError> {
		self.as_str().try_base58_check_decoded()
	}

	fn try_address_to_scripthash(&self) -> Result<ScriptHash, TypeError> {
		self.as_str().try_address_to_scripthash()
	}
}

impl StringExt for String {
	fn bytes_from_hex(&self) -> Result<Vec<u8>, hex::FromHexError> {
		hex::decode(self.trim_start_matches("0x"))
	}

	fn base64_decoded(&self) -> Result<Vec<u8>, base64::DecodeError> {
		base64::engine::general_purpose::STANDARD.decode(self)
	}

	fn base64_encoded(&self) -> String {
		base64::engine::general_purpose::STANDARD.encode(self.as_bytes())
	}

	fn base58_decoded(&self) -> Option<Vec<u8>> {
		self.try_base58_decoded().ok()
	}

	fn base58_check_decoded(&self) -> Option<Vec<u8>> {
		self.try_base58_check_decoded().ok()
	}

	fn base58_encoded(&self) -> String {
		bs58::encode(self.as_bytes()).into_string()
	}

	fn var_size(&self) -> usize {
		let bytes = self.as_bytes();
		let len = bytes.len();
		if len < 0xFD {
			1
		} else if len <= 0xFFFF {
			3
		} else if len <= 0xFFFFFFFF {
			5
		} else {
			9
		}
	}

	fn is_valid_address(&self) -> bool {
		self.try_address_to_scripthash().is_ok()
	}

	fn is_valid_hex(&self) -> bool {
		self.len().is_multiple_of(2) && self.chars().all(|c| c.is_ascii_hexdigit())
	}

	fn address_to_scripthash(&self) -> Result<ScriptHash, &'static str> {
		self.try_address_to_scripthash().map_err(|_| "Not a valid address")
	}

	fn try_reversed_hex(&self) -> Result<String, hex::FromHexError> {
		let mut bytes = self.bytes_from_hex()?;
		bytes.reverse();
		Ok(hex::encode(bytes))
	}

	fn reversed_hex(&self) -> String {
		self.try_reversed_hex().unwrap_or_else(|e| {
			panic!("invalid hex string; use try_reversed_hex for fallible handling: {}", e)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::{StringExt, TryStringExt};

	#[test]
	fn test_base58_check_decoded_validates_checksum() {
		let input = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtRw".to_string();
		let expected = vec![
			6, 161, 159, 136, 34, 110, 33, 238, 14, 79, 14, 218, 133, 13, 109, 40, 194, 236, 153,
			44, 61, 157, 254,
		];

		assert_eq!(input.base58_check_decoded(), Some(expected));
	}

	#[test]
	fn test_base58_check_decoded_rejects_invalid_checksum() {
		let input = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtrW".to_string();
		assert!(input.base58_check_decoded().is_none());
	}

	#[test]
	fn test_try_base58_decoded_rejects_invalid_input() {
		let input = "0oO1lL".to_string();
		assert!(matches!(
			input.try_base58_decoded(),
			Err(crate::neo_types::TypeError::InvalidFormat(message)) if message.contains("base58")
		));
	}

	#[test]
	fn test_try_base58_check_decoded_rejects_invalid_checksum() {
		let input = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtrW".to_string();
		assert!(matches!(
			input.try_base58_check_decoded(),
			Err(crate::neo_types::TypeError::InvalidFormat(message)) if message.contains("base58check")
		));
	}

	#[test]
	fn test_try_address_to_scripthash_returns_known_hash() {
		let address = "NTGYC16CN5QheM4ZwfhUp9JKq8bMjWtcAp".to_string();
		assert_eq!(
			hex::encode(address.try_address_to_scripthash().unwrap().as_bytes()),
			"87c06be672d5600dce4a260e7b2d497112c0ac50"
		);
	}

	#[test]
	fn test_try_reversed_hex_rejects_invalid_hex() {
		let input = "xyz".to_string();
		assert!(input.try_reversed_hex().is_err());
	}

	#[test]
	fn test_try_reversed_hex_reverses_valid_hex() {
		let input = "0a0b0c".to_string();
		assert_eq!(input.try_reversed_hex().unwrap(), "0c0b0a");
	}

	#[test]
	#[should_panic(expected = "invalid hex string; use try_reversed_hex")]
	fn test_reversed_hex_panics_on_invalid_hex() {
		let _ = "xyz".to_string().reversed_hex();
	}
}
