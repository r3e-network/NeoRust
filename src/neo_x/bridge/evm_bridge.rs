use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::sync::Arc;

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
	/// Placeholder bridge contract address on Neo X Mainnet.
	///
	/// **Important:** This must be set to the actual deployed bridge contract address
	/// before use. Use [`NeoXBridgeContractEVM::new`] with an explicit address for
	/// production deployments.
	pub const CONTRACT_ADDRESS: &'static str = "0x0000000000000000000000000000000000000000";

	/// Creates a new NeoXBridgeContractEVM instance with an explicit contract address.
	pub fn new(address: Address, provider: Arc<Provider<Http>>) -> Self {
		let contract = NeoXBridgeEVM::new(address, provider);
		Self { contract }
	}

	/// Creates an instance using the default bridge address and a provider.
	///
	/// **Warning:** The default address is a placeholder (`0x000...0`). For production
	/// use, call [`NeoXBridgeContractEVM::new`] with the actual deployed contract address.
	pub fn default_bridge(provider: Arc<Provider<Http>>) -> Self {
		let address: Address = Self::CONTRACT_ADDRESS
			.parse()
			.expect("CONTRACT_ADDRESS constant is not a valid address");
		Self::new(address, provider)
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
