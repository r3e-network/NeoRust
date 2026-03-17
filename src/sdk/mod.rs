//! High-level SDK API for simplified Neo blockchain interaction
//!
//! This module provides a user-friendly interface that wraps the lower-level
//! components, making common operations simple while still allowing access
//! to advanced features when needed.

pub mod hd_wallet;
pub mod transaction_simulator;
pub mod unified;
#[cfg(feature = "ws")]
pub mod websocket;

use crate::{
	neo_clients::{APITrait, HttpProvider, RpcCache, RpcClient},
	neo_error::unified::{ErrorRecovery, NeoError},
	neo_types::{ContractParameter, ScriptHash, ScriptHashExtension, StackItem},
	neo_wallets::wallet::Wallet,
};
use hex_literal::hex;
use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

/// Main entry point for the Neo SDK
///
/// Provides high-level, user-friendly methods for common blockchain operations
/// while maintaining access to lower-level APIs when needed.
///
/// # Examples
///
/// ```no_run
/// use neo3::sdk::Neo;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Quick connection to testnet
///     let neo = Neo::testnet().await?;
///     
///     // Check balance
///     let balance = neo.get_balance("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc").await?;
///     println!("Balance: {} GAS", balance.gas);
///     
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Neo {
	client: Arc<RpcClient<HttpProvider>>,
	network: Network,
	endpoint: String,
	cache: Option<RpcCache>,
	config: SdkConfig,
}

/// Network configuration
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Network {
	/// Neo MainNet
	MainNet,
	/// Neo TestNet
	TestNet,
	/// Custom network with RPC endpoint
	Custom(String),
}

/// SDK configuration options
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SdkConfig {
	/// Request timeout
	pub timeout: Duration,
	/// Number of retries for failed requests
	pub retries: u32,
	/// Enable caching
	pub cache_enabled: bool,
	/// Enable metrics collection
	pub metrics_enabled: bool,
}

impl SdkConfig {
	/// Creates a new builder for the configuration
	pub fn builder() -> SdkConfigBuilder {
		SdkConfigBuilder::default()
	}
}

/// Builder for `SdkConfig`
#[derive(Debug, Default, Clone)]
pub struct SdkConfigBuilder {
	timeout: Option<Duration>,
	retries: Option<u32>,
	cache_enabled: Option<bool>,
	metrics_enabled: Option<bool>,
}

impl SdkConfigBuilder {
	/// Sets the request timeout
	pub fn timeout(mut self, val: Duration) -> Self {
		self.timeout = Some(val);
		self
	}

	/// Sets the number of retries
	pub fn retries(mut self, val: u32) -> Self {
		self.retries = Some(val);
		self
	}

	/// Enables or disables caching
	pub fn cache_enabled(mut self, val: bool) -> Self {
		self.cache_enabled = Some(val);
		self
	}

	/// Enables or disables metrics
	pub fn metrics_enabled(mut self, val: bool) -> Self {
		self.metrics_enabled = Some(val);
		self
	}

	/// Builds the `SdkConfig`
	pub fn build(self) -> SdkConfig {
		let default = SdkConfig::default();
		SdkConfig {
			timeout: self.timeout.unwrap_or(default.timeout),
			retries: self.retries.unwrap_or(default.retries),
			cache_enabled: self.cache_enabled.unwrap_or(default.cache_enabled),
			metrics_enabled: self.metrics_enabled.unwrap_or(default.metrics_enabled),
		}
	}
}

impl Default for SdkConfig {
	fn default() -> Self {
		Self {
			timeout: Duration::from_secs(30),
			retries: 3,
			cache_enabled: true,
			metrics_enabled: false,
		}
	}
}

/// Exact token amount represented as a non-negative integer with an implied decimal scale.
///
/// Neo N3 NEP-17 balances are returned as raw integers (base units). This type keeps the raw
/// amount exact and provides safe, deterministic formatting without floating-point rounding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecimalAmount {
	raw: String,
	decimals: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalAmountParseError {
	Empty,
	NegativeNotAllowed,
	InvalidFormat,
	InvalidCharacter,
	TooManyFractionalDigits { provided: usize, allowed: u8 },
}

impl fmt::Display for DecimalAmountParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty => f.write_str("amount is empty"),
			Self::NegativeNotAllowed => f.write_str("amount must be non-negative"),
			Self::InvalidFormat => f.write_str("invalid amount format"),
			Self::InvalidCharacter => f.write_str("amount contains invalid characters"),
			Self::TooManyFractionalDigits { provided, allowed } => {
				write!(f, "too many fractional digits: provided {}, allowed {}", provided, allowed)
			},
		}
	}
}

impl std::error::Error for DecimalAmountParseError {}

impl DecimalAmount {
	/// Create an amount from a raw (base-unit) integer string and a decimal scale.
	///
	/// `raw` must be a non-negative base-10 integer. Leading zeros are normalized.
	pub fn from_raw(raw: impl Into<String>, decimals: u8) -> Self {
		let raw = raw.into();
		let raw = raw.trim();
		let raw = raw.trim_start_matches('+');

		let raw = if raw.chars().all(|c| c.is_ascii_digit()) {
			let raw = raw.trim_start_matches('0');
			if raw.is_empty() {
				"0".to_string()
			} else {
				raw.to_string()
			}
		} else {
			"0".to_string()
		};

		Self { raw, decimals }
	}

