//! Network and runtime configuration for the Neo N3 SDK.
//!
//! This module owns the global, mutable view of which Neo N3 network the
//! SDK is targeting (MainNet vs TestNet vs PrivateNet), the magic numbers
//! and constants associated with each network, and the helpers used by
//! transaction-builder code paths to look those values up.
//!
//! Most application code should _not_ touch this module directly — instead,
//! pass a [`sdk::Network`](crate::sdk::Network) value to
//! [`sdk::Neo::builder()`](crate::sdk::Neo::builder), which configures the
//! relevant constants internally.
//!
//! ## What's here
//!
//! - [`NeoConfig`] — runtime mutable configuration (network magic, block
//!   interval, validators, NNS resolver address, etc.).
//! - [`NeoConstants`] — compile-time protocol constants (seed nodes, well-
//!   known contract hashes, gas constants, …).
//! - [`TestConstants`] — fixtures used by SDK tests and quick demos; not
//!   intended for production code paths.

pub use config::*;
pub use constant::*;
pub use test_properties::*;

mod config;
mod constant;
mod test_properties;
