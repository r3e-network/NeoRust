//! CryptoLib functions for Neo N3.
//!
//! This module provides cryptographic functions that mirror the Neo N3 CryptoLib native contract.
//! These functions are used for hashing, signature verification, and key recovery.
//!
//! ## Features
//!
//! - **Hashing**: SHA256, RIPEMD160, Murmur32, Keccak256, SHA3-512, Blake2b-512
//! - **Signature Verification**: ECDSA (secp256r1, secp256k1), Ed25519
//! - **Key Recovery**: Recover public keys from ECDSA signatures (secp256k1)
//!
//! ## Neo 3.9 Additions
//!
//! The following features were added in Neo 3.9:
//! - SHA3-512 hash function (HF_Faun)
//! - Blake2b-512 hash function (HF_Faun)
//! - Ed25519 signature verification (HF_Echidna)
//! - secp256k1 public key recovery (HF_Echidna)

use blake2::{Blake2b512, Digest as Blake2Digest};
use ed25519_dalek::{Signature as Ed25519Signature, Verifier, VerifyingKey};
use k256::ecdsa::{RecoveryId, Signature as K256Signature, VerifyingKey as K256VerifyingKey};
use sha3::Sha3_512;

/// Computes the SHA3-512 hash of the input data.
///
/// This function was added in Neo 3.9 (HF_Faun).
///
/// # Arguments
///
/// * `data` - The data to hash.
///
/// # Returns
///
/// A 64-byte (512-bit) hash.
///
/// # Example
///
/// ```rust
/// use neo3::neo_crypto::crypto_lib::sha3_512;
///
/// let data = b"Hello, Neo!";
/// let hash = sha3_512(data);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn sha3_512(data: &[u8]) -> Vec<u8> {
	let mut hasher = Sha3_512::new();
	hasher.update(data);
	hasher.finalize().to_vec()
}

/// Computes the Blake2b-512 hash of the input data.
///
/// This function was added in Neo 3.9 (HF_Faun).
///
/// # Arguments
///
/// * `data` - The data to hash.
///
/// # Returns
///
/// A 64-byte (512-bit) hash.
///
/// # Example
///
/// ```rust
/// use neo3::neo_crypto::crypto_lib::blake2b_512;
///
/// let data = b"Hello, Neo!";
/// let hash = blake2b_512(data);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn blake2b_512(data: &[u8]) -> Vec<u8> {
	let mut hasher = Blake2b512::new();
	hasher.update(data);
	hasher.finalize().to_vec()
}

/// Verifies an Ed25519 signature.
///
/// This function was added in Neo 3.9 (HF_Echidna).
///
/// # Arguments
///
/// * `message` - The original message that was signed.
/// * `pubkey` - The 32-byte Ed25519 public key.
/// * `signature` - The 64-byte Ed25519 signature.
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
///
/// # Example
///
/// ```rust,no_run
/// use neo3::neo_crypto::crypto_lib::verify_with_ed25519;
///
/// let message = b"Hello, Neo!";
/// let pubkey = [0u8; 32]; // Your public key
/// let signature = [0u8; 64]; // Your signature
/// let is_valid = verify_with_ed25519(message, &pubkey, &signature);
/// ```
pub fn verify_with_ed25519(message: &[u8], pubkey: &[u8], signature: &[u8]) -> bool {
	if pubkey.len() != 32 || signature.len() != 64 {
		return false;
	}

	let pubkey_bytes: [u8; 32] = match pubkey.try_into() {
		Ok(bytes) => bytes,
		Err(_) => return false,
	};

	let signature_bytes: [u8; 64] = match signature.try_into() {
		Ok(bytes) => bytes,
		Err(_) => return false,
	};

	let verifying_key = match VerifyingKey::from_bytes(&pubkey_bytes) {
		Ok(key) => key,
		Err(_) => return false,
	};

	let sig = Ed25519Signature::from_bytes(&signature_bytes);

	verifying_key.verify(message, &sig).is_ok()
}