	/// Parse a human-formatted decimal string into base units with a fixed `decimals` scale.
	///
	/// Examples:
	/// - `decimals = 8`, `"50.5"` -> raw `"5050000000"`
	/// - `decimals = 0`, `"10"` -> raw `"10"`
	pub fn parse(amount: &str, decimals: u8) -> Result<Self, DecimalAmountParseError> {
		let amount = amount.trim();
		if amount.is_empty() {
			return Err(DecimalAmountParseError::Empty);
		}
		if amount.starts_with('-') {
			return Err(DecimalAmountParseError::NegativeNotAllowed);
		}
		if !amount.chars().any(|c| c.is_ascii_digit()) {
			return Err(DecimalAmountParseError::InvalidFormat);
		}

		let mut iter = amount.split('.');
		let whole = iter.next().unwrap_or("");
		let frac = iter.next();
		if iter.next().is_some() {
			return Err(DecimalAmountParseError::InvalidFormat);
		}

		let whole = if whole.is_empty() { "0" } else { whole };
		let frac = frac.unwrap_or("");

		if !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) {
			return Err(DecimalAmountParseError::InvalidCharacter);
		}

		let allowed = decimals as usize;
		let mut frac = frac.to_string();
		if frac.len() > allowed {
			let (kept, extra) = frac.split_at(allowed);
			if extra.chars().all(|c| c == '0') {
				frac = kept.to_string();
			} else {
				return Err(DecimalAmountParseError::TooManyFractionalDigits {
					provided: frac.len(),
					allowed: decimals,
				});
			}
		}
		while frac.len() < allowed {
			frac.push('0');
		}

		let whole = whole.trim_start_matches('0');
		let whole = if whole.is_empty() { "0" } else { whole };
		let raw = format!("{}{}", whole, frac);
		Ok(Self::from_raw(raw, decimals))
	}

	/// Raw base-unit integer as a base-10 string.
	pub fn raw(&self) -> &str {
		&self.raw
	}

	/// Decimal scale (number of fractional digits).
	pub fn decimals(&self) -> u8 {
		self.decimals
	}

	/// Format as a fixed-scale decimal string (always prints exactly `decimals` fractional digits).
	pub fn to_fixed_string(&self) -> String {
		let decimals = self.decimals as usize;
		if decimals == 0 {
			return self.raw.clone();
		}

		let raw = self.raw.as_str();
		let len = raw.len();
		if len > decimals {
			let (int_part, frac_part) = raw.split_at(len - decimals);
			format!("{}.{}", int_part, frac_part)
		} else {
			let zeros = "0".repeat(decimals - len);
			format!("0.{}{}", zeros, raw)
		}
	}

	/// Convert raw base-units to `i64` when it fits (useful for Neo VM integers).
	pub fn raw_i64(&self) -> Option<i64> {
		self.raw.parse::<i64>().ok()
	}
}

impl fmt::Display for DecimalAmount {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.to_fixed_string())
	}
}

/// Balance information for an address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
	/// NEO token balance
	pub neo: u64,
	/// GAS token balance (exact; 8 decimals)
	pub gas: DecimalAmount,
	/// Other NEP-17 token balances
	pub tokens: Vec<TokenBalance>,
}

/// Individual token balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
	/// Token contract hash
	pub contract: ScriptHash,
	/// Token symbol
	pub symbol: String,
	/// Token amount (exact; uses the token's on-chain decimals)
	pub amount: DecimalAmount,
}

/// Transaction hash type
pub type TxHash = String;

/// Common tokens
#[derive(Debug, Clone)]
pub enum Token {
	/// Native NEO token
	NEO,
	/// Native GAS token
	GAS,
	/// Custom NEP-17 token
	Custom(ScriptHash),
}

impl Neo {
	/// Connect to Neo TestNet with default configuration
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::sdk::Neo;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn testnet() -> Result<Self, NeoError> {
		Self::builder().network(Network::TestNet).build().await
	}

	/// Connect to Neo MainNet with default configuration
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::sdk::Neo;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::mainnet().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn mainnet() -> Result<Self, NeoError> {
		Self::builder().network(Network::MainNet).build().await
	}

	/// Create a new SDK builder for custom configuration
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::sdk::{Neo, Network};
	/// use std::time::Duration;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::builder()
	///     .network(Network::TestNet)
	///     .timeout(Duration::from_secs(60))
	///     .retries(5)
	///     .build()
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub fn builder() -> NeoBuilder {
		NeoBuilder::default()
	}

