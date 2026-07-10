use alloy::primitives::{Address, U256};
use alloy::providers::RootProvider;
use alloy::sol;
use std::sync::Arc;

use crate::neo_contract::ContractError;

// sol! binding for the Neo X EVM Bridge Contract.
// (alloy::sol! is the maintained successor to ethers::contract::abigen!.)
sol! {
	#[sol(rpc)]
	interface NeoXBridgeEVM {
		function withdraw(address token, uint256 amount, string memory destination) external payable;
		function getFee(address token) external view returns (uint256);
	}
}

/// A wrapper around the Neo X bridge contract on the EVM side.
/// Used to bridge assets from Neo X back to Neo N3.
pub struct NeoXBridgeContractEVM {
	contract: NeoXBridgeEVM::NeoXBridgeEVMInstance<Arc<RootProvider>>,
}

impl NeoXBridgeContractEVM {
	/// Creates a new NeoXBridgeContractEVM instance with an explicit contract address.
	pub fn new(address: Address, provider: Arc<RootProvider>) -> Self {
		let contract = NeoXBridgeEVM::new(address, provider);
		Self { contract }
	}

	/// Creates an instance using `NEOX_BRIDGE_EVM_ADDRESS`.
	pub fn default_bridge(provider: Arc<RootProvider>) -> Result<Self, ContractError> {
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
		if address == Address::ZERO {
			return Err(ContractError::InvalidArgError(
				"NEOX_BRIDGE_EVM_ADDRESS must not be the zero address".to_string(),
			));
		}
		Ok(Self::new(address, provider))
	}

	/// Returns the on-chain address of the bridge contract.
	pub fn address(&self) -> Address {
		*self.contract.address()
	}

	/// Gets the required bridge fee for a given token
	pub async fn get_fee(&self, token: Address) -> Result<U256, alloy::contract::Error> {
		self.contract.getFee(token).call().await
	}
}
