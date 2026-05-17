//! Binary encoding and decoding for the Neo N3 wire format.
//!
//! This module implements the byte-level serialization rules the Neo N3
//! protocol uses on the wire and inside `NeoVM`. It is what
//! [`neo_builder`](crate::neo_builder) reaches for when assembling
//! transactions and what [`neo_clients`](crate::neo_clients) uses to parse
//! raw RPC payloads.
//!
//! ## Layers
//!
//! - [`Encoder`] / [`Decoder`] — low-level cursors over `&mut Vec<u8>` and
//!   `&[u8]` for primitives (varint, fixed-size integers, byte arrays, …).
//! - [`NeoSerializable`] — derived trait for types that round-trip through
//!   the wire format.
//! - [`CodecError`] — failures during decoding (truncated input, invalid
//!   discriminants, length-prefix overflow).
//!
//! ## Stability
//!
//! The encoding tracks the Neo N3 reference implementation (C#); breaking
//! changes here would imply a protocol fork and are therefore extremely
//! rare. Higher-level abstractions in [`neo_builder`](crate::neo_builder)
//! should be preferred over hand-rolled codec usage.

pub use binary_decoder::*;
pub use binary_encoder::*;
pub use encode::*;
pub use error::*;

mod binary_decoder;
mod binary_encoder;
mod encode;
mod error;