	/// Get the balance of an address
	///
	/// Returns NEO, GAS, and all NEP-17 token balances.
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::sdk::Neo;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// let balance = neo.get_balance("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc").await?;
	/// println!("NEO: {}, GAS: {}", balance.neo, balance.gas);
	/// # Ok(())
	/// # }
	/// ```
	pub async fn get_balance(&self, address: &str) -> Result<Balance, NeoError> {
		use crate::neo_types::ScriptHashExtension;

		// Parse the address to script hash
		let script_hash = ScriptHash::from_address(address).map_err(|e| NeoError::Validation {
			message: format!("Invalid address: {}", e),
			field: "address".to_string(),
			value: Some(address.to_string()),
			recovery: crate::neo_error::unified::ErrorRecovery::new()
				.suggest("Check the address format")
				.suggest("Ensure it's a valid Neo N3 address"),
		})?;

		// Serve cached balance when enabled.
		if let Some(cache) = &self.cache {
			let cache_key = format!("balance:{}", address);
			if let Some(cached) = cache.get(&cache_key).await {
				if let Ok(balance) = serde_json::from_value::<Balance>(cached) {
					return Ok(balance);
				}
			}
		}

		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let retry_delay = Duration::from_millis(250);

		// Get NEO balance
		let neo_hash = ScriptHash::from(hex!("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"));
		let neo_balance = crate::neo_utils::error::retry(
			|| async {
				self.client
					.invoke_function(
						&neo_hash,
						"balanceOf".to_string(),
						vec![ContractParameter::h160(&script_hash)],
						None,
					)
					.await
			},
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to get NEO balance: {}", e),
			source: None,
			recovery: crate::neo_error::unified::ErrorRecovery::new()
				.suggest("Check network connection")
				.retryable(true),
		})?;

