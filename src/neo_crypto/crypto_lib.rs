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

	let recovered_key = K256VerifyingKey::recover_from_prehash(message_hash, &signature, recovery_id).ok()?;

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
		).unwrap();
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_sha3_512_empty() {
		let data = b"";
		let hash = sha3_512(data);
		assert_eq!(hash.len(), 64);
	}

	#[test]
	fn test_blake2b_512() {
		let data = b"hello world";
		let hash = blake2b_512(data);

		assert_eq!(hash.len(), 64);
		// Known Blake2b-512 hash of "hello world"
		let expected = hex::decode(
			"021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0"
		).unwrap();
		assert_eq!(hash, expected);
	}

	#[test]
	fn test_blake2b_512_empty() {
		let data = b"";
		let hash = blake2b_512(data);
		assert_eq!(hash.len(), 64);
	}

	#[test]
	fn test_verify_with_ed25519_invalid_lengths() {
		// Invalid public key length
		assert!(!verify_with_ed25519(b"message", &[0u8; 31], &[0u8; 64]));

		// Invalid signature length
		assert!(!verify_with_ed25519(b"message", &[0u8; 32], &[0u8; 63]));
	}

	#[test]
	fn test_recover_secp256k1_invalid_lengths() {
		// Invalid message hash length
		assert!(recover_secp256k1(&[0u8; 31], &[0u8; 65]).is_none());

		// Invalid signature length
		assert!(recover_secp256k1(&[0u8; 32], &[0u8; 64]).is_none());
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
