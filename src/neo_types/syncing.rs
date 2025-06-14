//! Types for `neo_syncing` RPC call

use ethereum_types::U64;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Structure used in `neo_syncing` RPC
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyncingStatus {
	/// When client is synced to highest block, neo_syncing with return string "false"
	IsFalse,
	/// When client is still syncing past blocks we get IsSyncing information.
	IsSyncing(Box<SyncProgress>),
}

impl Serialize for SyncingStatus {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			SyncingStatus::IsFalse => serializer.serialize_bool(false),
			SyncingStatus::IsSyncing(sync) => sync.serialize(serializer),
		}
	}
}

impl<'de> Deserialize<'de> for SyncingStatus {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Debug, Serialize, Deserialize)]
		#[serde(untagged)]
		pub(crate) enum SyncingStatusIntermediate {
			/// When client is synced to the highest block, neo_syncing with return string "false"
			IsFalse(bool),
			/// When client is still syncing past blocks we get IsSyncing information.
			IsSyncing(Box<SyncProgress>),
		}

		match SyncingStatusIntermediate::deserialize(deserializer)? {
			SyncingStatusIntermediate::IsFalse(false) => Ok(SyncingStatus::IsFalse),
			SyncingStatusIntermediate::IsFalse(true) => Err(serde::de::Error::custom(
				"neo_syncing returned `true` that is undefined value.",
			)),
			SyncingStatusIntermediate::IsSyncing(sync) => Ok(SyncingStatus::IsSyncing(sync)),
		}
	}
}

/// Represents the sync status of the node
///
/// **Note:** while the `neo_syncing` RPC response is defined as:
///
/// > Returns:
/// >
/// > Object|Boolean, An object with sync status data or FALSE, when not syncing:
///
/// > startingBlock: QUANTITY - The block at which the import started (will only be reset, after the
/// > sync reached his head)
/// > currentBlock: QUANTITY - The current block, same as neo_blockNumber
/// > highestBlock: QUANTITY - The estimated highest block
///
/// Geth returns additional fields: <https://github.com/neo/go-neo/blob/0ce494b60cd00d70f1f9f2dd0b9bfbd76204168a/ethclient/ethclient.go#L597-L617>
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Struct to track progress of node syncing
pub struct SyncProgress {
	/// Current block number we've processed
	pub current_block: U64,

	/// Highest known block number on the chain
	pub highest_block: U64,

	/// Block number we started syncing from
	pub starting_block: U64,

	/// Number of state entries pulled so far (optional)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub pulled_states: Option<U64>,

	/// Total number of known state entries (optional)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub known_states: Option<U64>,

	/// Bytes of bytecodes healed
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub healed_bytecode_bytes: Option<U64>,

	/// Number of bytecode entries healed
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub healed_bytecodes: Option<U64>,

	/// Bytes of trie nodes healed
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub healed_trienode_bytes: Option<U64>,

	/// Number of trie nodes healed
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub healed_trienodes: Option<U64>,

	/// Bytecode entries currently healing
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub healing_bytecode: Option<U64>,

	/// Trie nodes currently healing
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub healing_trienodes: Option<U64>,

	/// Bytes of account data synced
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub synced_account_bytes: Option<U64>,

	/// Accounts synced
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub synced_accounts: Option<U64>,

	/// Bytes of bytecode synced
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub synced_bytecode_bytes: Option<U64>,

	/// Bytecode entries synced
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub synced_bytecodes: Option<U64>,

	/// Storage entries synced
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub synced_storage: Option<U64>,

	/// Bytes of storage synced
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub synced_storage_bytes: Option<U64>,
}

#[cfg(test)]
mod tests {
	use super::*;

	// <https://github.com/gakonst/neo-rs/issues/1623>
	#[test]
	fn deserialize_sync_geth() {
		let s = r#"{
        "currentBlock": "0xeaa2b4",
        "healedBytecodeBytes": "0xaad91fe",
        "healedBytecodes": "0x61d3",
        "healedTrienodeBytes": "0x156ac02b1",
        "healedTrienodes": "0x2885aa4",
        "healingBytecode": "0x0",
        "healingTrienodes": "0x454",
        "highestBlock": "0xeaa329",
        "startingBlock": "0xea97ee",
        "syncedAccountBytes": "0xa29fec90d",
        "syncedAccounts": "0xa7ed9ad",
        "syncedBytecodeBytes": "0xdec39008",
        "syncedBytecodes": "0x8d407",
        "syncedStorage": "0x2a517da1",
        "syncedStorageBytes": "0x23634dbedf"
    }"#;

		let sync: SyncingStatus = serde_json::from_str(s).unwrap();
		match sync {
			SyncingStatus::IsFalse => {
				assert!(false, "Expected IsSyncing variant, got IsFalse")
			},
			SyncingStatus::IsSyncing(_) => {},
		}
	}

	#[test]
	fn deserialize_sync_minimal() {
		let s = r#"{
        "currentBlock": "0xeaa2b4",
        "highestBlock": "0xeaa329",
        "startingBlock": "0xea97ee"
    }"#;

		let sync: SyncingStatus = serde_json::from_str(s).unwrap();
		match sync {
			SyncingStatus::IsFalse => {
				assert!(false, "Expected IsSyncing variant, got IsFalse")
			},
			SyncingStatus::IsSyncing(_) => {},
		}
	}

	#[test]
	fn deserialize_sync_false() {
		let s = r"false";

		let sync: SyncingStatus = serde_json::from_str(s).unwrap();
		match sync {
			SyncingStatus::IsFalse => {},
			SyncingStatus::IsSyncing(_) => {
				assert!(false, "Expected IsFalse variant, got IsSyncing")
			},
		}
	}
}