		// Get GAS balance
		let gas_hash = ScriptHash::from(hex!("d2a4cff31913016155e38e474a2c06d08be276cf"));
		let gas_balance = crate::neo_utils::error::retry(
			|| async {
				self.client
					.invoke_function(
						&gas_hash,
						"balanceOf".to_string(),
						vec![ContractParameter::h160(&script_hash)],
						None,
					)
					.await
			},
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to get GAS balance: {}", e),
			source: None,
			recovery: crate::neo_error::unified::ErrorRecovery::new()
				.suggest("Check network connection")
				.retryable(true),
		})?;

		let neo = neo_balance
			.stack
			.first()
			.ok_or_else(|| invalid_balance_response("NEO", "missing stack item"))?;
		let neo = parse_balance_stack_item_u64(neo, "NEO")?;

		let gas_raw = gas_balance
			.stack
			.first()
			.ok_or_else(|| invalid_balance_response("GAS", "missing stack item"))?;
		let gas_raw = parse_balance_stack_item_u64(gas_raw, "GAS")?;
		let gas = DecimalAmount::from_raw(gas_raw.to_string(), 8); // GAS has 8 decimals

		// Get other NEP-17 token balances
		let nep17 = crate::neo_utils::error::retry(
			|| async { self.client.get_nep17_balances(script_hash).await },
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to get NEP-17 balances: {}", e),
			source: None,
			recovery: crate::neo_error::unified::ErrorRecovery::new()
				.suggest("Check network connection")
				.retryable(true),
		})?;

		let tokens = nep17
			.balances
			.into_iter()
			.filter(|b| b.asset_hash != neo_hash && b.asset_hash != gas_hash)
			.map(|b| {
				let decimals = parse_nep17_decimals(b.decimals.as_deref(), &b.asset_hash)?;
				let amount = DecimalAmount::from_raw(b.amount, decimals);

				Ok(TokenBalance {
					contract: b.asset_hash,
					symbol: b
						.symbol
						.clone()
						.or_else(|| b.name.clone())
						.unwrap_or_else(|| b.asset_hash.to_hex()),
					amount,
				})
			})
			.collect::<Result<Vec<_>, NeoError>>()?;

		let balance = Balance { neo, gas, tokens };

		// Cache for subsequent reads (short TTL; balances change frequently).
		if let Some(cache) = &self.cache {
			if let Ok(value) = serde_json::to_value(&balance) {
				cache.cache_balance(address.to_string(), value).await;
			}
		}

		Ok(balance)
	}

	/// Transfer tokens from one address to another
	///
	/// Handles all the complexity of building, signing, and sending the transaction.
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::neo_wallets::wallet::Wallet;
	/// use neo3::sdk::{Neo, Token};
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// let wallet = Wallet::new();
	/// let tx_hash = neo
	///     .transfer(
	///         &wallet,
	///         "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc",
	///         100,
	///         Token::GAS,
	///     )
	///     .await?;
	/// println!("Transaction sent: {}", tx_hash);
	/// # Ok(())
	/// # }
	/// ```
	pub async fn transfer(
		&self,
		from: &Wallet,
		to: &str,
		amount: u64,
		token: Token,
	) -> Result<TxHash, NeoError> {
		use crate::neo_builder::{AccountSigner, CallFlags, ScriptBuilder, TransactionBuilder};
		use crate::neo_error::unified::ErrorRecovery;
		use crate::neo_types::ScriptHashExtension;
		use crate::neo_wallets::WalletTrait;

		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let retry_delay = Duration::from_millis(250);

		let from_account = from.default_account().ok_or_else(|| NeoError::Wallet {
			message: "No default account set in wallet".to_string(),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Set a default account using set_default_account()")
				.suggest("Add an account to the wallet first"),
		})?;

		let to_hash = ScriptHash::from_address(to).map_err(|e| NeoError::Validation {
			message: format!("Invalid recipient address: {}", e),
			field: "to".to_string(),
			value: Some(to.to_string()),
			recovery: ErrorRecovery::new()
				.suggest("Check the address format")
				.suggest("Ensure it's a valid Neo N3 address"),
		})?;

		let contract_hash = match token {
			Token::NEO => ScriptHash::from(hex!("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")),
			Token::GAS => ScriptHash::from(hex!("d2a4cff31913016155e38e474a2c06d08be276cf")),
			Token::Custom(hash) => hash,
		};

		let amount_i64 = i64::try_from(amount).map_err(|_| NeoError::Validation {
			message: "Amount is too large to fit Neo VM integer".to_string(),
			field: "amount".to_string(),
			value: Some(amount.to_string()),
			recovery: ErrorRecovery::new()
				.suggest("Use a smaller amount")
				.suggest("Split into multiple transfers if needed"),
		})?;

		let mut sb = ScriptBuilder::new();
		sb.contract_call(
			&contract_hash,
			"transfer",
			&[
				ContractParameter::h160(&from_account.get_script_hash()),
				ContractParameter::h160(&to_hash),
				ContractParameter::integer(amount_i64),
				ContractParameter::any(),
			],
			Some(CallFlags::All),
		)
		.map_err(|e| NeoError::Contract {
			message: format!("Failed to build transfer script: {}", e),
			contract: Some(contract_hash.to_hex()),
			method: Some("transfer".to_string()),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check token contract hash")
				.suggest("Check transfer parameters"),
		})?;

		let signer =
			AccountSigner::called_by_entry(from_account).map_err(|e| NeoError::Transaction {
				message: format!("Failed to create signer: {}", e),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Ensure the wallet has an unlocked private key")
					.suggest("Multi-sig accounts require manual signing"),
			})?;

		let current_height = crate::neo_utils::error::retry(
			|| async { self.client.get_block_count().await },
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to fetch current block height: {}", e),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check network connection").retryable(true),
		})?;

		let mut tb = TransactionBuilder::with_client(self.client.as_ref());
		tb.extend_script(sb.to_bytes());
		tb.set_signers(vec![signer.into()]).map_err(|e| NeoError::Transaction {
			message: format!("Failed to set signers: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new().suggest("Ensure signer configuration is valid"),
		})?;
		tb.valid_until_block(current_height + 5760).map_err(|e| NeoError::Transaction {
			message: format!("Invalid valid-until-block: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new().suggest("Check network height").retryable(true),
		})?;

		let mut tx = tb.sign().await.map_err(|e| NeoError::Transaction {
			message: format!("Failed to sign transfer: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Ensure the wallet has the correct private key")
				.suggest("Check signer scopes"),
		})?;

		let result = {
			let mut attempt = 0usize;
			loop {
				attempt += 1;
				match tx.send_tx().await {
					Ok(result) => break Ok(result),
					Err(err) if attempt < max_attempts => {
						tracing::warn!(
							attempt = attempt,
							error = %err,
							"Send transaction failed; retrying"
						);
						tokio::time::sleep(retry_delay).await;
					},
					Err(err) => break Err(err),
				}
			}
		}
		.map_err(|e| NeoError::Transaction {
			message: format!("Failed to send transfer transaction: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check RPC endpoint availability")
				.retryable(true),
		})?;

		Ok(result.hash.to_string())
	}

	/// Deploy a smart contract
	///
	/// Simplifies the contract deployment process.
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::neo_wallets::wallet::Wallet;
	/// use neo3::sdk::Neo;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// let wallet = Wallet::new();
	/// let nef_bytes = Vec::new();
	/// let manifest = "{}".to_string();
	///
	/// let contract_hash = neo.deploy_contract(&wallet, nef_bytes, manifest).await?;
	/// println!("Contract deployed: {}", contract_hash);
	/// # Ok(())
	/// # }
	/// ```
	pub async fn deploy_contract(
		&self,
		deployer: &Wallet,
		nef: Vec<u8>,
		manifest: String,
	) -> Result<ScriptHash, NeoError> {
		use crate::neo_builder::{AccountSigner, CallFlags, ScriptBuilder, TransactionBuilder};
		use crate::neo_error::unified::ErrorRecovery;
		use crate::neo_types::{ContractManifest, NefFile, ScriptHashExtension};
		use crate::neo_wallets::WalletTrait;

		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let retry_delay = Duration::from_millis(250);

		let deployer_account = deployer.default_account().ok_or_else(|| NeoError::Wallet {
			message: "No default account set in deployer wallet".to_string(),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Set a default account using set_default_account()")
				.suggest("Add an account to the wallet first"),
		})?;

		let nef_file = NefFile::deserialize(&nef).map_err(|e| NeoError::Validation {
			message: format!("Invalid NEF file: {}", e),
			field: "nef".to_string(),
			value: None,
			recovery: ErrorRecovery::new()
				.suggest("Ensure the NEF bytes are valid")
				.suggest("Load the NEF file using std::fs::read"),
		})?;

		let manifest_bytes = manifest.as_bytes().to_vec();
		let manifest_struct: ContractManifest =
			serde_json::from_str(&manifest).map_err(|e| NeoError::Validation {
				message: format!("Invalid manifest JSON: {}", e),
				field: "manifest".to_string(),
				value: None,
				recovery: ErrorRecovery::new()
					.suggest("Ensure the manifest is valid JSON")
					.suggest("Provide the full contract manifest"),
			})?;

		let contract_name = manifest_struct.name.clone().ok_or_else(|| NeoError::Validation {
			message: "Manifest is missing contract name".to_string(),
			field: "manifest.name".to_string(),
			value: None,
			recovery: ErrorRecovery::new()
				.suggest("Include a `name` field in the manifest")
				.suggest("Check your contract compiler output"),
		})?;

		let nef_checksum = {
			if nef_file.checksum.len() != 4 {
				return Err(NeoError::Validation {
					message: "NEF checksum length is invalid".to_string(),
					field: "nef.checksum".to_string(),
					value: None,
					recovery: ErrorRecovery::new().suggest("Provide a valid NEF file"),
				});
			}
			let mut arr = [0u8; 4];
			arr.copy_from_slice(&nef_file.checksum);
			arr.reverse();
			u32::from_be_bytes(arr)
		};

		let contract_script = ScriptBuilder::build_contract_script(
			&deployer_account.get_script_hash(),
			nef_checksum,
			&contract_name,
		)
		.map_err(|e| NeoError::Contract {
			message: format!("Failed to derive contract script: {}", e),
			contract: None,
			method: Some("deploy".to_string()),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check NEF checksum and manifest name"),
		})?;
		let expected_hash = ScriptHash::from_script(&contract_script);

		let management_hash = ScriptHash::from(hex!("fffdc93764dbaddd97c48f252a53ea4643faa3fd"));
		let mut sb = ScriptBuilder::new();
		sb.contract_call(
			&management_hash,
			"deploy",
			&[
				ContractParameter::byte_array(nef),
				ContractParameter::byte_array(manifest_bytes),
				ContractParameter::any(),
			],
			Some(CallFlags::All),
		)
		.map_err(|e| NeoError::Contract {
			message: format!("Failed to build deploy script: {}", e),
			contract: Some(management_hash.to_hex()),
			method: Some("deploy".to_string()),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check NEF and manifest parameters"),
		})?;

		let signer = AccountSigner::called_by_entry(deployer_account).map_err(|e| {
			NeoError::Transaction {
				message: format!("Failed to create signer: {}", e),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Ensure the wallet has an unlocked private key"),
			}
		})?;

		let current_height = crate::neo_utils::error::retry(
			|| async { self.client.get_block_count().await },
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to fetch current block height: {}", e),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check network connection").retryable(true),
		})?;

		let mut tb = TransactionBuilder::with_client(self.client.as_ref());
		tb.extend_script(sb.to_bytes());
		tb.set_signers(vec![signer.into()]).map_err(|e| NeoError::Transaction {
			message: format!("Failed to set signer: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new().suggest("Ensure signer configuration is valid"),
		})?;
		tb.valid_until_block(current_height + 2400).map_err(|e| NeoError::Transaction {
			message: format!("Invalid valid-until-block: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new().retryable(true),
		})?;

		let mut tx = tb.sign().await.map_err(|e| NeoError::Transaction {
			message: format!("Failed to sign deploy transaction: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Ensure deployer account has a private key")
				.suggest("Multi-sig accounts require manual signing"),
		})?;

		{
			let mut attempt = 0usize;
			loop {
				attempt += 1;
				match tx.send_tx().await {
					Ok(_) => break,
					Err(err) if attempt < max_attempts => {
						tracing::warn!(
							attempt = attempt,
							error = %err,
							"Send transaction failed; retrying"
						);
						tokio::time::sleep(retry_delay).await;
					},
					Err(err) => {
						return Err(NeoError::Transaction {
							message: format!("Failed to send deploy transaction: {}", err),
							tx_hash: None,
							source: None,
							recovery: ErrorRecovery::new()
								.suggest("Check RPC endpoint availability")
								.retryable(true),
						});
					},
				}
			}
		}

		Ok(expected_hash)
	}

	/// Invoke a smart contract method (read-only)
	///
	/// For contract methods that don't modify state.
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::neo_types::{ContractParameter, ScriptHash};
	/// use neo3::sdk::Neo;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// let contract_hash = ScriptHash::zero();
	/// let result = neo
	///     .invoke_read(&contract_hash, "balanceOf", Vec::<ContractParameter>::new())
	///     .await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn invoke_read(
		&self,
		contract: &ScriptHash,
		method: &str,
		params: Vec<ContractParameter>,
	) -> Result<serde_json::Value, NeoError> {
		use crate::neo_error::unified::ErrorRecovery;
		use crate::neo_types::ScriptHashExtension;

		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let retry_delay = Duration::from_millis(250);

		let result = crate::neo_utils::error::retry(
			|| async {
				self.client
					.invoke_function(contract, method.to_string(), params.clone(), None)
					.await
			},
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to invoke read-only method: {}", e),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check network connection").retryable(true),
		})?;

		if result.has_state_fault() {
			return Err(NeoError::Contract {
				message: result
					.exception
					.clone()
					.unwrap_or_else(|| "Invocation resulted in FAULT state".to_string()),
				contract: Some(contract.to_hex()),
				method: Some(method.to_string()),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check contract parameters")
					.suggest("Ensure the method is safe/read-only"),
			});
		}

		serde_json::to_value(result).map_err(|e| NeoError::Other {
			message: format!("Failed to serialize invocation result: {}", e),
			source: None,
			recovery: ErrorRecovery::new(),
		})
	}

	/// Invoke a smart contract method (with transaction)
	///
	/// For contract methods that modify state.
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::neo_types::{ContractParameter, ScriptHash};
	/// use neo3::neo_wallets::wallet::Wallet;
	/// use neo3::sdk::Neo;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// let wallet = Wallet::new();
	/// let contract_hash = ScriptHash::zero();
	/// let tx_hash = neo
	///     .invoke_write(&wallet, &contract_hash, "transfer", Vec::<ContractParameter>::new())
	///     .await?;
	/// println!("Transaction sent: {}", tx_hash);
	/// # Ok(())
	/// # }
	/// ```
	pub async fn invoke_write(
		&self,
		signer: &Wallet,
		contract: &ScriptHash,
		method: &str,
		params: Vec<ContractParameter>,
	) -> Result<TxHash, NeoError> {
		use crate::neo_builder::{AccountSigner, CallFlags, ScriptBuilder, TransactionBuilder};
		use crate::neo_error::unified::ErrorRecovery;
		use crate::neo_types::ScriptHashExtension;
		use crate::neo_wallets::WalletTrait;

		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let retry_delay = Duration::from_millis(250);

		let signer_account = signer.default_account().ok_or_else(|| NeoError::Wallet {
			message: "No default account set in signer wallet".to_string(),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Set a default account using set_default_account()")
				.suggest("Add an account to the wallet first"),
		})?;

		let mut sb = ScriptBuilder::new();
		sb.contract_call(contract, method, params.as_slice(), Some(CallFlags::All))
			.map_err(|e| NeoError::Contract {
				message: format!("Failed to build invocation script: {}", e),
				contract: Some(contract.to_hex()),
				method: Some(method.to_string()),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check contract hash")
					.suggest("Check method name and parameters"),
			})?;

		let signer_obj =
			AccountSigner::called_by_entry(signer_account).map_err(|e| NeoError::Transaction {
				message: format!("Failed to create signer: {}", e),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Ensure signer wallet has an unlocked private key"),
			})?;

		let current_height = crate::neo_utils::error::retry(
			|| async { self.client.get_block_count().await },
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to fetch current block height: {}", e),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check network connection").retryable(true),
		})?;

		let mut tb = TransactionBuilder::with_client(self.client.as_ref());
		tb.extend_script(sb.to_bytes());
		tb.set_signers(vec![signer_obj.into()]).map_err(|e| NeoError::Transaction {
			message: format!("Failed to set signer: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new(),
		})?;
		tb.valid_until_block(current_height + 2400).map_err(|e| NeoError::Transaction {
			message: format!("Invalid valid-until-block: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new().retryable(true),
		})?;

		let mut tx = tb.sign().await.map_err(|e| NeoError::Transaction {
			message: format!("Failed to sign invocation transaction: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Ensure signer has a private key")
				.suggest("Multi-sig accounts require manual signing"),
		})?;

		let result = {
			let mut attempt = 0usize;
			loop {
				attempt += 1;
				match tx.send_tx().await {
					Ok(result) => break Ok(result),
					Err(err) if attempt < max_attempts => {
						tracing::warn!(
							attempt = attempt,
							error = %err,
							"Send transaction failed; retrying"
						);
						tokio::time::sleep(retry_delay).await;
					},
					Err(err) => break Err(err),
				}
			}
		}
		.map_err(|e| NeoError::Transaction {
			message: format!("Failed to send invocation transaction: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check RPC endpoint availability")
				.retryable(true),
		})?;

		Ok(result.hash.to_string())
	}

	/// Wait for a transaction to be confirmed
	///
	/// # Examples
	///
	/// ```no_run
	/// use neo3::sdk::Neo;
	/// use std::time::Duration;
	///
	/// # #[tokio::main]
	/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let neo = Neo::testnet().await?;
	/// let tx_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
	/// neo.wait_for_confirmation(tx_hash, Duration::from_secs(60)).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn wait_for_confirmation(
		&self,
		tx_hash: &str,
		timeout: Duration,
	) -> Result<(), NeoError> {
		use crate::neo_error::unified::ErrorRecovery;
		use primitive_types::H256;
		use std::str::FromStr;
		use std::time::Instant;

		let tx_h256 = H256::from_str(tx_hash).map_err(|e| NeoError::Validation {
			message: format!("Invalid transaction hash: {}", e),
			field: "tx_hash".to_string(),
			value: Some(tx_hash.to_string()),
			recovery: ErrorRecovery::new().suggest("Provide a valid 0x-prefixed transaction hash"),
		})?;

		let start = Instant::now();
		while start.elapsed() < timeout {
			match self.client.get_application_log(tx_h256).await {
				Ok(_) => return Ok(()),
				Err(_) => tokio::time::sleep(Duration::from_secs(1)).await,
			}
		}

		Err(NeoError::Timeout {
			duration: timeout,
			operation: "wait_for_confirmation".to_string(),
			recovery: ErrorRecovery::new()
				.suggest("Increase the timeout duration")
				.suggest("Check the transaction hash")
				.retryable(true),
		})
	}

	/// Get the current block height
	pub async fn get_block_height(&self) -> Result<u32, NeoError> {
		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let retry_delay = Duration::from_millis(250);

		crate::neo_utils::error::retry(
			|| async { self.client.get_block_count().await },
			max_attempts,
			retry_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to get block height: {}", e),
			source: None,
			recovery: crate::neo_error::unified::ErrorRecovery::new()
				.suggest("Check network connection")
				.retryable(true),
		})
	}

	/// Get access to the underlying RPC client for advanced operations
	pub fn client(&self) -> &RpcClient<HttpProvider> {
		&self.client
	}

	/// Get the configured RPC endpoint URL.
	pub fn endpoint(&self) -> &str {
		&self.endpoint
	}

	/// Get the current network
	pub fn network(&self) -> &Network {
		&self.network
	}
}

