use alloy::primitives::{Address as AlloyAddress, U256 as AlloyU256};
use alloy::rpc::types::eth::TransactionRequest;
use primitive_types::H160;
use serde::{Deserialize, Serialize};
/// Neo X EVM transaction for interacting with the Neo X EVM-compatible chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoXTransaction {
	to: Option<H160>,
	data: Vec<u8>,
	value: u64,
	gas_limit: u64,
	gas_price: u64,
}

impl NeoXTransaction {
	/// Creates a new NeoXTransaction instance
	///
	/// # Arguments
	///
	/// * `to` - The recipient address (None for contract creation)
	/// * `data` - The transaction data
	/// * `value` - The transaction value
	/// * `gas_limit` - The gas limit for the transaction
	/// * `gas_price` - The gas price for the transaction
	///
	/// # Returns
	///
	/// A new NeoXTransaction instance
	pub fn new(
		to: Option<H160>,
		data: Vec<u8>,
		value: u64,
		gas_limit: u64,
		gas_price: u64,
	) -> Self {
		Self { to, data, value, gas_limit, gas_price }
	}

	/// Gets the recipient address
	pub fn to(&self) -> Option<H160> {
		self.to
	}

	/// Gets the transaction data
	pub fn data(&self) -> &Vec<u8> {
		&self.data
	}

	/// Gets the transaction value
	pub fn value(&self) -> u64 {
		self.value
	}

	/// Gets the gas limit for the transaction
	pub fn gas_limit(&self) -> u64 {
		self.gas_limit
	}

	/// Gets the gas price for the transaction
	pub fn gas_price(&self) -> u64 {
		self.gas_price
	}

	/// Converts this transaction into an alloy `TransactionRequest`.
	/// This makes it simple to submit the transaction via an alloy provider.
	pub fn into_alloy_request(self) -> TransactionRequest {
		Self::build_alloy_request(
			self.to,
			self.data,
			AlloyU256::from(self.value),
			self.gas_limit,
			self.gas_price,
		)
	}

	/// Builds an Alloy transaction request with a full-width EVM value.
	///
	/// This additive path preserves the legacy [`NeoXTransaction`] serialization and accessors
	/// while allowing callers to transfer values above [`u64::MAX`].
	pub fn build_alloy_request(
		to: Option<H160>,
		data: Vec<u8>,
		value: AlloyU256,
		gas_limit: u64,
		gas_price: u64,
	) -> TransactionRequest {
		TransactionRequest {
			to: to.map(|hash| alloy::primitives::TxKind::Call(AlloyAddress::from(hash.0))),
			input: data.into(),
			value: Some(value),
			gas: Some(gas_limit),
			max_fee_per_gas: Some(gas_price as u128),
			..Default::default()
		}
	}
}

impl From<NeoXTransaction> for TransactionRequest {
	fn from(tx: NeoXTransaction) -> Self {
		tx.into_alloy_request()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_into_alloy_request() {
		let tx = NeoXTransaction::new(None, vec![1, 2, 3], 1000, 21000, 1_000_000_000);
		let legacy_value: u64 = tx.value();
		let serialized = serde_json::to_value(&tx).unwrap();
		assert_eq!(legacy_value, 1000);
		assert_eq!(serialized["value"], serde_json::json!(1000));
		assert_eq!(serde_json::from_value::<NeoXTransaction>(serialized).unwrap().value(), 1000);

		let req = tx.into_alloy_request();
		assert!(matches!(req.to, None | Some(alloy::primitives::TxKind::Create)));
		assert_eq!(req.value, Some(AlloyU256::from(1000)));
		assert_eq!(req.gas, Some(21000));
		assert_eq!(req.max_fee_per_gas, Some(1_000_000_000));
	}

	#[test]
	fn preserves_values_above_u64() {
		let value = AlloyU256::from(u64::MAX) + AlloyU256::from(1_u8);
		let request =
			NeoXTransaction::build_alloy_request(None, Vec::new(), value, 21_000, 1_000_000_000);

		assert_eq!(request.value, Some(value));
	}
}
