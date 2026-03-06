use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Base58CheckError {
	#[error("invalid base58 string: {0}")]
	InvalidBase58(String),
	#[error("base58check payload is missing checksum bytes")]
	MissingChecksum,
	#[error("base58check checksum mismatch")]
	InvalidChecksum,
}

/// Encodes a byte slice into a base58check string.
///
/// # Arguments
///
/// * `bytes` - A byte slice to be encoded.
///
/// # Example
///
/// ```
/// use neo3::neo_crypto::base58check_encode;
/// let bytes = [0x01, 0x02, 0x03];
/// let encoded = base58check_encode(&bytes);
/// ```
pub fn base58check_encode(bytes: &[u8]) -> String {
	let checksum = &calculate_checksum(bytes)[..4];
	let bytes_with_checksum = [bytes, checksum].concat();
	bs58::encode(bytes_with_checksum).into_string()
}

/// Decodes a base58check string into a byte vector.
///
/// # Arguments
///
/// * `input` - A base58check string to be decoded.
///
/// # Example
///
/// ```
/// use neo3::neo_crypto::base58check_decode;
/// let input = "Abc123";
/// let decoded = base58check_decode(input);
/// ```
pub fn try_base58check_decode(input: &str) -> Result<Vec<u8>, Base58CheckError> {
	let bytes_with_checksum = bs58::decode(input)
		.into_vec()
		.map_err(|err| Base58CheckError::InvalidBase58(err.to_string()))?;

	if bytes_with_checksum.len() < 4 {
		return Err(Base58CheckError::MissingChecksum);
	}

	let (bytes, checksum) = bytes_with_checksum.split_at(bytes_with_checksum.len() - 4);
	let expected_checksum = calculate_checksum(bytes);

	if checksum != &expected_checksum[..4] {
		return Err(Base58CheckError::InvalidChecksum);
	}

	Ok(bytes.to_vec())
}

pub fn base58check_decode(input: &str) -> Option<Vec<u8>> {
	try_base58check_decode(input).ok()
}

/// Calculates the checksum of a byte slice.
///
/// # Arguments
///
/// * `input` - A byte slice to calculate the checksum for.
///
/// # Example
///
/// ```
/// use neo3::neo_crypto::calculate_checksum;
/// let bytes = [0x01, 0x02, 0x03];
/// let checksum = calculate_checksum(&bytes);
/// ```
pub fn calculate_checksum(input: &[u8]) -> [u8; 4] {
	let mut hasher = Sha256::new();
	hasher.update(input);
	let hash = hasher.finalize();
	let hash256 = Sha256::digest(hash);
	let mut checksum = [0u8; 4];
	checksum.copy_from_slice(&hash256[..4]);
	checksum
}

#[cfg(test)]
mod base58_tests {
	use super::*;

	// Define tuples of arbitrary strings that are mapped to valid Base58 encodings
	static VALID_STRING_DECODED_TO_ENCODED: &[(&str, &str)] = &[
		("", ""),
		(" ", "Z"),
		("-", "n"),
		("0", "q"),
		("1", "r"),
		("-1", "4SU"),
		("11", "4k8"),
		("abc", "ZiCa"),
		("1234598760", "3mJr7AoUXx2Wqd"),
		("abcdefghijklmnopqrstuvwxyz", "3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f"),
		(
			"00000000000000000000000000000000000000000000000000000000000000",
			"3sN2THZeE9Eh9eYrwkvZqNstbHGvrxSAM7gXUXvyFQP8XvQLUqNCS27icwUeDT7ckHm4FUHM2mTVh1vbLmk7y",
		),
	];

	// Define invalid strings
	static INVALID_STRINGS: &[&str] =
		&["0", "O", "I", "l", "3mJr0", "O3yxU", "3sNI", "4kl8", "0OIl", "!@#$%^&*()-_=+~`"];

	#[test]
	fn test_base58_encoding_for_valid_strings() {
		for (decoded, encoded) in VALID_STRING_DECODED_TO_ENCODED {
			let bytes = decoded.as_bytes();
			let result = bs58::encode(bytes).into_string();
			assert_eq!(&result, *encoded);
		}
	}

	#[test]
	fn test_base58_decoding_for_valid_strings() {
		for (decoded, encoded) in VALID_STRING_DECODED_TO_ENCODED {
			let result = bs58::decode(encoded).into_vec().unwrap();
			assert_eq!(result, Vec::from(*decoded));
		}
	}

	#[test]
	fn test_base58_decoding_for_invalid_strings() {
		for invalid_string in INVALID_STRINGS {
			let result = base58check_decode(invalid_string);
			assert!(result.is_none());
		}
	}

	#[test]
	fn test_base58check_encoding() {
		let input_data: Vec<u8> = vec![
			6, 161, 159, 136, 34, 110, 33, 238, 14, 79, 14, 218, 133, 13, 109, 40, 194, 236, 153,
			44, 61, 157, 254,
		];
		let expected_output = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtRw";
		let actual_output = base58check_encode(&input_data);
		assert_eq!(actual_output, expected_output);
	}

	#[test]
	fn test_base58check_decoding() {
		let input_string = "tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtRw";
		let expected_output_data: Vec<u8> = vec![
			6, 161, 159, 136, 34, 110, 33, 238, 14, 79, 14, 218, 133, 13, 109, 40, 194, 236, 153,
			44, 61, 157, 254,
		];
		let actual_output = base58check_decode(input_string);
		assert_eq!(actual_output, Some(expected_output_data));
	}

	#[test]
	fn test_try_base58check_decode_reports_invalid_characters() {
		assert!(matches!(
			try_base58check_decode("0oO1lL"),
			Err(Base58CheckError::InvalidBase58(_))
		));
	}

	#[test]
	fn test_try_base58check_decode_reports_invalid_checksum() {
		assert!(matches!(
			try_base58check_decode("tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtrW"),
			Err(Base58CheckError::InvalidChecksum)
		));
	}

	#[test]
	fn test_try_base58check_decode_reports_missing_checksum_bytes() {
		assert!(matches!(try_base58check_decode("1"), Err(Base58CheckError::MissingChecksum)));
	}

	#[test]
	fn test_base58check_empty_roundtrip() {
		let encoded = base58check_encode(&[]);
		assert_eq!(base58check_decode(&encoded), Some(Vec::new()));
	}

	#[test]
	fn test_base58check_decoding_with_invalid_characters() {
		assert!(base58check_decode("0oO1lL").is_none());
	}

	#[test]
	fn test_base58check_decoding_with_invalid_checksum() {
		assert!(base58check_decode("tz1Y3qqTg9HdrzZGbEjiCPmwuZ7fWVxpPtrW").is_none());
	}
}
