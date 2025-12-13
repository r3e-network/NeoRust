//! Constants used throughout the Neo N3 SDK.
//!
//! This module provides access to various constants used in the Neo N3 blockchain,
//! including native contract addresses, network endpoints, and other configuration constants.

pub mod native_contracts;

// Re-export commonly used constants
pub use native_contracts::{
	CONTRACT_MANAGEMENT, CRYPTO_LIB, GAS_TOKEN, LEDGER, NAME_SERVICE, NEO_TOKEN, ORACLE, POLICY,
	ROLE_MANAGEMENT, STD_LIB,
};
