use ethers::types::U256;

use crate::neo_clients::HttpProvider as N3HttpProvider;
use crate::neo_x::evm::provider::NeoXProvider;
use crate::neo_x::evm::wallet::{NeoXClient, NeoXWallet};
use crate::neo_x::evm::transaction::NeoXTransaction;
use crate::sdk::Token;
use crate::neo_wallets::wallet::Wallet as N3Wallet;
use crate::neo_wallets::WalletTrait;
use crate::neo_x::bridge::evm_bridge::NeoXBridgeContractEVM;
use std::str::FromStr;

/// Unified Ecosystem Client that pairs a Provider with a Wallet for either Neo N3 or Neo X.
/// Designed with an ethers-rs style interface for cross-chain consistency.
pub enum EcosystemClient<'a> {
	/// N3 Network Client
	N3 {
		provider: crate::sdk::Neo,
		wallet: N3Wallet,
	},
	/// Neo X EVM Network Client
	NeoX {
		client: NeoXClient<'a, N3HttpProvider>,
	},
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
				let address = wallet.default_account().ok_or("No default account")?.address_or_scripthash.address();
				let balance = provider.get_balance(&address).await.map_err(|e| e.to_string())?;
				Ok(balance.gas.to_string())
			},
			Self::NeoX { client } => {
				let bal = client.get_balance().await?;
				Ok(bal.to_string())
			}
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
				let tx_hash = provider.transfer(wallet, to, amount_u64, Token::GAS).await.map_err(|e| e.to_string())?;
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
			}
		}
	}

	/// Bridges tokens from the current chain to the other.
	/// If currently on N3, bridges to Neo X.
	/// If currently on Neo X, bridges to N3.
	pub async fn bridge_to_other_chain(&self, destination_address: &str, amount: &str) -> Result<String, String> {
		match self {
			Self::N3 { provider: _, wallet: _ } => {
				// We currently don't expose the raw RpcClient from `Neo` directly, 
				// but in a production implementation, we would extract the client, 
				// instantiate NeoXBridgeContract, and send the TX.
				// This acts as the facade indicating how it routes.
				Err("N3 bridge execution requires transaction signer implementation mapping to be fully implemented".into())
			},
			Self::NeoX { client } => {
				// Initialize Neo X -> N3 Bridge using EVM bindings
				let amount_wei = U256::from_dec_str(amount).map_err(|e| e.to_string())?;
				let token_addr: ethers::types::Address = "0x0000000000000000000000000000000000000000".parse().unwrap();
				
				let evm = client.provider.evm_provider().ok_or("No EVM provider configured")?;
				let bridge = NeoXBridgeContractEVM::default_bridge(evm.clone());
				
				// Generate the ContractCall builder
				let _call = bridge.withdraw(token_addr, amount_wei, destination_address.to_string());
				
				// In a full implementation, we would extract the underlying tx from `call`
				// and send it via the client's wallet.
				// let tx = call.tx;
				// client.send_transaction(tx).await;
				Err("NeoX to N3 bridge method constructed but requires SignerMiddleware execution binding".into())
			}
		}
	}
}
