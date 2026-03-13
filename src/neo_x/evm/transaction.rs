use ethers::types::{TransactionRequest, H160 as EthersH160, U256};
use primitive_types::H160;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

	/// Converts this transaction into an ethers-rs `TransactionRequest`.
	/// This makes it simple to submit the transaction via an Ethers provider.
	pub fn into_ethers_request(self) -> TransactionRequest {
		let mut req = TransactionRequest::new()
			.data(self.data)
			.value(U256::from(self.value))
			.gas(U256::from(self.gas_limit))
			.gas_price(U256::from(self.gas_price));
		
		if let Some(to) = self.to {
			// Convert primitive_types::H160 to ethers::types::H160
			let to_ethers = EthersH160::from_str(&format!("{:?}", to)).unwrap_or_default();
			req = req.to(to_ethers);
		}
		
		req
	}
}

impl From<NeoXTransaction> for TransactionRequest {
	fn from(tx: NeoXTransaction) -> Self {
		tx.into_ethers_request()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_into_ethers_request() {
		let tx = NeoXTransaction::new(
			None,
			vec![1, 2, 3],
			1000,
			21000,
			1000000000,
		);
		
		let req = tx.into_ethers_request();
		assert!(req.to.is_none());
		assert_eq!(req.value.unwrap(), U256::from(1000));
		assert_eq!(req.gas.unwrap(), U256::from(21000));
		assert_eq!(req.gas_price.unwrap(), U256::from(1000000000));
	}
}
