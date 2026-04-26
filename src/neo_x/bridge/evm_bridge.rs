use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::sync::Arc;

use crate::neo_contract::ContractError;

// Abigen binding for the Neo X EVM Bridge Contract
abigen!(
	NeoXBridgeEVM,
	r#"[
		function withdraw(address token, uint256 amount, string memory destination) public payable
		function getFee(address token) public view returns (uint256)
	]"#
);

/// A wrapper around the Neo X bridge contract on the EVM side.
/// Used to bridge assets from Neo X back to Neo N3.
pub struct NeoXBridgeContractEVM {
	contract: NeoXBridgeEVM<Provider<Http>>,
}

impl NeoXBridgeContractEVM {
	/// Creates a new NeoXBridgeContractEVM instance with an explicit contract address.
	pub fn new(address: Address, provider: Arc<Provider<Http>>) -> Self {
		let contract = NeoXBridgeEVM::new(address, provider);
		Self { contract }
	}

	/// Creates an instance using `NEOX_BRIDGE_EVM_ADDRESS`.
	pub fn default_bridge(provider: Arc<Provider<Http>>) -> Result<Self, ContractError> {
		let configured = std::env::var("NEOX_BRIDGE_EVM_ADDRESS").map_err(|_| {
			ContractError::InvalidStateError(
				"NEOX_BRIDGE_EVM_ADDRESS must be set to the deployed Neo X bridge contract address"
					.to_string(),
			)
		})?;
		let address: Address = configured.parse().map_err(|e| {
			ContractError::InvalidArgError(format!(
				"Invalid NEOX_BRIDGE_EVM_ADDRESS '{}': {}",
				configured, e
			))
		})?;
		if address == Address::zero() {
			return Err(ContractError::InvalidArgError(
				"NEOX_BRIDGE_EVM_ADDRESS must not be the zero address".to_string(),
			));
		}
		Ok(Self::new(address, provider))
	}

	/// Builds a transaction to withdraw tokens from Neo X back to Neo N3.
	/// NOTE: This returns the method builder, which you can use to send or estimate gas.
	pub fn withdraw(
		&self,
		token: Address,
		amount: U256,
		destination_n3_address: String,
	) -> ethers::contract::ContractCall<Provider<Http>, ()> {
		self.contract.withdraw(token, amount, destination_n3_address)
	}

	/// Gets the required bridge fee for a given token
	pub async fn get_fee(
		&self,
		token: Address,
	) -> Result<U256, ethers::contract::ContractError<Provider<Http>>> {
		self.contract.get_fee(token).call().await
	}
}