/// Builder for configuring the Neo SDK
#[derive(Debug)]
pub struct NeoBuilder {
	network: Network,
	config: SdkConfig,
}

impl Default for NeoBuilder {
	fn default() -> Self {
		Self { network: Network::TestNet, config: SdkConfig::default() }
	}
}

impl NeoBuilder {
	/// Set the network to connect to
	pub fn network(mut self, network: Network) -> Self {
		self.network = network;
		self
	}

	/// Set the request timeout
	pub fn timeout(mut self, timeout: Duration) -> Self {
		self.config.timeout = timeout;
		self
	}

	/// Set the number of retries for failed requests
	pub fn retries(mut self, retries: u32) -> Self {
		self.config.retries = retries;
		self
	}

	/// Enable or disable caching
	pub fn cache(mut self, enabled: bool) -> Self {
		self.config.cache_enabled = enabled;
		self
	}

	/// Enable or disable metrics collection
	pub fn metrics(mut self, enabled: bool) -> Self {
		self.config.metrics_enabled = enabled;
		self
	}

	/// Build the Neo SDK instance
	pub async fn build(self) -> Result<Neo, NeoError> {
		use crate::neo_error::unified::ErrorRecovery;

		let endpoint = match &self.network {
			Network::MainNet => "https://mainnet1.neo.org:443".to_string(),
			Network::TestNet => "https://testnet1.neo.org:443".to_string(),
			Network::Custom(url) => url.clone(),
		};

		let url = url::Url::parse(&endpoint).map_err(|e| NeoError::Network {
			message: format!("Invalid RPC endpoint URL: {}", e),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check the RPC endpoint URL")
				.suggest("Ensure it includes a valid scheme (http/https)"),
		})?;

		let http_client =
			reqwest::Client::builder().timeout(self.config.timeout).build().map_err(|e| {
				NeoError::Network {
					message: format!("Failed to build HTTP client: {}", e),
					source: None,
					recovery: ErrorRecovery::new()
						.suggest("Check your TLS configuration")
						.suggest("Verify system root certificates are available"),
				}
			})?;

		let provider = HttpProvider::new_with_client(url, http_client);
		let client = Arc::new(RpcClient::new(provider));

		// Test connection
		let max_attempts = self.config.retries.saturating_add(1) as usize;
		let connect_delay = Duration::from_millis(250);

		crate::neo_utils::error::retry(
			|| async { client.get_block_count().await },
			max_attempts,
			connect_delay,
		)
		.await
		.map_err(|e| NeoError::Network {
			message: format!("Failed to connect to Neo network: {}", e),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Verify the RPC endpoint is accessible")
				.suggest("Check your internet connection")
				.suggest("Try a different RPC endpoint")
				.retryable(true)
				.retry_after(std::time::Duration::from_secs(5)),
		})?;

		let cache = self.config.cache_enabled.then(RpcCache::new_rpc_cache);

		Ok(Neo { client, network: self.network, endpoint, cache, config: self.config })
	}
}

