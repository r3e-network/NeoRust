use std::{fmt::Debug, hash::Hash};

use crate::neo_crypto::utils::FromBase64String;
use primitive_types::H256;
use serde::{Deserialize, Serialize};

use crate::{
	builder::TransactionError,
	codec::{Decoder, Encoder, NeoSerializable},
	prelude::Base64Encode,
};

use super::oracle_response_code::OracleResponseCode;

#[derive(Serialize, Deserialize, PartialEq, Hash, Debug, Clone)]
#[serde(tag = "type")]
pub enum TransactionAttribute {
	#[serde(rename = "HighPriority")]
	HighPriority,

	#[serde(rename = "OracleResponse")]
	OracleResponse(OracleResponse),

	#[serde(rename = "NotValidBefore")]
	NotValidBefore { height: u32 },

	#[serde(rename = "Conflicts")]
	Conflicts { hash: H256 },

	#[serde(rename = "NotaryAssisted")]
	NotaryAssisted { n: u16 },
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Debug, Clone)]
pub struct OracleResponse {
	pub id: u64,
	pub response_code: OracleResponseCode,
	pub result: String,
}

impl TransactionAttribute {
	pub const MAX_RESULT_SIZE: usize = 0xffff;

	pub fn to_bytes(&self) -> Vec<u8> {
		self.to_array()
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
		let mut reader = Decoder::new(bytes);
		Self::decode(&mut reader).map_err(|_| "Invalid transaction attribute")
	}

	pub fn to_json(&self) -> String {
		serde_json::to_string(self).unwrap_or_else(|e| {
			tracing::warn!(error = %e, "Failed to serialize TransactionAttribute to JSON");
			String::new()
		})
	}

	// Get the height for NotValidBefore attribute
	pub fn get_height(&self) -> Option<&u32> {
		match self {
			TransactionAttribute::NotValidBefore { height } => Some(height),
			_ => None,
		}
	}

	// Get the hash for Conflicts attribute
	pub fn get_hash(&self) -> Option<&H256> {
		match self {
			TransactionAttribute::Conflicts { hash } => Some(hash),
			_ => None,
		}
	}
}

impl NeoSerializable for TransactionAttribute {
	type Error = TransactionError;

	fn size(&self) -> usize {
		match self {
			TransactionAttribute::HighPriority => 1,
			TransactionAttribute::OracleResponse(OracleResponse {
				id: _,
				response_code: _,
				result,
			}) => {
				let len = result.len() as u64;
				let var_int_size = if len < 0xfd {
					1
				} else if len <= 0xffff {
					3
				} else if len <= 0xffffffff {
					5
				} else {
					9
				};
				1 + 8 + 1 + var_int_size + result.len()
			},
			TransactionAttribute::NotValidBefore { height: _ } => 1 + 4,
			TransactionAttribute::Conflicts { hash: _ } => 1 + 32,
			TransactionAttribute::NotaryAssisted { n: _ } => 1 + 2,
		}
	}

	fn encode(&self, writer: &mut Encoder) {
		match self {
			TransactionAttribute::HighPriority => {
				writer.write_u8(0x01);
			},
			TransactionAttribute::OracleResponse(OracleResponse { id, response_code, result }) => {
				writer.write_u8(0x11);
				writer.write_u64(*id);
				writer.write_u8(*response_code as u8);
				let decoded = match result.from_base64_string() {
					Ok(bytes) => bytes,
					Err(err) => {
						tracing::warn!(
							error = %err,
							"OracleResponse.result is not valid base64; encoding raw string bytes"
						);
						result.as_bytes().to_vec()
					},
				};
				if let Err(e) = writer.write_var_bytes(&decoded) {
					tracing::warn!(error = %e, "Failed to encode oracle response");
				}
			},
			TransactionAttribute::NotValidBefore { height } => {
				writer.write_u8(0x20);
				writer.write_u32(*height);
				// We assume Post-Domovoi, so no 28-byte padding.
				// If we needed to support pre-Domovoi, we'd need context which Encoder doesn't have easily.
				// Given this is a Neo 3.9 SDK, we use the modern format.
			},
			TransactionAttribute::Conflicts { hash } => {
				writer.write_u8(0x21);
				writer.write_bytes(hash.as_bytes());
			},
			TransactionAttribute::NotaryAssisted { n } => {
				writer.write_u8(0x22);
				writer.write_u16(*n);
			},
		}
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		match reader.read_u8_safe()? {
			0x01 => Ok(TransactionAttribute::HighPriority),
			0x11 => {
				let id = reader.read_u64().map_err(|e| {
					TransactionError::TransactionConfiguration(format!(
						"Failed to read oracle response ID: {}",
						e
					))
				})?;

				let response_code_byte = reader.read_u8_safe()?;
				let response_code =
					OracleResponseCode::try_from(response_code_byte).map_err(|_| {
						TransactionError::TransactionConfiguration(
							"Invalid oracle response code".to_string(),
						)
					})?;
				let result = reader
					.read_var_bytes_bounded(Self::MAX_RESULT_SIZE)
					.map_err(|e| {
						TransactionError::TransactionConfiguration(format!(
							"Failed to read oracle response result: {}",
							e
						))
					})?
					.to_base64();

				Ok(TransactionAttribute::OracleResponse(OracleResponse {
					id,
					response_code,
					result,
				}))
			},
			0x20 => {
				let height = reader.read_u32().map_err(|e| {
					TransactionError::TransactionConfiguration(format!(
						"Failed to read NotValidBefore height: {}",
						e
					))
				})?;
				// Again, we assume post-Domovoi format (no padding consumer)
				Ok(TransactionAttribute::NotValidBefore { height })
			},
			0x21 => {
				let hash_bytes = reader.read_bytes(32).map_err(|e| {
					TransactionError::TransactionConfiguration(format!(
						"Failed to read Conflicts hash: {}",
						e
					))
				})?;
				let hash = H256::from_slice(&hash_bytes);
				Ok(TransactionAttribute::Conflicts { hash })
			},
			0x22 => {
				let n = reader.read_u16().map_err(|e| {
					TransactionError::TransactionConfiguration(format!(
						"Failed to read NotaryAssisted n: {}",
						e
					))
				})?;
				Ok(TransactionAttribute::NotaryAssisted { n })
			},
			t => Err(TransactionError::TransactionConfiguration(format!(
				"Invalid transaction attribute type: {}",
				t
			))),
		}
	}

	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}
