//! # KeyPair
//!
//! `KeyPair` is a module that provides an implementation for Elliptic Curve Key Pairs using the `p256` crate.
//!
//! This structure can be used to manage and manipulate EC key pairs,
//! including generating new pairs, importing them from raw bytes,
//! and converting them to various formats.
//!
//! ## Security
//!
//! This type implements `ZeroizeOnDrop` to ensure that the private key is securely
//! erased from memory when the key pair is dropped.

use rand_core::OsRng;

use crate::{
	builder::VerificationScript,
	crypto::{
		private_key_from_wif, wif_from_private_key, CryptoError, PublicKeyExtension,
		Secp256r1PrivateKey, Secp256r1PublicKey, Secp256r1Signature,
	},
	neo_types::{ScriptHash, ScriptHashExtension},
};
use p256::elliptic_curve::zeroize::{Zeroize, ZeroizeOnDrop};

/// Represents an Elliptic Curve Key Pair containing both a private and a public key.
#[derive(Debug, Clone)]
pub struct KeyPair {
	/// The private key component of the key pair.
	pub private_key: Secp256r1PrivateKey,

	/// The public key component of the key pair.
	pub public_key: Secp256r1PublicKey,

	/// Whether this key pair contains a real private key.
	/// `false` when created via `from_public_key()`, which uses a placeholder.
	pub has_private_key: bool,
}

impl KeyPair {
	/// Creates a new `KeyPair` instance given a private key and its corresponding public key.
	///
	/// # Arguments
	///
	/// * `private_key` - A `Secp256r1PrivateKey` representing the private key.
	/// * `public_key` - A `Secp256r1PublicKey` representing the public key.
	pub fn new(private_key: Secp256r1PrivateKey, public_key: Secp256r1PublicKey) -> Self {
		Self { private_key, public_key, has_private_key: true }
	}

	/// Returns a clone of the private key.
	///
	/// PERFORMANCE NOTE: Consider using `private_key_ref()` to avoid cloning
	/// when you only need to read the key.
	pub fn private_key(&self) -> Secp256r1PrivateKey {
		self.private_key.clone()
	}

	/// Returns a reference to the private key without cloning.
	///
	/// Use this method when you only need to read the private key
	/// to avoid unnecessary memory allocation.
	#[inline]
	pub fn private_key_ref(&self) -> &Secp256r1PrivateKey {
		&self.private_key
	}

	/// Returns a clone of the public key.
	///
	/// PERFORMANCE NOTE: Consider using `public_key_ref()` to avoid cloning
	/// when you only need to read the key.
	pub fn public_key(&self) -> Secp256r1PublicKey {
		self.public_key.clone()
	}

	/// Returns a reference to the public key without cloning.
	///
	/// Use this method when you only need to read the public key
	/// to avoid unnecessary memory allocation.
	#[inline]
	pub fn public_key_ref(&self) -> &Secp256r1PublicKey {
		&self.public_key
	}

	/// Derives a new `KeyPair` instance from just a private key.
	/// The public key is derived from the given private key.
	///
	/// # Arguments
	///
	/// * `private_key` - A `Secp256r1PrivateKey` representing the private key.
	pub fn from_secret_key(private_key: &Secp256r1PrivateKey) -> Self {
		let public_key = private_key.clone().to_public_key();
		Self::new(private_key.clone(), public_key)
	}

	/// Returns the 32-byte representation of the private key.
	pub fn private_key_bytes(&self) -> [u8; 32] {
		self.private_key.to_raw_bytes()
	}

	/// Returns the 64-byte uncompressed representation of the public key.
	pub fn public_key_bytes(&self) -> [u8; 64] {
		let mut buf = [0u8; 64];
		let uncompressed = self.public_key.get_encoded(false);
		debug_assert_eq!(
			uncompressed.len(),
			65,
			"Expected an uncompressed secp256r1 public key to be 65 bytes (0x04 || X || Y)"
		);
		if uncompressed.len() == 65 {
			buf.copy_from_slice(&uncompressed[1..]);
		} else if uncompressed.len() == 64 {
			buf.copy_from_slice(&uncompressed);
		}

		buf
	}