/// Quick transfer builder for simplified token transfers
#[derive(Debug)]
#[allow(dead_code)]
pub struct Transfer {
	from: Wallet,
	to: String,
	amount: u64,
	token: Token,
	memo: Option<String>,
}

impl Transfer {
	/// Create a new transfer
	pub fn new(from: Wallet, to: impl Into<String>, amount: u64, token: Token) -> Self {
		Self { from, to: to.into(), amount, token, memo: None }
	}

	/// Add an optional memo to the transfer
	pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
		self.memo = Some(memo.into());
		self
	}

	/// Execute the transfer
	pub async fn execute(self, client: &RpcClient<HttpProvider>) -> Result<TxHash, NeoError> {
		use crate::neo_builder::{AccountSigner, CallFlags, ScriptBuilder, TransactionBuilder};
		use crate::neo_error::unified::ErrorRecovery;
		use crate::neo_types::ScriptHashExtension;
		use crate::neo_wallets::WalletTrait;

		let from_account = self.from.default_account().ok_or_else(|| NeoError::Wallet {
			message: "No default account set in wallet".to_string(),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Set a default account using set_default_account()")
				.suggest("Add an account to the wallet first"),
		})?;

		let to_hash = ScriptHash::from_address(&self.to).map_err(|e| NeoError::Validation {
			message: format!("Invalid recipient address: {}", e),
			field: "to".to_string(),
			value: Some(self.to.clone()),
			recovery: ErrorRecovery::new()
				.suggest("Check the address format")
				.suggest("Ensure it's a valid Neo N3 address"),
		})?;

		let contract_hash = match self.token {
			Token::NEO => ScriptHash::from(hex!("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")),
			Token::GAS => ScriptHash::from(hex!("d2a4cff31913016155e38e474a2c06d08be276cf")),
			Token::Custom(hash) => hash,
		};

		let amount_i64 = i64::try_from(self.amount).map_err(|_| NeoError::Validation {
			message: "Amount is too large to fit Neo VM integer".to_string(),
			field: "amount".to_string(),
			value: Some(self.amount.to_string()),
			recovery: ErrorRecovery::new().suggest("Use a smaller amount"),
		})?;

		let mut sb = ScriptBuilder::new();
		sb.contract_call(
			&contract_hash,
			"transfer",
			&[
				ContractParameter::h160(&from_account.get_script_hash()),
				ContractParameter::h160(&to_hash),
				ContractParameter::integer(amount_i64),
				ContractParameter::any(),
			],
			Some(CallFlags::All),
		)
		.map_err(|e| NeoError::Contract {
			message: format!("Failed to build transfer script: {}", e),
			contract: Some(contract_hash.to_hex()),
			method: Some("transfer".to_string()),
			source: None,
			recovery: ErrorRecovery::new(),
		})?;

		let signer =
			AccountSigner::called_by_entry(from_account).map_err(|e| NeoError::Transaction {
				message: format!("Failed to create signer: {}", e),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Ensure the wallet has an unlocked private key"),
			})?;

		let current_height = client.get_block_count().await.map_err(|e| NeoError::Network {
			message: format!("Failed to fetch current block height: {}", e),
			source: None,
			recovery: ErrorRecovery::new().retryable(true),
		})?;

		let mut tb = TransactionBuilder::with_client(client);
		tb.extend_script(sb.to_bytes());
		tb.set_signers(vec![signer.into()]).map_err(|e| NeoError::Transaction {
			message: format!("Failed to set signers: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new(),
		})?;
		tb.valid_until_block(current_height + 5760).map_err(|e| NeoError::Transaction {
			message: format!("Invalid valid-until-block: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new(),
		})?;

		let mut tx = tb.sign().await.map_err(|e| NeoError::Transaction {
			message: format!("Failed to sign transfer: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new(),
		})?;

		let result = tx.send_tx().await.map_err(|e| NeoError::Transaction {
			message: format!("Failed to send transfer: {}", e),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new().retryable(true),
		})?;

		Ok(result.hash.to_string())
	}
}

fn invalid_balance_response(token: &str, detail: impl Into<String>) -> NeoError {
	let detail = detail.into();
	NeoError::Other {
		message: format!("Invalid {token} balance response: {detail}"),
		source: None,
		recovery: ErrorRecovery::new()
			.suggest("Retry against another RPC endpoint")
			.suggest("Inspect the raw balance response from the node"),
	}
}

fn parse_balance_stack_item_u64(item: &StackItem, token: &str) -> Result<u64, NeoError> {
	let bytes = item.as_bytes().ok_or_else(|| {
		invalid_balance_response(token, "balance stack item is not byte-convertible")
	})?;
	let value = BigInt::from_signed_bytes_le(&bytes);

	match value.sign() {
		Sign::Minus => Err(invalid_balance_response(token, "balance cannot be negative")),
		_ => value
			.to_u64()
			.ok_or_else(|| invalid_balance_response(token, "balance does not fit into u64")),
	}
}

fn parse_nep17_decimals(decimals: Option<&str>, asset_hash: &ScriptHash) -> Result<u8, NeoError> {
	let raw = decimals.ok_or_else(|| NeoError::Other {
		message: format!("Missing decimals for NEP-17 token {}", asset_hash.to_hex()),
		source: None,
		recovery: ErrorRecovery::new()
			.suggest("Retry against another RPC endpoint")
			.suggest("Verify the token metadata is available from the node"),
	})?;

	raw.parse::<u8>().map_err(|_| NeoError::Other {
		message: format!("Invalid decimals '{}' for NEP-17 token {}", raw, asset_hash.to_hex()),
		source: None,
		recovery: ErrorRecovery::new()
			.suggest("Retry against another RPC endpoint")
			.suggest("Verify the token contract returns a valid decimals value"),
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::neo_types::StackItem;

	#[test]
	fn test_builder_configuration() {
		let builder = Neo::builder()
			.network(Network::TestNet)
			.timeout(Duration::from_secs(60))
			.retries(5)
			.cache(true)
			.metrics(false);

		assert_eq!(builder.config.timeout, Duration::from_secs(60));
		assert_eq!(builder.config.retries, 5);
		assert!(builder.config.cache_enabled);
		assert!(!builder.config.metrics_enabled);
	}

	#[test]
	fn test_parse_balance_stack_item_u64_rejects_negative_value() {
		let item = StackItem::Integer { value: -1 };
		let err = parse_balance_stack_item_u64(&item, "NEO").unwrap_err();

		match err {
			NeoError::Other { message, .. } => {
				assert!(message.contains("balance cannot be negative"));
			},
			other => panic!("expected balance parsing error, got {other:?}"),
		}
	}

	#[test]
	fn test_parse_nep17_decimals_rejects_invalid_value() {
		let asset_hash = ScriptHash::zero();
		let err = parse_nep17_decimals(Some("not-a-u8"), &asset_hash).unwrap_err();

		match err {
			NeoError::Other { message, .. } => {
				assert!(message.contains("Invalid decimals"));
			},
			other => panic!("expected decimals parsing error, got {other:?}"),
		}
	}
}
