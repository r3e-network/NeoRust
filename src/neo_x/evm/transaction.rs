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
		TransactionRequest {
			to: self.to.map(|h| alloy::primitives::TxKind::Call(AlloyAddress::from(h.0))),
			input: self.data.into(),
			value: Some(AlloyU256::from(self.value)),
			gas: Some(self.gas_limit),
			max_fee_per_gas: Some(self.gas_price as u128),
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

		let req = tx.into_alloy_request();
		assert!(matches!(req.to, None | Some(alloy::primitives::TxKind::Create)));
		assert_eq!(req.value, Some(AlloyU256::from(1000)));
		assert_eq!(req.gas, Some(21000));
		assert_eq!(req.max_fee_per_gas, Some(1_000_000_000));
	}
}
