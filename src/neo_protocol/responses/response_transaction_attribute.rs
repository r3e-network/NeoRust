use crate::builder::OracleResponseCode;
use primitive_types::H256;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionAttributeType {
	HighPriority,
	OracleResponse,
	NotValidBefore,
	Conflicts,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq)]
pub struct HighPriorityAttribute {}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq)]
pub struct OracleResponseAttribute {
	#[serde(flatten)]
	pub oracle_response: OracleResponse,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq)]
pub struct NotValidBeforeAttribute {
	#[serde(rename = "height", deserialize_with = "deserialize_height")]
	pub height: i64,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq)]
pub struct ConflictsAttribute {
	#[serde(rename = "hash")]
	pub hash: H256,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum TransactionAttributeEnum {
	#[serde(rename = "HighPriority")]
	HighPriority(HighPriorityAttribute),

	#[serde(rename = "OracleResponse")]
	OracleResponse(OracleResponseAttribute),

	#[serde(rename = "NotValidBefore")]
	NotValidBefore(NotValidBeforeAttribute),

	#[serde(rename = "Conflicts")]
	Conflicts(ConflictsAttribute),
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Debug, Clone)]
pub struct OracleResponse {
	pub(crate) id: u32,
	#[serde(rename = "code")]
	pub(crate) response_code: OracleResponseCode,
	pub(crate) result: String,
}

/// Custom deserialization function for height that handles both number and string JSON values.
fn deserialize_height<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
	D: Deserializer<'de>,
{
	let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
	match value {
		serde_json::Value::Number(num) => {
			num.as_i64().ok_or_else(|| serde::de::Error::custom("invalid number"))
		},
		serde_json::Value::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom),
		_ => Err(serde::de::Error::custom("invalid type for height")),
	}
}
