//! # NeoRust prelude
//!
//! A curated re-export of the most commonly used types, traits, and helpers
//! from across the crate. Pull it in once and skip the import boilerplate:
//!
//! ```rust
//! use neo3::prelude::*;
//! ```
//!
//! ## What you get
//!
//! | Category | Highlights |
//! |----------|------------|
//! | Primitives | [`Address`], [`ScriptHash`], [`Bytes`], `H160`, `H256`, `U256` |
//! | Contracts | [`ContractManifest`], [`ContractParameter`], [`InvocationResult`], [`NefFile`] |
//! | VM | [`OpCode`], [`StackItem`], [`VMState`] |
//! | NNS | [`NNSName`] |
//! | Errors | Legacy `NeoError` alias (see [`crate::neo_error::unified::NeoError`] for the modern type) |
//! | Module aliases | `builder`, `providers`, `codec`, `config`, `crypto`, `protocol`, `wallets`, `x` |
//! | Encoding helpers | [`Base64Encode`], [`StringExt`], `ToBase58`, `ToHexString`, … |
//!
//! ## When to import the prelude vs explicit paths
//!
//! - **Use the prelude** when writing application code that touches many parts
//!   of the SDK (transactions, contracts, wallets, RPC).
//! - **Use explicit imports** in library code, or when you only need one or
//!   two types — it keeps namespaces clean and helps incremental compilation.
//!
//! [`neo_error::unified::NeoError`]: crate::neo_error::unified::NeoError
// Core error type (legacy alias)
pub use crate::neo_error::NeoError;

// SDK version
pub use crate::VERSION;

// === Core Types ===
// Basic blockchain types
pub use crate::neo_types::{
	Address, AddressOrScriptHash, Base64Encode, Bytes, NameOrAddress, ScriptHash,
	ScriptHashExtension, StringExt, ToBase58, TryBase64Encode, TryStringExt,
};

// Contract-related types
pub use crate::neo_types::{
	ContractManifest, ContractParameter, ContractParameterType, ContractState, InvocationResult,
	NefFile,
};

// VM and runtime types
pub use crate::neo_types::{OpCode, StackItem, VMState};

// NNS-related types
pub use crate::neo_types::NNSName;

// Common external types
pub use primitive_types::{H160, H256, U256};
pub use serde_json::Value as ParameterValue;
pub use url::Url;

// === Serialization Helpers ===
pub use crate::neo_types::{
	// H160/H256 serialization
	deserialize_h160,
	deserialize_h256,
	// Other serialization helpers
	deserialize_script_hash,
	// U256 serialization
	deserialize_u256,
	deserialize_u64,
	deserialize_vec_h256,
	deserialize_vec_u256,
	deserialize_wildcard,
	serialize_h160,
	serialize_h256,
	serialize_script_hash,
	serialize_u256,
	serialize_u64,

	serialize_vec_h256,

	serialize_vec_u256,
	serialize_wildcard,
};

// === Core Functionality Modules ===
// These are aliased module names for user convenience
pub use crate::{
	neo_builder as builder, neo_clients as providers, neo_codec as codec, neo_config as config,
	neo_crypto as crypto, neo_protocol as protocol, neo_wallets as wallets, neo_x as x,
};

// === Extension modules ===
// These are full modules that provide specialized functionality
pub use crate::neo_fs; // NeoFS distributed storage

// Re-export ValueExtension
pub use crate::neo_types::ValueExtension;

// === Utility Traits ===
// Hex and base64 encoding/decoding utilities
pub use crate::neo_crypto::utils::{FromBase64String, FromHexString, ToHexString};
