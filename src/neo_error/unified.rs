//! Unified error handling system for improved developer experience
//!
//! This module provides a hierarchical error system with context,
//! recovery suggestions, and better error messages.
#![allow(dead_code)]

use std::fmt;
use thiserror::Error;

/// Unified error type for the entire Neo SDK
///
/// Provides consistent error handling with context and recovery suggestions.
#[derive(Debug, Error)]
pub enum NeoError {
	/// Network-related errors
	#[error("Network error: {message}")]
	Network {
		message: String,
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		recovery: ErrorRecovery,
	},

	/// Wallet and account errors
	#[error("Wallet error: {message}")]
	Wallet {
		message: String,
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		recovery: ErrorRecovery,
	},

	/// Smart contract errors
	#[error("Contract error: {message}")]
	Contract {
		message: String,
		contract: Option<String>,
		method: Option<String>,
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		recovery: ErrorRecovery,
	},

	/// Transaction errors
	#[error("Transaction failed: {message}")]
	Transaction {
		message: String,
		tx_hash: Option<String>,
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		recovery: ErrorRecovery,
	},

	/// Configuration errors
	#[error("Configuration error: {message}")]
	Configuration { message: String, field: Option<String>, recovery: ErrorRecovery },

	/// Validation errors
	#[error("Validation error: {message}")]
	Validation { message: String, field: String, value: Option<String>, recovery: ErrorRecovery },

	/// Insufficient funds error
	#[error("Insufficient funds: need {required} but have {available}")]
	InsufficientFunds {
		required: String,
		available: String,
		token: String,
		recovery: ErrorRecovery,
	},

	/// Timeout error
	#[error("Operation timed out after {duration:?}")]
	Timeout { duration: std::time::Duration, operation: String, recovery: ErrorRecovery },

	/// Rate limit error
	#[error("Rate limit exceeded: {message}")]
	RateLimit { message: String, retry_after: Option<std::time::Duration>, recovery: ErrorRecovery },

	/// Generic error with context
	#[error("{message}")]
	Other {
		message: String,
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		recovery: ErrorRecovery,
	},
}

/// Error recovery suggestions
#[derive(Debug, Clone, Default)]
pub struct ErrorRecovery {
	/// Suggested actions to recover from the error
	pub suggestions: Vec<String>,
	/// Whether the operation can be retried
	pub retryable: bool,
	/// Recommended retry delay if retryable
	pub retry_after: Option<std::time::Duration>,
	/// Links to documentation
	pub docs: Vec<String>,
}

impl ErrorRecovery {
	/// Create a new recovery suggestion
	pub fn new() -> Self {
		Self::default()
	}

	/// Add a suggestion
	pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
		self.suggestions.push(suggestion.into());
		self
	}

	/// Mark as retryable
	pub fn retryable(mut self, retryable: bool) -> Self {
		self.retryable = retryable;
		self
	}

	/// Set retry delay
	pub fn retry_after(mut self, duration: std::time::Duration) -> Self {
		self.retry_after = Some(duration);
		self.retryable = true;
		self
	}

	/// Add documentation link
	pub fn doc(mut self, link: impl Into<String>) -> Self {
		self.docs.push(link.into());
		self
	}
}

impl fmt::Display for ErrorRecovery {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if !self.suggestions.is_empty() {
			writeln!(f, "\n💡 Suggestions:")?;
			for suggestion in &self.suggestions {
				writeln!(f, "   • {}", suggestion)?;
			}
		}

		if self.retryable {
			write!(f, "\n🔄 This operation can be retried")?;
			if let Some(duration) = self.retry_after {
				write!(f, " after {:?}", duration)?;
			}
			writeln!(f)?;
		}

		if !self.docs.is_empty() {
			writeln!(f, "\n📚 See documentation:")?;
			for doc in &self.docs {
				writeln!(f, "   • {}", doc)?;
			}
		}

		Ok(())
	}
}

/// Result type alias for Neo operations
pub type Result<T> = std::result::Result<T, NeoError>;

