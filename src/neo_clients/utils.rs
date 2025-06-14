use std::{future::Future, pin::Pin, str::FromStr, sync::Arc, time::Duration};

use crate::{
	builder::VerificationScript,
	config::DEFAULT_ADDRESS_VERSION,
	crypto::{private_key_to_public_key, HashableForVec, Secp256r1PrivateKey, Secp256r1PublicKey},
	neo_clients::ProviderError,
	ScriptHash, ScriptHashExtension,
};
use futures_timer::Delay;
use futures_util::{stream, FutureExt, StreamExt};
use primitive_types::{H160, U256};
use regex::Regex;
use wiremock::{Match, Request};

/// A simple gas escalation policy
pub type EscalationPolicy = Box<dyn Fn(U256, usize) -> U256 + Send + Sync>;

// Helper type alias
#[cfg(target_arch = "wasm32")]
pub(crate) type PinBoxFut<'a, T> = Pin<Box<dyn Future<Output = Result<T, ProviderError>> + 'a>>;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type PinBoxFut<'a, T> =
	Pin<Box<dyn Future<Output = Result<T, ProviderError>> + Send + 'a>>;

/// Calls the future if `item` is None, otherwise returns a `futures::ok`
pub async fn maybe<F, T, E>(item: Option<T>, f: F) -> Result<T, E>
where
	F: Future<Output = Result<T, E>>,
{
	if let Some(item) = item {
		futures_util::future::ok(item).await
	} else {
		f.await
	}
}

/// Create a stream that emits items at a fixed interval. Used for rate control
pub fn interval(duration: Duration) -> impl futures_core::stream::Stream<Item = ()> + Send + Unpin {
	stream::unfold((), move |_| Delay::new(duration).map(|_| Some(((), ())))).map(drop)
}

// A generic function to serialize any data structure that implements Serialize trait
pub fn serialize<T: serde::Serialize>(t: &T) -> serde_json::Value {
	serde_json::to_value(t).expect("Failed to serialize value")
}

/// Convert a script to a script hash.
pub fn script_hash_from_script(script: &[u8]) -> ScriptHash {
	let hash = script.sha256_ripemd160();
	H160::from_slice(&hash)
}

/// Convert a public key to an address.
pub fn public_key_to_address(public_key: &Secp256r1PublicKey) -> String {
	let script_hash = public_key_to_script_hash(public_key);
	script_hash_to_address(&script_hash)
}

/// Convert a public key to a script hash.
pub fn public_key_to_script_hash(public_key: &Secp256r1PublicKey) -> ScriptHash {
	let script = VerificationScript::from_public_key(public_key);
	ScriptHash::from_script(&script.script())
}

/// Convert a private key to a script hash.
pub fn private_key_to_script_hash(private_key: &Secp256r1PrivateKey) -> ScriptHash {
	let pubkey = private_key_to_public_key(private_key);
	public_key_to_script_hash(&pubkey)
}

/// Convert a private key to an address.
pub fn private_key_to_address(private_key: &Secp256r1PrivateKey) -> String {
	let script_hash = private_key_to_script_hash(private_key);
	script_hash_to_address(&script_hash)
}

/// Convert a script hash to an address.
pub fn script_hash_to_address(script_hash: &ScriptHash) -> String {
	let mut data = vec![DEFAULT_ADDRESS_VERSION];
	let mut script_hash_bytes = script_hash.clone().as_bytes().to_vec();
	script_hash_bytes.reverse();
	data.extend_from_slice(&script_hash_bytes);
	let sha = &data.hash256().hash256();
	data.extend_from_slice(&sha[..4]);
	bs58::encode(data).into_string()
}

/// Convert an address to a script hash.
pub fn address_to_script_hash(address: &str) -> Result<ScriptHash, ProviderError> {
	let bytes = match bs58::decode(address).into_vec() {
		Ok(bytes) => bytes,
		Err(_) => return Err(ProviderError::InvalidAddress),
	};
	let _salt = bytes[0];
	let hash = &bytes[1..21];
	let checksum = &bytes[21..25];
	let sha = &bytes[..21].hash256().hash256();
	let check = &sha[..4];
	if checksum != check {
		return Err(ProviderError::InvalidAddress);
	}

	let mut rev = [0u8; 20];
	rev.clone_from_slice(hash);
	rev.reverse();
	Ok(H160::from(&rev))
}

/// Convert a script hash to hex format.
pub fn script_hash_to_hex(script_hash: &ScriptHash) -> String {
	let bytes: [u8; 20] = script_hash.to_fixed_bytes();
	hex::hex::encode(bytes)
}

/// Convert a script hash in hex format to a ScriptHash.
pub fn script_hash_from_hex(hex: &str) -> Result<ScriptHash, ProviderError> {
	H160::from_str(hex).map_err(|_| ProviderError::InvalidAddress)
}

/// Convert an address to hex format.
pub fn address_to_hex(address: &str) -> Result<String, ProviderError> {
	let script_hash = H160::from_address(address)?;
	Ok(hex::hex::encode(script_hash.to_fixed_bytes()))
}

/// Convert a hex format script hash to an address.
pub fn hex_to_address(hex: &str) -> Result<String, ProviderError> {
	let script_hash = H160::from_str(hex).map_err(|_| ProviderError::InvalidAddress)?;
	Ok(script_hash.to_address())
}

pub struct BodyRegexMatcher {
	pattern: Arc<Regex>,
}

impl BodyRegexMatcher {
	pub fn new(pattern: &str) -> Self {
		BodyRegexMatcher { pattern: Arc::new(Regex::new(pattern).expect("Invalid regex pattern")) }
	}
}

impl Match for BodyRegexMatcher {
	fn matches(&self, request: &Request) -> bool {
		std::str::from_utf8(&request.body)
			.map(|body_str| self.pattern.is_match(body_str))
			.unwrap_or(false)
	}
}