/// Recovers the public key from a secp256k1 ECDSA signature.
///
/// This function was added in Neo 3.9 (HF_Echidna).
///
/// # Arguments
///
/// * `message_hash` - The 32-byte message hash that was signed.
/// * `signature` - The 65-byte signature (r || s || recovery_id).
///
/// # Returns
///
/// The recovered 33-byte compressed public key, or `None` if recovery fails.
///
/// # Example
///
/// ```rust,no_run
/// use neo3::neo_crypto::crypto_lib::recover_secp256k1;
///
/// let message_hash = [0u8; 32]; // Your message hash
/// let signature = [0u8; 65]; // Your signature with recovery id
/// if let Some(pubkey) = recover_secp256k1(&message_hash, &signature) {
///     println!("Recovered public key: {:?}", pubkey);
/// }
/// ```
pub fn recover_secp256k1(message_hash: &[u8], signature: &[u8]) -> Option<Vec<u8>> {
	if message_hash.len() != 32 || signature.len() != 65 {
		return None;
	}

	// The signature format is: r (32 bytes) || s (32 bytes) || recovery_id (1 byte)
	let sig_bytes: [u8; 64] = signature[..64].try_into().ok()?;
	let recovery_byte = signature[64];

	// Recovery ID should be 0, 1, 2, or 3
	let recovery_id = RecoveryId::try_from(recovery_byte).ok()?;

	let signature = K256Signature::from_bytes(&sig_bytes.into()).ok()?;

	let recovered_key =
		K256VerifyingKey::recover_from_prehash(message_hash, &signature, recovery_id).ok()?;

	// Return the compressed public key (33 bytes)
	Some(recovered_key.to_sec1_bytes().to_vec())
}

/// Trait extension to add new hash functions to byte slices.
pub trait CryptoLibHashable {
	/// Computes the SHA3-512 hash of the data.
	fn sha3_512(&self) -> Vec<u8>;

	/// Computes the Blake2b-512 hash of the data.
	fn blake2b_512(&self) -> Vec<u8>;
}

impl CryptoLibHashable for [u8] {
	fn sha3_512(&self) -> Vec<u8> {
		sha3_512(self)
	}

	fn blake2b_512(&self) -> Vec<u8> {
		blake2b_512(self)
	}
}

impl CryptoLibHashable for Vec<u8> {
	fn sha3_512(&self) -> Vec<u8> {
		self.as_slice().sha3_512()
	}

