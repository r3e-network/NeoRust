use ethers::types::U256;

use crate::neo_clients::HttpProvider as N3HttpProvider;
use crate::neo_error::unified::{ErrorRecovery, NeoError};
use crate::neo_types::ScriptHash;
use crate::neo_wallets::wallet::Wallet as N3Wallet;
use crate::neo_wallets::WalletTrait;
use crate::neo_x::bridge::bridge_contract::NeoXBridgeContract;
use crate::neo_x::bridge::evm_bridge::NeoXBridgeContractEVM;
use crate::neo_x::evm::provider::NeoXProvider;
use crate::neo_x::evm::transaction::NeoXTransaction;
use crate::neo_x::evm::wallet::{NeoXClient, NeoXWallet};
use crate::sdk::{DecimalAmount, Token};
use std::str::FromStr;

/// Gas price for Neo X EVM transactions (1 Gwei in Wei).
const NEOX_GAS_PRICE_WEI: u64 = 1_000_000_000;

/// Gas limit for simple Neo X native token transfers.
const NEOX_TRANSFER_GAS_LIMIT: u64 = 21_000;

/// Gas limit for Neo X bridge contract interactions.
const NEOX_BRIDGE_GAS_LIMIT: u64 = 200_000;

/// Number of decimals for the GAS token on Neo N3.
const GAS_DECIMALS: u8 = 8;

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
	pub async fn get_balance(&self) -> Result<String, NeoError> {
		match self {
			Self::N3 { provider, wallet } => {
				let address = wallet
					.default_account()
					.ok_or_else(|| NeoError::Wallet {
						message: "No default account".into(),
						source: None,
						recovery: ErrorRecovery::default(),
					})?
					.address_or_scripthash
					.address();
				let balance = provider.get_balance(&address).await.map_err(|e| NeoError::Network {
					message: e.to_string(),
					source: None,
					recovery: ErrorRecovery::default(),
				})?;
				Ok(balance.gas.to_string())
			},
			Self::NeoX { client } => {
				let bal = client.get_balance().await.map_err(|e| NeoError::Network {
					message: e,
					source: None,
					recovery: ErrorRecovery::default(),
				})?;
				Ok(bal.to_string())
			},
		}
	}

	/// Transfers the native asset (GAS) from the active wallet to the target address.
	/// `amount` is specified as a string representing standard decimal formatting (or Wei for Neo X if unscaled).
	pub async fn transfer(&self, to: &str, amount: &str) -> Result<String, NeoError> {
		match self {
			Self::N3 { provider, wallet } => {
				let parsed =
					DecimalAmount::parse(amount, GAS_DECIMALS).map_err(|e| NeoError::Validation {
						message: e.to_string(),
						field: "amount".into(),
						value: Some(amount.to_string()),
						recovery: ErrorRecovery::default(),
					})?;
				let amount_u64 =
					parsed.raw().parse::<u64>().map_err(|e| NeoError::Validation {
						message: e.to_string(),
						field: "amount".into(),
						value: Some(amount.to_string()),
						recovery: ErrorRecovery::default(),
					})?;
				let tx_hash = provider
					.transfer(wallet, to, amount_u64, Token::GAS)
					.await
					.map_err(|e| NeoError::Transaction {
						message: e.to_string(),
						tx_hash: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;
				Ok(tx_hash)
			},
			Self::NeoX { client } => {
				let to_addr =
					primitive_types::H160::from_str(to).map_err(|e| NeoError::Validation {
						message: e.to_string(),
						field: "to".into(),
						value: Some(to.to_string()),
						recovery: ErrorRecovery::default(),
					})?;
				let value =
					U256::from_dec_str(amount).map_err(|e| NeoError::Validation {
						message: e.to_string(),
						field: "amount".into(),
						value: Some(amount.to_string()),
						recovery: ErrorRecovery::default(),
					})?;

				let tx = NeoXTransaction::new(
					Some(to_addr),
					vec![],
					value.as_u64(),
					NEOX_TRANSFER_GAS_LIMIT,
					NEOX_GAS_PRICE_WEI,
				);
				let receipt =
					client.send_transaction(tx).await.map_err(|e| NeoError::Transaction {
						message: e,
						tx_hash: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;
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
	) -> Result<String, NeoError> {
		match self {
			Self::N3 { provider, wallet } => {
				let parsed =
					DecimalAmount::parse(amount, GAS_DECIMALS).map_err(|e| NeoError::Validation {
						message: e.to_string(),
						field: "amount".into(),
						value: Some(amount.to_string()),
						recovery: ErrorRecovery::default(),
					})?;
				let amount_i64 = parsed.raw_i64().ok_or_else(|| NeoError::Validation {
					message: "amount overflows i64".into(),
					field: "amount".into(),
					value: Some(amount.to_string()),
					recovery: ErrorRecovery::default(),
				})?;

				let rpc_client = provider.client();
				let bridge =
					NeoXBridgeContract::new(Some(rpc_client)).map_err(|e| NeoError::Contract {
						message: e.to_string(),
						contract: Some("NeoXBridge".into()),
						method: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;

				// Ensure wallet has a default account to sign with
				let account =
					wallet.default_account().ok_or_else(|| NeoError::Wallet {
						message: "No default account in wallet".into(),
						source: None,
						recovery: ErrorRecovery::default(),
					})?;

				let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")
					.map_err(|e| NeoError::Validation {
						message: e.to_string(),
						field: "gas_token".into(),
						value: None,
						recovery: ErrorRecovery::default(),
					})?;

				let mut builder = bridge
					.deposit(&gas_token, amount_i64, destination_address, account)
					.await
					.map_err(|e| NeoError::Transaction {
						message: e.to_string(),
						tx_hash: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;

				// Sign and send the N3 transaction
				let mut signed_tx =
					builder.sign().await.map_err(|e| NeoError::Transaction {
						message: e.to_string(),
						tx_hash: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;
				let tx_response =
					signed_tx.send_tx().await.map_err(|e| NeoError::Transaction {
						message: e.to_string(),
						tx_hash: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;
				Ok(format!("N3 -> Neo X Bridge Transaction Sent: {:?}", tx_response.hash))
			},
			Self::NeoX { client } => {
				// Initialize Neo X -> N3 Bridge using EVM bindings
				let amount_wei = U256::from_dec_str(amount).map_err(|e| NeoError::Validation {
					message: e.to_string(),
					field: "amount".into(),
					value: Some(amount.to_string()),
					recovery: ErrorRecovery::default(),
				})?;
				let token_addr: ethers::types::Address = ethers::types::Address::zero();

				let evm =
					client.provider.evm_provider().ok_or_else(|| NeoError::Configuration {
						message: "No EVM provider configured".into(),
						field: Some("evm_provider".into()),
						recovery: ErrorRecovery::default(),
					})?;
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
					to_addr,
					data,
					value,
					NEOX_BRIDGE_GAS_LIMIT,
					NEOX_GAS_PRICE_WEI,
				);

				// Send the transaction using the NeoXClient wrapper
				let receipt =
					client.send_transaction(tx).await.map_err(|e| NeoError::Transaction {
						message: e,
						tx_hash: None,
						source: None,
						recovery: ErrorRecovery::default(),
					})?;
				Ok(format!("Neo X -> N3 Bridge Transaction Sent: {:?}", receipt.transaction_hash))
			},
		}
	}
}
