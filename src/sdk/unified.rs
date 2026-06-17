use alloy::primitives::{Address as AlloyAddress, U256};
use alloy::sol_types::SolCall;

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

/// Build a [`NeoError::Validation`] for a malformed amount field.
fn amount_validation_error<E: std::fmt::Display>(amount: &str, err: E) -> NeoError {
	NeoError::Validation {
		message: err.to_string(),
		field: "amount".into(),
		value: Some(amount.to_string()),
		recovery: ErrorRecovery::new()
			.suggest("Provide a non-negative decimal value")
			.suggest("Check the token's decimals scale"),
	}
}

/// Build a [`NeoError::Wallet`] for the "wallet has no default account" condition.
fn no_default_account_error() -> NeoError {
	NeoError::Wallet {
		message: "No default account in wallet".into(),
		source: None,
		recovery: ErrorRecovery::new()
			.suggest("Set a default account via Wallet::set_default_account")
			.suggest("Add an account to the wallet first"),
	}
}

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
	N3 {
		/// High-level Neo N3 RPC + transaction client.
		provider: crate::sdk::Neo,
		/// Wallet holding the signing key for outbound N3 transactions.
		wallet: N3Wallet,
	},
	/// Neo X EVM Network Client
	NeoX {
		/// EVM client paired with a Neo X wallet.
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
	pub async fn get_balance(&self) -> Result<String, NeoError> {
		match self {
			Self::N3 { provider, wallet } => {
				let address = wallet
					.default_account()
					.ok_or_else(no_default_account_error)?
					.address_or_scripthash
					.address();
				let balance = provider
					.get_balance(&address)
					.await
					.map_err(|e| NeoError::network("Neo X balance lookup", e))?;
				Ok(balance.gas.to_string())
			},
			Self::NeoX { client } => {
				let bal = client
					.get_balance()
					.await
					.map_err(|e| NeoError::network("Neo X balance lookup", e))?;
				Ok(bal.to_string())
			},
		}
	}

	/// Transfers the native asset (GAS) from the active wallet to the target address.
	/// `amount` is specified as a string representing standard decimal formatting (or Wei for Neo X if unscaled).
	pub async fn transfer(&self, to: &str, amount: &str) -> Result<String, NeoError> {
		match self {
			Self::N3 { provider, wallet } => {
				let parsed = DecimalAmount::parse(amount, GAS_DECIMALS)
					.map_err(|e| amount_validation_error(amount, e))?;
				let amount_u64 =
					parsed.raw().parse::<u64>().map_err(|e| amount_validation_error(amount, e))?;
				let tx_hash = provider
					.transfer(wallet, to, amount_u64, Token::GAS)
					.await
					.map_err(|e| NeoError::transaction("N3 transfer failed", e))?;
				Ok(tx_hash)
			},
			Self::NeoX { client } => {
				let to_addr = primitive_types::H160::from_str(to).map_err(|e| {
					NeoError::validation("to", Some(to.to_string()), &e.to_string())
				})?;
				let value =
					U256::from_str(amount).map_err(|e| amount_validation_error(amount, e))?;

				let tx = NeoXTransaction::new(
					Some(to_addr),
					vec![],
					value.to::<u64>(),
					NEOX_TRANSFER_GAS_LIMIT,
					NEOX_GAS_PRICE_WEI,
				);
				let receipt = client
					.send_transaction(tx)
					.await
					.map_err(|e| NeoError::transaction("Neo X transfer failed", e))?;
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
				let parsed = DecimalAmount::parse(amount, GAS_DECIMALS)
					.map_err(|e| amount_validation_error(amount, e))?;
				let amount_i64 = parsed
					.raw_i64()
					.ok_or_else(|| amount_validation_error(amount, "amount overflows i64"))?;

				let rpc_client = provider.client();
				let bridge = NeoXBridgeContract::new(Some(rpc_client)).map_err(|e| {
					NeoError::contract(
						"Failed to bind NeoX bridge contract",
						Some("NeoXBridge".into()),
						None,
						e,
					)
				})?;

				let account = wallet.default_account().ok_or_else(no_default_account_error)?;

				let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")
					.map_err(|e| {
						NeoError::validation("gas_token", None::<String>, &e.to_string())
					})?;

				let mut builder = bridge
					.deposit(&gas_token, amount_i64, destination_address, account)
					.await
					.map_err(|e| NeoError::transaction("NeoX bridge deposit", e))?;

				let mut signed_tx = builder
					.sign()
					.await
					.map_err(|e| NeoError::transaction("Failed to sign bridge tx", e))?;
				let tx_response = signed_tx
					.send_tx()
					.await
					.map_err(|e| NeoError::transaction("Failed to send bridge tx", e))?;
				Ok(format!("N3 -> Neo X Bridge Transaction Sent: {:?}", tx_response.hash))
			},
			Self::NeoX { client } => {
				let amount_wei =
					U256::from_str(amount).map_err(|e| amount_validation_error(amount, e))?;
				let token_addr = AlloyAddress::ZERO;

				let evm =
					client.provider.evm_provider().ok_or_else(|| NeoError::Configuration {
						message: "No EVM provider configured".into(),
						field: Some("evm_provider".into()),
						recovery: ErrorRecovery::new()
							.suggest("Configure the EVM provider when constructing NeoXProvider"),
					})?;
				let bridge = NeoXBridgeContractEVM::default_bridge(evm.clone()).map_err(|e| {
					NeoError::contract(
						"Failed to bind NeoX EVM bridge",
						Some("NeoXBridgeEVM".into()),
						Some("default_bridge".into()),
						e,
					)
				})?;

				// ABI-encode the withdraw calldata without broadcasting, then wrap it
				// in a NeoXTransaction addressed to the bridge contract.
				let withdraw_call = crate::neo_x::bridge::evm_bridge::NeoXBridgeEVM::withdrawCall {
					token: token_addr,
					amount: amount_wei,
					destination: destination_address.to_string(),
				};
				let data = withdraw_call.abi_encode();
				let to_addr = primitive_types::H160::from(bridge.address().into_array());

				let tx = NeoXTransaction::new(
					Some(to_addr),
					data,
					0,
					NEOX_BRIDGE_GAS_LIMIT,
					NEOX_GAS_PRICE_WEI,
				);

				let receipt = client
					.send_transaction(tx)
					.await
					.map_err(|e| NeoError::transaction("Failed to send Neo X bridge tx", e))?;
				Ok(format!("Neo X -> N3 Bridge Transaction Sent: {:?}", receipt.transaction_hash))
			},
		}
	}
}
