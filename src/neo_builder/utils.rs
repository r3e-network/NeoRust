use serde_json::Value;

use crate::{
	builder::{
		ScriptBuilder, Signer, TransactionAttribute, TransactionSendToken, TransactionSigner,
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
	let threshold_u8 = match u8::try_from(threshold) {
		Ok(v) if v > 0 => v,
		_ => {
			tracing::warn!(threshold, "Invalid multi-sig threshold; returning zero script hash");
			return ScriptHash::zero();
		},
	};

	if threshold > public_keys.len() {
		tracing::warn!(
			threshold,
			public_keys_len = public_keys.len(),
			"Multi-sig threshold exceeds public key count; returning zero script hash"
		);
		return ScriptHash::zero();
	}

	match ScriptBuilder::build_multi_sig_script(public_keys, threshold_u8) {
		Ok(script) => ScriptHash::from_script(&script),
		Err(e) => {
			tracing::warn!(error = %e, "Failed to build multi-sig script; returning zero script hash");
			ScriptHash::zero()
		},
	}
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

impl ValueExtension for TransactionAttribute {
	fn to_value(&self) -> Value {
		Value::String(self.to_json())
	}
}

impl ValueExtension for TransactionSendToken {
	fn to_value(&self) -> Value {
		Value::String(serde_json::to_string(self).unwrap_or_else(|e| {
			tracing::warn!(error = %e, "Failed to serialize TransactionSendToken to JSON");
			String::new()
		}))
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
		Value::String(serde_json::to_string(self).unwrap_or_else(|e| {
			tracing::warn!(error = %e, "Failed to serialize Signer to JSON");
			String::new()
		}))
	}
}

impl VecValueExtension for Vec<Signer> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}

impl ValueExtension for TransactionSigner {
	fn to_value(&self) -> Value {
		Value::String(serde_json::to_string(self).unwrap_or_else(|e| {
			tracing::warn!(error = %e, "Failed to serialize TransactionSigner to JSON");
			String::new()
		}))
	}
}

impl VecValueExtension for Vec<TransactionSigner> {
	fn to_value(&self) -> Value {
		self.iter().map(|x| x.to_value()).collect()
	}
}
