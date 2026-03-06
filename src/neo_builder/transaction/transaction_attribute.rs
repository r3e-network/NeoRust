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
	NotaryAssisted { n: u8 },
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Debug, Clone)]
pub struct OracleResponse {
	pub id: u64,
	pub response_code: OracleResponseCode,
	pub result: String,
}

impl TransactionAttribute {
	pub const MAX_RESULT_SIZE: usize = 0xffff;

	fn try_oracle_response_result_bytes(result: &str) -> Result<Vec<u8>, TransactionError> {
		result.from_base64_string().map_err(|err| {
			TransactionError::TransactionConfiguration(format!(
				"OracleResponse.result must be valid base64: {}",
				err
			))
		})
	}

	fn oracle_response_serialized_size(decoded_len: usize) -> usize {
		let len = decoded_len as u64;
		let var_int_size = if len < 0xfd {
			1
		} else if len <= 0xffff {
			3
		} else if len <= 0xffffffff {
			5
		} else {
			9
		};
		1 + 8 + 1 + var_int_size + decoded_len
	}

	pub fn try_size(&self) -> Result<usize, TransactionError> {
		match self {
			TransactionAttribute::HighPriority => Ok(1),
			TransactionAttribute::OracleResponse(OracleResponse {
				id: _,
				response_code: _,
				result,
			}) => {
				let decoded_len = Self::try_oracle_response_result_bytes(result)?.len();
				Ok(Self::oracle_response_serialized_size(decoded_len))
			},
			TransactionAttribute::NotValidBefore { height: _ } => Ok(1 + 4),
			TransactionAttribute::Conflicts { hash: _ } => Ok(1 + 32),
			TransactionAttribute::NotaryAssisted { n: _ } => Ok(1 + 1),
		}
	}

	pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError> {
		match self {
			TransactionAttribute::HighPriority => writer.write_u8(0x01),
			TransactionAttribute::OracleResponse(OracleResponse { id, response_code, result }) => {
				let decoded = Self::try_oracle_response_result_bytes(result)?;
				writer.write_u8(0x11);
				writer.write_u64(*id);
				writer.write_u8(*response_code as u8);
				writer.write_var_bytes(&decoded).map_err(|err| {
					TransactionError::TransactionConfiguration(format!(
						"Failed to encode oracle response: {}",
						err
					))
				})?;
			},
			TransactionAttribute::NotValidBefore { height } => {
				writer.write_u8(0x20);
				writer.write_u32(*height);
			},
			TransactionAttribute::Conflicts { hash } => {
				writer.write_u8(0x21);
				writer.write_bytes(hash.as_bytes());
			},
			TransactionAttribute::NotaryAssisted { n } => {
				writer.write_u8(0x22);
				writer.write_u8(*n);
			},
		}

		Ok(())
	}

	pub fn try_to_bytes(&self) -> Result<Vec<u8>, TransactionError> {
		let mut writer = Encoder::new();
		self.try_encode(&mut writer)?;
		Ok(writer.to_bytes())
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		self.try_to_bytes().unwrap_or_else(|err| {
			tracing::warn!(
				error = %err,
				"Failed to serialize transaction attribute via safe path; falling back to legacy encoder"
			);
			self.to_array()
		})
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
		let mut reader = Decoder::new(bytes);
		Self::decode(&mut reader).map_err(|_| "Invalid transaction attribute")
	}

	pub fn try_to_json(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}

	pub fn to_json(&self) -> String {
		self.try_to_json().unwrap_or_else(|e| {
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
				let decoded_len =
					result.from_base64_string().map(|bytes| bytes.len()).unwrap_or(result.len());
				Self::oracle_response_serialized_size(decoded_len)
			},
			TransactionAttribute::NotValidBefore { height: _ } => 1 + 4,
			TransactionAttribute::Conflicts { hash: _ } => 1 + 32,
			TransactionAttribute::NotaryAssisted { n: _ } => 1 + 1,
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
				writer.write_u8(*n);
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
				let n = reader.read_u8_safe()?;
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{codec::Encoder, prelude::Base64Encode};
	use primitive_types::H256;

	#[test]
	fn test_try_to_json_matches_serde_json() {
		let attribute = TransactionAttribute::Conflicts { hash: H256::zero() };
		assert_eq!(attribute.try_to_json().unwrap(), serde_json::to_string(&attribute).unwrap());
	}

	#[test]
	fn test_try_to_bytes_rejects_invalid_oracle_response_base64() {
		let attribute = TransactionAttribute::OracleResponse(OracleResponse {
			id: 1,
			response_code: OracleResponseCode::Success,
			result: "not-base64".to_string(),
		});

		assert!(matches!(
			attribute.try_to_bytes(),
			Err(TransactionError::TransactionConfiguration(message))
				if message.contains("valid base64")
		));
	}

	#[test]
	fn test_try_to_bytes_matches_legacy_for_valid_oracle_response() {
		let attribute = TransactionAttribute::OracleResponse(OracleResponse {
			id: 7,
			response_code: OracleResponseCode::Success,
			result: vec![1_u8, 2, 3].to_base64(),
		});

		assert_eq!(attribute.try_to_bytes().unwrap(), attribute.to_bytes());
	}

	#[test]
	fn test_try_encode_rejects_invalid_oracle_response_base64() {
		let attribute = TransactionAttribute::OracleResponse(OracleResponse {
			id: 1,
			response_code: OracleResponseCode::Success,
			result: "not-base64".to_string(),
		});
		let mut writer = Encoder::new();

		assert!(matches!(
			attribute.try_encode(&mut writer),
			Err(TransactionError::TransactionConfiguration(message))
				if message.contains("valid base64")
		));
	}

	#[test]
	fn test_try_encode_matches_safe_bytes_for_valid_oracle_response() {
		let attribute = TransactionAttribute::OracleResponse(OracleResponse {
			id: 7,
			response_code: OracleResponseCode::Success,
			result: vec![1_u8, 2, 3].to_base64(),
		});
		let mut writer = Encoder::new();

		attribute.try_encode(&mut writer).unwrap();
		assert_eq!(writer.to_bytes(), attribute.try_to_bytes().unwrap());
	}

	#[test]
	fn test_try_size_rejects_invalid_oracle_response_base64() {
		let attribute = TransactionAttribute::OracleResponse(OracleResponse {
			id: 1,
			response_code: OracleResponseCode::Success,
			result: "not-base64".to_string(),
		});

		assert!(matches!(
			attribute.try_size(),
			Err(TransactionError::TransactionConfiguration(message))
				if message.contains("valid base64")
		));
	}

	#[test]
	fn test_try_size_matches_safe_bytes_len_for_valid_oracle_response() {
		let attribute = TransactionAttribute::OracleResponse(OracleResponse {
			id: 7,
			response_code: OracleResponseCode::Success,
			result: vec![1_u8, 2, 3].to_base64(),
		});

		assert_eq!(attribute.try_size().unwrap(), attribute.try_to_bytes().unwrap().len());
	}
}