	/// Signs a message using this key pair's private key.
	///
	/// Returns an error if this key pair was created via `from_public_key()`
	/// and does not contain a real private key.
	pub fn sign(&self, message: &[u8]) -> Result<Secp256r1Signature, CryptoError> {
		if !self.has_private_key {
			return Err(CryptoError::SigningError);
		}
		self.private_key.sign_tx(message)
	}

	/// Verifies a signature against a message using this key pair's public key.
	pub fn verify(
		&self,
		message: &[u8],
		signature: &Secp256r1Signature,
	) -> Result<bool, CryptoError> {
		self.public_key.verify(message, signature).map(|_| true)
	}
}

impl KeyPair {
	/// Generates a new random `KeyPair`.
	pub fn new_random() -> Self {
		let mut rng = OsRng; // A cryptographically secure random number generator
		let secret_key = Secp256r1PrivateKey::random(&mut rng);
		Self::from_secret_key(&secret_key)
	}

	/// Creates an `KeyPair` from a given 32-byte private key.
	///
	/// # Arguments
	///
	/// * `private_key` - A 32-byte slice representing the private key.
	pub fn from_private_key(private_key: &[u8; 32]) -> Result<Self, CryptoError> {
		let secret_key = Secp256r1PrivateKey::from_bytes(private_key)?;
		Ok(Self::from_secret_key(&secret_key))
	}

	/// Creates an `KeyPair` from a given Wallet Import Format (WIF) string.
	/// This will use the private key encoded in the WIF to generate the key pair.
	///
	///  # Arguments
	///
	/// * `wif` - A Wallet Import Format (WIF) string.
	///
	/// The WIF string should be in the format `Kx...` or `Lx...`.
	/// The key pair will be generated from the private key encoded in the WIF.
	/// The public key will be derived from the private key.
	pub fn from_wif(wif: &str) -> Result<Self, CryptoError> {
		let private_key = private_key_from_wif(wif)?;
		Ok(Self::from_secret_key(&private_key))
	}

	/// WARNING: Creates a KeyPair with a PLACEHOLDER private key.
	/// This KeyPair CANNOT be used for signing. It exists only for
	/// public-key-only operations like address derivation.
	///
	/// # Arguments
	///
	/// * `public_key` - The 64-byte uncompressed public key (x || y, without the 0x04 prefix).
	pub fn from_public_key(public_key: &[u8; 64]) -> Result<Self, CryptoError> {
		let public_key = Secp256r1PublicKey::from_slice(public_key)?;
		let mut placeholder_private_key = [0u8; 32];
		placeholder_private_key[31] = 1; // secp256r1 private keys must be in the range [1, n-1]
		let secret_key = Secp256r1PrivateKey::from_bytes(&placeholder_private_key)?;
		Ok(Self { private_key: secret_key, public_key, has_private_key: false })
	}

	/// Exports the key pair as a Wallet Import Format (WIF) string
	///
	/// Returns: The WIF encoding of this key pair
	pub fn export_as_wif(&self) -> String {
		wif_from_private_key(&self.private_key())
	}

	pub fn get_script_hash(&self) -> ScriptHash {
		let vs = VerificationScript::from_public_key(&self.public_key());
		vs.hash()
	}

	pub fn get_address(&self) -> String {
		self.get_script_hash().to_address()
	}
}

impl PartialEq for KeyPair {
	fn eq(&self, other: &Self) -> bool {
		self.private_key == other.private_key
			&& self.public_key == other.public_key
			&& self.has_private_key == other.has_private_key
	}
}

// Implement Zeroize for manual zeroization if needed
impl Zeroize for KeyPair {
	fn zeroize(&mut self) {
		self.private_key.zeroize();
		// Note: public_key is not sensitive, so we don't zeroize it
	}
}

// Automatically zeroize private key when KeyPair is dropped
impl ZeroizeOnDrop for KeyPair {}

#[cfg(test)]
mod tests {
	use crate::{config::TestConstants, crypto::KeyPair, ScriptHash, ScriptHashExtension};
	use hex;

	#[test]
	fn test_public_key_wif() {
		let private_key =
			hex::decode("c7134d6fd8e73d819e82755c64c93788d8db0961929e025a53363c4cc02a6962")
				.unwrap();
		let private_key_arr: &[u8; 32] = private_key.as_slice().try_into().unwrap();
		let key_pair = KeyPair::from_private_key(private_key_arr).unwrap();
		assert_eq!(
			key_pair.export_as_wif(),
			"L3tgppXLgdaeqSGSFw1Go3skBiy8vQAM7YMXvTHsKQtE16PBncSU"
		);
	}