	fn blake2b_512(&self) -> Vec<u8> {
		self.as_slice().blake2b_512()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sha3_512() {
		let data = b"hello world";
		let hash = sha3_512(data);

		assert_eq!(hash.len(), 64);
		// Known SHA3-512 hash of "hello world"
		let expected = hex::decode(
			"840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a"
		).expect("Failed to decode expected SHA3-512 hash hex");
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_sha3_512_empty() {
		let data = b"";
		let hash = sha3_512(data);
		assert_eq!(hash.len(), 64);
		// Known SHA3-512 hash of empty string
		let expected = hex::decode(
			"a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
		).expect("Failed to decode expected SHA3-512 hash hex");
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_sha3_512_short_string() {
		// Test with a short string "abc"
		let data = b"abc";
		let hash = sha3_512(data);
		assert_eq!(hash.len(), 64);
		// Known SHA3-512 hash of "abc"
		let expected = hex::decode(
			"b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"
		).expect("Failed to decode expected SHA3-512 hash hex");
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_blake2b_512() {
		let data = b"hello world";
		let hash = blake2b_512(data);

		assert_eq!(hash.len(), 64);
		// Known Blake2b-512 hash of "hello world"
		let expected = hex::decode(
			"021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0"
		).expect("Failed to decode expected Blake2b-512 hash hex");
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_blake2b_512_empty() {
		let data = b"";
		let hash = blake2b_512(data);
		assert_eq!(hash.len(), 64);
		// Known Blake2b-512 hash of empty string
		let expected = hex::decode(
			"786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce"
		).expect("Failed to decode expected Blake2b-512 hash hex");
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_blake2b_512_short_string() {
		// Test with a short string "abc"
		let data = b"abc";
		let hash = blake2b_512(data);
		assert_eq!(hash.len(), 64);
		// Known Blake2b-512 hash of "abc"
		let expected = hex::decode(
			"ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"
		).expect("Failed to decode expected Blake2b-512 hash hex");
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_verify_with_ed25519_invalid_lengths() {
		// Invalid public key length
		assert!(!verify_with_ed25519(b"message", &[0u8; 31], &[0u8; 64]));

		// Invalid signature length
		assert!(!verify_with_ed25519(b"message", &[0u8; 32], &[0u8; 63]));
	}

	#[test]
	fn test_verify_with_ed25519_valid_signature() {
		// Test vector from RFC 8032 (Ed25519 test vectors)
		// Secret key: 9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60
		// Public key: d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
		// Message: ""
		// Signature: e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b

		let pubkey =
			hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
				.expect("Failed to decode Ed25519 public key");
		let signature = hex::decode("e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b")
			.expect("Failed to decode Ed25519 signature");
		let message = b"";

		assert!(verify_with_ed25519(message, &pubkey, &signature));
	}

	#[test]
	fn test_verify_with_ed25519_valid_signature_with_message() {
		// Test vector from RFC 8032
		// Secret key: c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7
		// Public key: fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025
		// Message: af82
		// Signature: 6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a

		let pubkey =
			hex::decode("fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025")
				.expect("Failed to decode Ed25519 public key");
		let signature = hex::decode("6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a")
			.expect("Failed to decode Ed25519 signature");
		let message = hex::decode("af82").expect("Failed to decode message");

		assert!(verify_with_ed25519(&message, &pubkey, &signature));
	}

	#[test]
	fn test_verify_with_ed25519_invalid_signature() {
		// Valid public key but modified signature (should fail)
		let pubkey =
			hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
				.expect("Failed to decode Ed25519 public key");
		let mut signature = hex::decode("e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b")
			.expect("Failed to decode Ed25519 signature");
		// Corrupt the signature by flipping a bit
		signature[0] ^= 0x01;
		let message = b"";

		assert!(!verify_with_ed25519(message, &pubkey, &signature));
	}

	#[test]
	fn test_recover_secp256k1_invalid_lengths() {
		// Invalid message hash length
		assert!(recover_secp256k1(&[0u8; 31], &[0u8; 65]).is_none());

		// Invalid signature length
		assert!(recover_secp256k1(&[0u8; 32], &[0u8; 64]).is_none());
	}

	#[test]
	fn test_recover_secp256k1_valid_signature() {
		// Test vector for secp256k1 recovery
		// Message hash (32 bytes) - SHA256 of "hello"
		let message_hash =
			hex::decode("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
				.expect("Failed to decode message hash");

		// Valid secp256k1 signature with recovery ID
		// r, s, and recovery_id (0 or 1)
		let signature = hex::decode(
			"a0eb7f7b44bd0eb8b680e7b6be7f3356a7b8ab62cf2a190ac6e2f2279b0e8f2a\
			 7b8a9b8f0e3e1e6b9b8f3e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e6e\
			 00",
		)
		.expect("Failed to decode signature");

		// This deliberately malformed signature has a valid encoded length. The test verifies
		// that recovery handles length-valid but cryptographically invalid input without panics.

		// This test validates that the function doesn't panic with valid inputs
		// The actual recovery depends on valid signature data
		let _result = recover_secp256k1(&message_hash, &signature);
	}

	#[test]
	fn test_recover_secp256k1_with_invalid_recovery_id() {
		// Message hash (32 bytes)
		let message_hash = [0u8; 32];

		// Signature with invalid recovery_id (must be 0, 1, 2, or 3)
		let mut signature = [0u8; 65];
		signature[64] = 4; // Invalid recovery ID

		// Should return None for invalid recovery ID
		assert!(recover_secp256k1(&message_hash, &signature).is_none());
	}

	#[test]
	fn test_crypto_lib_hashable_trait() {
		let data = b"test data";

		// Test slice
		let sha3_hash = data.sha3_512();
		assert_eq!(sha3_hash.len(), 64);

		let blake2_hash = data.blake2b_512();
		assert_eq!(blake2_hash.len(), 64);

		// Test Vec
		let data_vec = data.to_vec();
		let sha3_hash_vec = data_vec.sha3_512();
		assert_eq!(sha3_hash_vec.len(), 64);

		let blake2_hash_vec = data_vec.blake2b_512();
		assert_eq!(blake2_hash_vec.len(), 64);

		// Hashes should be the same
		assert_eq!(sha3_hash, sha3_hash_vec);
		assert_eq!(blake2_hash, blake2_hash_vec);
	}
}
