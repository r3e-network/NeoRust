use ethers::types::U256;

use crate::neo_clients::HttpProvider as N3HttpProvider;
use crate::neo_types::ScriptHash;
use crate::neo_wallets::wallet::Wallet as N3Wallet;
use crate::neo_wallets::WalletTrait;
use crate::neo_x::bridge::bridge_contract::NeoXBridgeContract;
use crate::neo_x::bridge::evm_bridge::NeoXBridgeContractEVM;
use crate::neo_x::evm::provider::NeoXProvider;
use crate::neo_x::evm::transaction::NeoXTransaction;
use crate::neo_x::evm::wallet::{NeoXClient, NeoXWallet};
use crate::sdk::Token;
use std::str::FromStr;

/// Unified Ecosystem Client that pairs a Provider with a Wallet for either Neo N3 or Neo X.
/// Designed with an ethers-rs style interface for cross-chain consistency.
pub enum EcosystemClient<'a> {
	/// N3 Network Client
	N3 { provider: crate::sdk::Neo, wallet: N3Wallet },
	/// Neo X EVM Network Client
	NeoX { client: NeoXClient<'a, N3HttpProvider> },
}

impl<'a> EcosystemClient<'a> {
	/// Creates a new N3 client
	pub fn new_n3(provider: crate::sdk::Neo, wallet: N3Wallet) -> Self {
		Self::N3 { provider, wallet }
	}

	/// Creates a new Neo X EVM client using standard RPC
	pub fn new_neox(wallet: NeoXWallet, provider: NeoXProvider<'a, N3HttpProvider>) -> Self {
		let client = NeoXClient::new(wallet, provider);
		Self::NeoX { client }
	}

	/// Connects to Neo X using an Anti-MEV configured endpoint.
	/// This guards transactions against mempool sandwich attacks and front-running.
	pub fn new_neox_anti_mev(wallet: NeoXWallet) -> Self {
		let provider = NeoXProvider::new_anti_mev(None);
		let client = NeoXClient::new(wallet, provider);
		Self::NeoX { client }
	}

	/// Gets the native gas/token balance of the configured wallet.
	pub async fn get_balance(&self) -> Result<String, String> {
		match self {
			Self::N3 { provider, wallet } => {
				let address = wallet
					.default_account()
					.ok_or("No default account")?
					.address_or_scripthash
					.address();
				let balance = provider.get_balance(&address).await.map_err(|e| e.to_string())?;
				Ok(balance.gas.to_string())
			},
			Self::NeoX { client } => {
				let bal = client.get_balance().await?;
				Ok(bal.to_string())
			},
		}
	}

	/// Transfers the native asset (GAS) from the active wallet to the target address.
	/// `amount` is specified as a string representing standard decimal formatting (or Wei for Neo X if unscaled).
	pub async fn transfer(&self, to: &str, amount: &str) -> Result<String, String> {
		match self {
			Self::N3 { provider, wallet } => {
				let amount_f64 = amount.parse::<f64>().map_err(|e| e.to_string())?;
				// Convert float GAS to integer for SDK (requires decimals logic, simple cast here)
				let amount_u64 = (amount_f64 * 100_000_000.0) as u64;
				let tx_hash = provider
					.transfer(wallet, to, amount_u64, Token::GAS)
					.await
					.map_err(|e| e.to_string())?;
				Ok(tx_hash)
			},
			Self::NeoX { client } => {
				let to_addr = primitive_types::H160::from_str(to).map_err(|e| e.to_string())?;
				// Treat amount as raw WEI for simplicity or parse appropriately
				let value = U256::from_dec_str(amount).map_err(|e| e.to_string())?;

				let tx = NeoXTransaction::new(
					Some(to_addr),
					vec![],
					value.as_u64(),
					21000,
					1000000000, // 1 Gwei
				);
				let receipt = client.send_transaction(tx).await?;
				Ok(format!("{:?}", receipt.transaction_hash))
			},
		}
	}

	/// Bridges tokens from the current chain to the other.
	/// If currently on N3, bridges to Neo X.
	/// If currently on Neo X, bridges to N3.
	pub async fn bridge_to_other_chain(
		&self,
		destination_address: &str,
		amount: &str,
	) -> Result<String, String> {
		match self {
			Self::N3 { provider, wallet } => {
				let amount_f64 = amount.parse::<f64>().map_err(|e| e.to_string())?;
				let amount_u64 = (amount_f64 * 100_000_000.0) as i64;

				let rpc_client = provider.client();
				let bridge =
					NeoXBridgeContract::new(Some(rpc_client)).map_err(|e| e.to_string())?;

				// Ensure wallet has a default account to sign with
				let account = wallet.default_account().ok_or("No default account in wallet")?;

				let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")
					.map_err(|e| e.to_string())?;

				let mut builder = bridge
					.deposit(&gas_token, amount_u64, destination_address, account)
					.await
					.map_err(|e| e.to_string())?;

				// Sign and send the N3 transaction
				let mut signed_tx = builder.sign().await.map_err(|e| e.to_string())?;
				let tx_response = signed_tx.send_tx().await.map_err(|e| e.to_string())?;
				Ok(format!("N3 -> Neo X Bridge Transaction Sent: {:?}", tx_response.hash))
			},
			Self::NeoX { client } => {
				// Initialize Neo X -> N3 Bridge using EVM bindings
				let amount_wei = U256::from_dec_str(amount).map_err(|e| e.to_string())?;
				let token_addr: ethers::types::Address =
					"0x0000000000000000000000000000000000000000".parse().unwrap();

				let evm = client.provider.evm_provider().ok_or("No EVM provider configured")?;
				let bridge = NeoXBridgeContractEVM::default_bridge(evm.clone());

				// Generate the ContractCall builder
				let call = bridge.withdraw(token_addr, amount_wei, destination_address.to_string());

				// Extract the underlying tx from `call` and map it to our unified type
				let req = call.tx;
				let data = req.data().map(|d| d.to_vec()).unwrap_or_default();
				let value = req.value().map(|v| v.as_u64()).unwrap_or_default();
				let to_addr = req.to().map(|to| match to {
					ethers::types::NameOrAddress::Address(a) => primitive_types::H160::from(a.0),
					_ => primitive_types::H160::zero(), // Name unsupported in this context
				});

				let tx = NeoXTransaction::new(
					to_addr, data, value, 200_000,    // Estimate gas limit
					1000000000, // 1 Gwei
				);

				// Send the transaction using the NeoXClient wrapper
				let receipt = client.send_transaction(tx).await?;
				Ok(format!("Neo X -> N3 Bridge Transaction Sent: {:?}", receipt.transaction_hash))
			},
		}
	}
}