	#[test]
	fn test_address() {
		let private_key = hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY).unwrap();
		let private_key_arr: &[u8; 32] = private_key.as_slice().try_into().unwrap();
		let key_pair = KeyPair::from_private_key(private_key_arr).unwrap();
		assert_eq!(key_pair.get_address(), TestConstants::DEFAULT_ACCOUNT_ADDRESS);
	}

	#[test]
	fn test_script_hash() {
		let private_key = hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY).unwrap();
		let private_key_arr: &[u8; 32] = private_key.as_slice().try_into().unwrap();
		let key_pair = KeyPair::from_private_key(private_key_arr).unwrap();
		assert_eq!(
			key_pair.get_script_hash(),
			ScriptHash::from_hex(TestConstants::DEFAULT_ACCOUNT_SCRIPT_HASH).unwrap()
		);
	}

	#[test]
	fn test_public_key_bytes_uncompressed_xy() {
		let private_key = hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY).unwrap();
		let private_key_arr: &[u8; 32] = private_key.as_slice().try_into().unwrap();
		let key_pair = KeyPair::from_private_key(private_key_arr).unwrap();

		let bytes = key_pair.public_key_bytes();
		let uncompressed = key_pair.public_key().get_encoded(false);
		assert_eq!(uncompressed.len(), 65);
		assert_eq!(&bytes[..], &uncompressed[1..]);
	}

	// #[test]
	// pub fn setup_new_ec_public_key_and_get_encoded_and_get_ec_point() {
	//     let expected_x = hex!("b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816");
	//     let expected_y = hex!("5f4f7fb1c5862465543c06dd5a2aa414f6583f92a5cc3e1d4259df79bf6839c9");

	//     let expected_ec_point = EncodedPoint::from_affine_coordinates(
	//         &expected_x.into(),
	//         &expected_y.into(),
	//         false
	//     );

	//     let enc_ec_point = "03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816";
	//     let enc_ec_point_bytes = hex::decode(enc_ec_point).unwrap();

	//     let pub_key = Secp256r1PublicKey::from_encoded(&enc_ec_point).unwrap();

	//     assert_eq!(pub_key.get_encoded_point(false), expected_ec_point);
	//     assert_eq!(pub_key.get_encoded(true), enc_ec_point_bytes);
	//     assert_eq!(pub_key.get_encoded_compressed_hex(), enc_ec_point);
	// }

	// #[test]
	// pub fn create_ec_public_key_from_uncompressed_ec_point() {
	//     let ec_point = "04b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e1368165f4f7fb1c5862465543c06dd5a2aa414f6583f92a5cc3e1d4259df79bf6839c9";

	//     let pub_key = Secp256r1PublicKey::from_encoded(&ec_point).unwrap();

	//     assert_eq!(
	//         pub_key.get_encoded_compressed_hex(),
	//         "03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816"
	//     );
	// }

	// #[test]
	// pub fn invalid_size() {
	// 	///
	// 	/// Need futher adjustments to deal with specifc error messages in PublicKey
	// 	///
	//     let pub_key_hex = "03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e1368"; //only 32 bits

	//     let pub_key = Secp256r1PublicKey::from_encoded(&pub_key_hex);

	//     assert_eq!(
	// 		pub_key,
	// 		None
	// 	);
	// }

	// #[test]
	// pub fn clean_hex_prefix() {
	// 	///
	// 	/// Need futher adjustments to deal with specifc error messages in PublicKey
	// 	///
	//     let pub_key_hex = "0x03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816";
	// 	let expected = "03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816";

	//     let pub_key = Secp256r1PublicKey::from_encoded(&pub_key_hex).unwrap();

	//     assert_eq!(
	//         pub_key.get_encoded_compressed_hex(),
	//         expected
	//     );
	// }

	// #[test]
	// pub fn serialize_public_key() {
	// 	///
	// 	/// Need futher adjustments to deal with specifc error messages in PublicKey
	// 	///
	// 	let enc_point = "03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816";
	//     let pub_key = Secp256r1PublicKey::from_encoded(&enc_point).unwrap();

	//     assert_eq!(
	//         pub_key.to_array(),
	//         enc_point.from_hex().unwrap()
	//     );
	// }
}
