//! Policy native contract for Neo N3.
//!
//! The Policy contract manages system policies including fees, blocked accounts,
//! and various network parameters.
//!
//! Updated for Neo 3.9 with support for:
//! - Milliseconds per block (HF_Echidna)
//! - Max valid until block increment (HF_Echidna)
//! - Max traceable blocks (HF_Echidna)
//! - Attribute fees (HF_Echidna)
//! - Execution fee factor with decimals (HF_Faun)
//! - Whitelist fee contracts (HF_Faun)
//! - Fund recovery from blocked accounts (HF_Faun)

use async_trait::async_trait;
use primitive_types::H160;
use serde::{Deserialize, Serialize};

use crate::{
	neo_builder::TransactionBuilder,
	neo_clients::{JsonRpcProvider, RpcClient},
	neo_contract::{traits::SmartContractTrait, ContractError},
	neo_types::{
		serde_with_utils::{deserialize_script_hash, serialize_script_hash},
		ScriptHash,
	},
	ScriptHashExtension,
};

/// The Policy native contract for managing system policies.
///
/// This contract controls network parameters such as fees, blocked accounts,
/// and various protocol settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyContract<'a, P: JsonRpcProvider> {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	script_hash: ScriptHash,
	#[serde(skip)]
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, P: JsonRpcProvider + 'static> PolicyContract<'a, P> {
	/// The name of the Policy contract.
	pub const NAME: &'static str = "PolicyContract";

	/// The default execution fee factor.
	pub const DEFAULT_EXEC_FEE_FACTOR: u32 = 30;

	/// The default storage price.
	pub const DEFAULT_STORAGE_PRICE: u32 = 100000;

	/// The default network fee per byte (in datoshi, 1 datoshi = 1e-8 GAS).
	pub const DEFAULT_FEE_PER_BYTE: u32 = 1000;

	/// The default fee for transaction attributes.
	pub const DEFAULT_ATTRIBUTE_FEE: u32 = 0;

	/// The default fee for NotaryAssisted attribute (in datoshi).
	pub const DEFAULT_NOTARY_ASSISTED_ATTRIBUTE_FEE: u32 = 10_000_000;

	/// The maximum execution fee factor.
	pub const MAX_EXEC_FEE_FACTOR: u64 = 100;

	/// The maximum fee for transaction attributes.
	pub const MAX_ATTRIBUTE_FEE: u32 = 10_0000_0000;

	/// The maximum storage price.
	pub const MAX_STORAGE_PRICE: u32 = 10_000_000;

	/// The maximum block generation time in milliseconds.
	pub const MAX_MILLISECONDS_PER_BLOCK: u32 = 30_000;

	/// The maximum MaxValidUntilBlockIncrement value (1 day of 1-second blocks).
	pub const MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT: u32 = 86400;

	/// The maximum MaxTraceableBlocks value (1 year of 15-second blocks).
	pub const MAX_MAX_TRACEABLE_BLOCKS: u32 = 2_102_400;

	/// Creates a new PolicyContract instance.
	pub fn new(provider: Option<&'a RpcClient<P>>) -> Self {
		Self { script_hash: Self::calc_native_contract_hash_unchecked(Self::NAME), provider }
	}

	// ========== Basic Policy Methods ==========

	/// Gets the network fee per transaction byte (in datoshi).
	pub async fn get_fee_per_byte(&self) -> Result<i64, ContractError> {
		Ok(self.call_function_returning_int("getFeePerByte", vec![]).await? as i64)
	}

	/// Gets the execution fee factor.
	///
	/// This is a multiplier used to calculate system fees for transactions.
	/// After HF_Faun, this returns the factor without decimals (use `get_exec_pico_fee_factor`
	/// for the full precision value).
	pub async fn get_exec_fee_factor(&self) -> Result<u32, ContractError> {
		Ok(self.call_function_returning_int("getExecFeeFactor", vec![]).await? as u32)
	}

	/// Gets the execution fee factor in pico-GAS (1 pico-GAS = 1e-12 GAS).
	///
	/// This method is available after HF_Faun and returns the fee factor with full decimal precision.
	pub async fn get_exec_pico_fee_factor(&self) -> Result<i64, ContractError> {
		Ok(self.call_function_returning_int("getExecPicoFeeFactor", vec![]).await? as i64)
	}

	/// Gets the storage price (cost per byte of storage).
	pub async fn get_storage_price(&self) -> Result<u32, ContractError> {
		Ok(self.call_function_returning_int("getStoragePrice", vec![]).await? as u32)
	}

	// ========== Block Time and Limits (HF_Echidna) ==========

	/// Gets the block generation time in milliseconds.
	///
	/// This method is available after HF_Echidna.
	pub async fn get_milliseconds_per_block(&self) -> Result<u32, ContractError> {
		Ok(self.call_function_returning_int("getMillisecondsPerBlock", vec![]).await? as u32)
	}

	/// Sets the block generation time in milliseconds.
	///
	/// This method requires committee approval.
	/// Value must be between 1 and MAX_MILLISECONDS_PER_BLOCK (30,000).
	///
	/// Available after HF_Echidna.
	pub async fn set_milliseconds_per_block(
		&self,
		value: u32,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setMillisecondsPerBlock", vec![value.into()]).await
	}

	/// Gets the maximum ValidUntilBlock increment for transactions.
	///
	/// This method is available after HF_Echidna.
	pub async fn get_max_valid_until_block_increment(&self) -> Result<u32, ContractError> {
		Ok(self.call_function_returning_int("getMaxValidUntilBlockIncrement", vec![]).await? as u32)
	}

	/// Sets the maximum ValidUntilBlock increment.
	///
	/// This method requires committee approval.
	/// Value must be between 1 and MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT (86,400)
	/// and less than MaxTraceableBlocks.
	///
	/// Available after HF_Echidna.
	pub async fn set_max_valid_until_block_increment(
		&self,
		value: u32,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setMaxValidUntilBlockIncrement", vec![value.into()]).await
	}

	/// Gets the maximum number of blocks accessible to smart contracts.
	///
	/// This method is available after HF_Echidna.
	pub async fn get_max_traceable_blocks(&self) -> Result<u32, ContractError> {
		Ok(self.call_function_returning_int("getMaxTraceableBlocks", vec![]).await? as u32)
	}

	/// Sets the maximum number of traceable blocks.
	///
	/// This method requires committee approval.
	/// Value must be between 1 and MAX_MAX_TRACEABLE_BLOCKS (2,102,400),
	/// cannot be increased, and must be greater than MaxValidUntilBlockIncrement.
	///
	/// Available after HF_Echidna.
	pub async fn set_max_traceable_blocks(
		&self,
		value: u32,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setMaxTraceableBlocks", vec![value.into()]).await
	}

	// ========== Attribute Fees (HF_Echidna) ==========

	/// Gets the fee for a specific transaction attribute type.
	///
	/// Available after HF_Echidna.
	///
	/// # Arguments
	///
	/// * `attribute_type` - The transaction attribute type (as a byte value).
	pub async fn get_attribute_fee(&self, attribute_type: u8) -> Result<u32, ContractError> {
		Ok(self.call_function_returning_int("getAttributeFee", vec![attribute_type.into()]).await?
			as u32)
	}

	/// Sets the fee for a specific transaction attribute type.
	///
	/// This method requires committee approval.
	///
	/// Available after HF_Echidna.
	///
	/// # Arguments
	///
	/// * `attribute_type` - The transaction attribute type (as a byte value).
	/// * `value` - The fee value (must be <= MAX_ATTRIBUTE_FEE).
	pub async fn set_attribute_fee(
		&self,
		attribute_type: u8,
		value: u32,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setAttributeFee", vec![attribute_type.into(), value.into()]).await
	}

	// ========== Account Blocking ==========

	/// Checks if the specified account is blocked.
	pub async fn is_blocked(&self, script_hash: &H160) -> Result<bool, ContractError> {
		self.call_function_returning_bool("isBlocked", vec![script_hash.into()]).await
	}

	/// Blocks an account.
	///
	/// This method requires committee approval.
	/// After HF_Faun, blocking an account will also clear its votes.
	pub async fn block_account(
		&self,
		account: &H160,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("blockAccount", vec![account.into()]).await
	}

	/// Blocks an account by address.
	///
	/// This method requires committee approval.
	pub async fn block_account_address(
		&self,
		address: &str,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let account = ScriptHash::from_address(address)
			.map_err(|_| ContractError::InvalidAccount("Invalid address".to_string()))?;
		self.block_account(&account).await
	}

	/// Unblocks an account.
	///
	/// This method requires committee approval.
	pub async fn unblock_account(
		&self,
		account: &H160,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("unblockAccount", vec![account.into()]).await
	}

	/// Unblocks an account by address.
	///
	/// This method requires committee approval.
	pub async fn unblock_account_address(
		&self,
		address: &str,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let account = ScriptHash::from_address(address)
			.map_err(|_| ContractError::InvalidAccount("Invalid address".to_string()))?;
		self.unblock_account(&account).await
	}

	// ========== Fee Factor and Price Setters ==========

	/// Sets the network fee per byte.
	///
	/// This method requires committee approval.
	/// Value must be between 0 and 100,000,000.
	pub async fn set_fee_per_byte(
		&self,
		fee: i64,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setFeePerByte", vec![fee.into()]).await
	}

	/// Sets the execution fee factor.
	///
	/// This method requires committee approval.
	/// After HF_Faun, the maximum value is increased to account for decimals.
	pub async fn set_exec_fee_factor(
		&self,
		fee: u64,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setExecFeeFactor", vec![fee.into()]).await
	}

	/// Sets the storage price.
	///
	/// This method requires committee approval.
	/// Value must be between 1 and MAX_STORAGE_PRICE (10,000,000).
	pub async fn set_storage_price(
		&self,
		price: u32,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("setStoragePrice", vec![price.into()]).await
	}

	// ========== Whitelist Fee Contracts (HF_Faun) ==========

	/// Sets a whitelisted contract method for fee exemption.
	///
	/// When a contract method is whitelisted, invocations use the specified fixed fee
	/// instead of the calculated execution fee.
	///
	/// This method requires committee approval.
	/// Available after HF_Faun.
	///
	/// # Arguments
	///
	/// * `contract_hash` - The contract to whitelist.
	/// * `method` - The method name.
	/// * `arg_count` - The number of arguments the method takes.
	/// * `fixed_fee` - The fixed execution fee for this method.
	pub async fn set_whitelist_fee_contract(
		&self,
		contract_hash: &H160,
		method: &str,
		arg_count: i32,
		fixed_fee: i64,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function(
			"setWhitelistFeeContract",
			vec![contract_hash.into(), method.into(), arg_count.into(), fixed_fee.into()],
		)
		.await
	}

	/// Removes a whitelisted contract method.
	///
	/// This method requires committee approval.
	/// Available after HF_Faun.
	///
	/// # Arguments
	///
	/// * `contract_hash` - The contract to remove from whitelist.
	/// * `method` - The method name.
	/// * `arg_count` - The number of arguments the method takes.
	pub async fn remove_whitelist_fee_contract(
		&self,
		contract_hash: &H160,
		method: &str,
		arg_count: i32,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function(
			"removeWhitelistFeeContract",
			vec![contract_hash.into(), method.into(), arg_count.into()],
		)
		.await
	}

	// ========== Fund Recovery (HF_Faun) ==========

	/// Recovers funds from a blocked account.
	///
	/// This method requires almost full committee approval and can only be called
	/// after the account has been blocked for at least 1 year.
	///
	/// Available after HF_Faun.
	///
	/// # Arguments
	///
	/// * `account` - The blocked account to recover funds from.
	/// * `token` - The token contract to recover (must implement NEP-17).
	pub async fn recover_fund(
		&self,
		account: &H160,
		token: &H160,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		self.invoke_function("recoverFund", vec![account.into(), token.into()]).await
	}
}