/// Convert legacy `Neo3Error` values into the unified error type.
///
/// This is a best-effort mapping intended to ease migration for callers that
/// still receive `Neo3Error` from lower-level APIs.
impl From<super::Neo3Error> for NeoError {
	fn from(err: super::Neo3Error) -> Self {
		use super::{Neo3Error, NetworkError, TransactionError};

		match err {
			Neo3Error::Network(network_error) => match network_error {
				NetworkError::RateLimitExceeded => NeoError::RateLimit {
					message: network_error.to_string(),
					retry_after: None,
					recovery: ErrorRecovery::new()
						.suggest("Retry after a short delay")
						.retryable(true),
				},
				_ => NeoError::Network {
					message: network_error.to_string(),
					source: None,
					recovery: ErrorRecovery::new().retryable(true),
				},
			},
			Neo3Error::Wallet(wallet_error) => NeoError::Wallet {
				message: wallet_error.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			Neo3Error::Transaction(tx_error) => match tx_error {
				TransactionError::InsufficientFunds { required, available } => {
					NeoError::InsufficientFunds {
						required: required.to_string(),
						available: available.to_string(),
						token: "unknown".to_string(),
						recovery: ErrorRecovery::new()
							.suggest("Check the account balance")
							.retryable(false),
					}
				},
				_ => NeoError::Transaction {
					message: tx_error.to_string(),
					tx_hash: None,
					source: None,
					recovery: ErrorRecovery::new(),
				},
			},
			Neo3Error::Contract(contract_error) => NeoError::Contract {
				message: contract_error.to_string(),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
			Neo3Error::Config(message) => {
				NeoError::Configuration { message, field: None, recovery: ErrorRecovery::new() }
			},
			Neo3Error::Crypto(crypto_error) => NeoError::Other {
				message: crypto_error.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			Neo3Error::Serialization(serialization_error) => NeoError::Other {
				message: serialization_error.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			Neo3Error::Generic { message } => {
				NeoError::Other { message, source: None, recovery: ErrorRecovery::new() }
			},
			Neo3Error::UnsupportedOperation(message) => {
				NeoError::Other { message, source: None, recovery: ErrorRecovery::new() }
			},
		}
	}
}

/// Builder for creating detailed errors with context
pub struct ErrorBuilder {
	kind: ErrorKind,
	message: String,
	source: Option<Box<dyn std::error::Error + Send + Sync>>,
	recovery: ErrorRecovery,
	context: ErrorContext,
}

#[derive(Debug)]
enum ErrorKind {
	Network,
	Wallet,
	Contract,
	Transaction,
	Configuration,
	Validation,
	InsufficientFunds,
	Timeout,
	RateLimit,
	Other,
}

#[derive(Debug, Default)]
struct ErrorContext {
	contract: Option<String>,
	method: Option<String>,
	tx_hash: Option<String>,
	field: Option<String>,
	value: Option<String>,
	required: Option<String>,
	available: Option<String>,
	token: Option<String>,
	duration: Option<std::time::Duration>,
	operation: Option<String>,
}

impl ErrorBuilder {
	/// Create a network error
	pub fn network(message: impl Into<String>) -> Self {
		Self {
			kind: ErrorKind::Network,
			message: message.into(),
			source: None,
			recovery: ErrorRecovery::default(),
			context: ErrorContext::default(),
		}
	}

	/// Create a wallet error
	pub fn wallet(message: impl Into<String>) -> Self {
		Self {
			kind: ErrorKind::Wallet,
			message: message.into(),
			source: None,
			recovery: ErrorRecovery::default(),
			context: ErrorContext::default(),
		}
	}

	/// Create a contract error
	pub fn contract(message: impl Into<String>) -> Self {
		Self {
			kind: ErrorKind::Contract,
			message: message.into(),
			source: None,
			recovery: ErrorRecovery::default(),
			context: ErrorContext::default(),
		}
	}

	/// Add source error
	pub fn source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
		self.source = Some(Box::new(source));
		self
	}

	/// Add contract context
	pub fn with_contract(mut self, contract: impl Into<String>) -> Self {
		self.context.contract = Some(contract.into());
		self
	}

	/// Add method context
	pub fn with_method(mut self, method: impl Into<String>) -> Self {
		self.context.method = Some(method.into());
		self
	}

	/// Add recovery suggestion
	pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
		self.recovery = self.recovery.suggest(suggestion);
		self
	}

	/// Mark as retryable
	pub fn retryable(mut self) -> Self {
		self.recovery = self.recovery.retryable(true);
		self
	}

	/// Build the error
	pub fn build(self) -> NeoError {
		match self.kind {
			ErrorKind::Network => NeoError::Network {
				message: self.message,
				source: self.source,
				recovery: self.recovery,
			},
			ErrorKind::Wallet => NeoError::Wallet {
				message: self.message,
				source: self.source,
				recovery: self.recovery,
			},
			ErrorKind::Contract => NeoError::Contract {
				message: self.message,
				contract: self.context.contract,
				method: self.context.method,
				source: self.source,
				recovery: self.recovery,
			},
			_ => NeoError::Other {
				message: self.message,
				source: self.source,
				recovery: self.recovery,
			},
		}
	}
}

/// Extension trait for adding context to errors
pub trait ErrorContextExt {
	/// Add context to this error
	fn context(self, message: impl Into<String>) -> NeoError;

	/// Add recovery suggestions
	fn recover(self, suggestion: impl Into<String>) -> NeoError;
}

impl<E> ErrorContextExt for E
where
	E: std::error::Error + Send + Sync + 'static,
{
	fn context(self, message: impl Into<String>) -> NeoError {
		NeoError::Other {
			message: message.into(),
			source: Some(Box::new(self)),
			recovery: ErrorRecovery::default(),
		}
	}

	fn recover(self, suggestion: impl Into<String>) -> NeoError {
		NeoError::Other {
			message: self.to_string(),
			source: Some(Box::new(self)),
			recovery: ErrorRecovery::new().suggest(suggestion),
		}
	}
}

// =============================================================================
// From implementations for legacy error types
// =============================================================================

impl From<crate::neo_crypto::CryptoError> for NeoError {
	fn from(err: crate::neo_crypto::CryptoError) -> Self {
		use crate::neo_crypto::CryptoError;
		match &err {
			CryptoError::InvalidPassphrase(msg) => NeoError::Wallet {
				message: format!("Invalid passphrase: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check that the password is correct")
					.suggest("Ensure the wallet file is not corrupted"),
			},
			CryptoError::InvalidPrivateKey | CryptoError::InvalidPublicKey => NeoError::Wallet {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the key format is correct")
					.suggest("Ensure the key was not truncated"),
			},
			CryptoError::SigningError | CryptoError::SignatureVerificationError => {
				NeoError::Transaction {
					message: err.to_string(),
					tx_hash: None,
					source: None,
					recovery: ErrorRecovery::new().suggest("Verify the signing key is correct"),
				}
			},
			CryptoError::DecryptionError(msg) => NeoError::Wallet {
				message: format!("Decryption failed: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check that the password is correct")
					.suggest("Ensure the encrypted data is not corrupted"),
			},
			_ => NeoError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_wallets::WalletError> for NeoError {
	fn from(err: crate::neo_wallets::WalletError) -> Self {
		use crate::neo_wallets::WalletError;
		match &err {
			WalletError::NoKeyPair => NeoError::Wallet {
				message: "No key pair available".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Import a private key or create a new account")
					.suggest("Decrypt the wallet if it is encrypted"),
			},
			WalletError::NoDefaultAccount => NeoError::Wallet {
				message: "No default account set".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Set a default account using set_default_account()")
					.suggest("Add an account to the wallet first"),
			},
			WalletError::NoAccounts => NeoError::Wallet {
				message: "Wallet has no accounts".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Create a new account")
					.suggest("Import an existing account"),
			},
			WalletError::DecryptionError(msg) => NeoError::Wallet {
				message: format!("Decryption failed: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the password is correct")
					.suggest("Check if the wallet file is corrupted"),
			},
			WalletError::AccountState(msg) => NeoError::Wallet {
				message: format!("Account state error: {}", msg),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			WalletError::FileError(msg) => NeoError::Wallet {
				message: format!("File operation failed: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check file permissions")
					.suggest("Verify the file path is correct"),
			},
			_ => NeoError::Wallet {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_builder::BuilderError> for NeoError {
	fn from(err: crate::neo_builder::BuilderError) -> Self {
		use crate::neo_builder::BuilderError;
		match &err {
			BuilderError::SignerConfiguration(msg) => NeoError::Transaction {
				message: format!("Signer configuration error: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify signer accounts are valid")
					.suggest("Check signer scopes are appropriate"),
			},
			BuilderError::TransactionConfiguration(msg) => NeoError::Transaction {
				message: format!("Transaction configuration error: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Review transaction parameters")
					.suggest("Ensure all required fields are set"),
			},
			BuilderError::TooManySigners(msg) => NeoError::Transaction {
				message: format!("Too many signers: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Reduce the number of signers")
					.doc("https://docs.neo.org/docs/n3/foundation/Transactions"),
			},
			BuilderError::InvalidScript(msg) => NeoError::Transaction {
				message: format!("Invalid script: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the script is correctly formatted")
					.suggest("Check for invalid opcodes"),
			},
			_ => NeoError::Transaction {
				message: err.to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_builder::TransactionError> for NeoError {
	fn from(err: crate::neo_builder::TransactionError) -> Self {
		use crate::neo_builder::TransactionError;
		match &err {
			TransactionError::NoSigners => NeoError::Transaction {
				message: "Transaction has no signers".to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Add at least one signer to the transaction")
					.suggest("Use set_signers() or add_signer()"),
			},
			TransactionError::NoScript | TransactionError::EmptyScript => NeoError::Transaction {
				message: "Transaction has no script".to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Set the transaction script using set_script()")
					.suggest("Build a script using ScriptBuilder"),
			},
			TransactionError::InsufficientFunds => NeoError::InsufficientFunds {
				required: "unknown".to_string(),
				available: "unknown".to_string(),
				token: "GAS".to_string(),
				recovery: ErrorRecovery::new()
					.suggest("Check account balance")
					.suggest("Acquire more GAS tokens")
					.retryable(false),
			},
			TransactionError::TxTooLarge => NeoError::Transaction {
				message: "Transaction exceeds maximum size".to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Split the transaction into smaller parts")
					.suggest("Reduce the script size"),
			},
			_ => NeoError::Transaction {
				message: err.to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_contract::ContractError> for NeoError {
	fn from(err: crate::neo_contract::ContractError) -> Self {
		use crate::neo_contract::ContractError;
		match &err {
			ContractError::InvalidNeoName(name) => NeoError::Contract {
				message: format!("Invalid NNS name: {}", name),
				contract: Some("NameService".to_string()),
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the domain name format")
					.suggest("Ensure the name follows NNS naming rules"),
			},
			ContractError::UnresolvableDomainName(name) => NeoError::Contract {
				message: format!("Cannot resolve domain: {}", name),
				contract: Some("NameService".to_string()),
				method: Some("resolve".to_string()),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the domain is registered")
					.suggest("Check if the domain has expired"),
			},
			ContractError::InvocationFailed(msg) => NeoError::Contract {
				message: format!("Contract invocation failed: {}", msg),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check contract parameters")
					.suggest("Verify the contract is deployed")
					.retryable(true),
			},
			ContractError::ProviderNotSet(msg) => NeoError::Configuration {
				message: format!("Provider not configured: {}", msg),
				field: Some("provider".to_string()),
				recovery: ErrorRecovery::new()
					.suggest("Set an RPC provider before calling contract methods")
					.suggest("Use with_provider() to configure the client"),
			},
			_ => NeoError::Contract {
				message: err.to_string(),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_types::TypeError> for NeoError {
	fn from(err: crate::neo_types::TypeError) -> Self {
		use crate::neo_types::TypeError;
		match &err {
			TypeError::InvalidAddress => NeoError::Validation {
				message: "Invalid Neo address".to_string(),
				field: "address".to_string(),
				value: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the address format (should start with 'N')")
					.suggest("Verify the address checksum"),
			},
			TypeError::InvalidPrivateKey | TypeError::InvalidPublicKey => NeoError::Validation {
				message: err.to_string(),
				field: "key".to_string(),
				value: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the key format")
					.suggest("Check key length (32 bytes for private, 33/65 for public)"),
			},
			TypeError::InvalidFormat(msg) => NeoError::Validation {
				message: format!("Invalid format: {}", msg),
				field: "data".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Check the data format"),
			},
			TypeError::NumericOverflow => NeoError::Validation {
				message: "Numeric overflow".to_string(),
				field: "number".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Use a smaller value"),
			},
			_ => NeoError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_clients::ProviderError> for NeoError {
	fn from(err: crate::neo_clients::ProviderError) -> Self {
		NeoError::Network {
			message: err.to_string(),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check network connectivity")
				.suggest("Verify the RPC endpoint is accessible")
				.retryable(true),
		}
	}
}

impl From<std::io::Error> for NeoError {
	fn from(err: std::io::Error) -> Self {
		NeoError::Other {
			message: format!("IO error: {}", err),
			source: Some(Box::new(err)),
			recovery: ErrorRecovery::new()
				.suggest("Check file permissions")
				.suggest("Verify the path exists"),
		}
	}
}

impl From<hex::FromHexError> for NeoError {
	fn from(err: hex::FromHexError) -> Self {
		NeoError::Validation {
			message: format!("Invalid hex string: {}", err),
			field: "hex".to_string(),
			value: None,
			recovery: ErrorRecovery::new()
				.suggest("Ensure the string contains only hex characters (0-9, a-f)")
				.suggest("Check for correct string length"),
		}
	}
}

impl From<serde_json::Error> for NeoError {
	fn from(err: serde_json::Error) -> Self {
		NeoError::Other {
			message: format!("JSON error: {}", err),
			source: Some(Box::new(err)),
			recovery: ErrorRecovery::new().suggest("Check JSON format and structure"),
		}
	}
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_error_builder() {
		let error = ErrorBuilder::network("Connection failed")
			.suggest("Check your internet connection")
			.suggest("Try a different RPC endpoint")
			.retryable()
			.build();

		match error {
			NeoError::Network { recovery, .. } => {
				assert_eq!(recovery.suggestions.len(), 2);
				assert!(recovery.retryable);
			},
			_ => panic!("Wrong error type"),
		}
	}

	#[test]
	fn test_error_display() {
		let error = NeoError::InsufficientFunds {
			required: "100 GAS".to_string(),
			available: "50 GAS".to_string(),
			token: "GAS".to_string(),
			recovery: ErrorRecovery::new()
				.suggest("Acquire more GAS tokens")
				.suggest("Reduce the transaction amount")
				.doc("https://docs.neo.org/tokens/gas"),
		};

		let display = format!("{}", error);
		assert!(display.contains("Insufficient funds"));
		assert!(display.contains("need 100 GAS but have 50 GAS"));
	}
}