#[async_trait]
impl<'a, P: JsonRpcProvider> SmartContractTrait<'a> for PolicyContract<'a, P> {
	type P = P;

	fn script_hash(&self) -> H160 {
		self.script_hash
	}

	fn set_script_hash(&mut self, script_hash: H160) {
		self.script_hash = script_hash;
	}

	fn provider(&self) -> Option<&RpcClient<P>> {
		self.provider
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::neo_clients::MockProvider;

	#[test]
	fn test_policy_contract_constants() {
		assert_eq!(PolicyContract::<MockProvider>::DEFAULT_EXEC_FEE_FACTOR, 30);
		assert_eq!(PolicyContract::<MockProvider>::DEFAULT_STORAGE_PRICE, 100000);
		assert_eq!(PolicyContract::<MockProvider>::DEFAULT_FEE_PER_BYTE, 1000);
		assert_eq!(PolicyContract::<MockProvider>::MAX_MILLISECONDS_PER_BLOCK, 30_000);
		assert_eq!(PolicyContract::<MockProvider>::MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT, 86400);
		assert_eq!(PolicyContract::<MockProvider>::MAX_MAX_TRACEABLE_BLOCKS, 2_102_400);
	}

	#[test]
	fn test_policy_contract_name() {
		assert_eq!(PolicyContract::<MockProvider>::NAME, "PolicyContract");
	}
}
