#![deny(missing_docs)]
//! Unified error handling system for improved developer experience
//!
//! This module provides a hierarchical error system with context,
//! recovery suggestions, and better error messages.

use std::fmt;
use thiserror::Error;

/// Unified error type for the entire Neo SDK
///
/// Provides consistent error handling with context and recovery suggestions.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NeoError {
	/// Network / RPC transport failure.
	#[error("Network error: {message}")]
	Network {
		/// Human-readable description of the failure (sans source error).
		message: String,
		/// Underlying transport error, if any (`reqwest`, JSON-RPC, etc.).
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints (retryable flag, suggested actions, doc links).
		recovery: ErrorRecovery,
	},

	/// Wallet / account / key-material problem.
	#[error("Wallet error: {message}")]
	Wallet {
		/// Human-readable description.
		message: String,
		/// Underlying wallet/crypto error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Smart-contract invocation problem (missing method, VM fault, …).
	#[error("Contract error: {message}")]
	Contract {
		/// Human-readable description.
		message: String,
		/// Contract hash (hex) the failure was associated with, if known.
		contract: Option<String>,
		/// Contract method name the failure was associated with, if known.
		method: Option<String>,
		/// Underlying error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Transaction construction, signing, or submission failure.
	#[error("Transaction failed: {message}")]
	Transaction {
		/// Human-readable description.
		message: String,
		/// Transaction hash (hex) involved in the failure, if known.
		tx_hash: Option<String>,
		/// Underlying error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Bad SDK/client configuration.
	#[error("Configuration error: {message}")]
	Configuration {
		/// Human-readable description.
		message: String,
		/// Name of the offending configuration field, if applicable.
		field: Option<String>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// User-supplied input validation failure.
	#[error("Validation error: {message}")]
	Validation {
		/// Human-readable description.
		message: String,
		/// Name of the offending input field.
		field: String,
		/// Offending value (when safe to echo back).
		value: Option<String>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Account balance is too low for the requested operation.
	#[error("Insufficient funds: need {required} but have {available}")]
	InsufficientFunds {
		/// Required amount, formatted for display.
		required: String,
		/// Available amount, formatted for display.
		available: String,
		/// Token symbol or contract identifier.
		token: String,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Operation exceeded its deadline.
	#[error("Operation timed out after {duration:?}")]
	Timeout {
		/// How long the operation was waited on before giving up.
		duration: std::time::Duration,
		/// Name of the operation that timed out.
		operation: String,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Provider applied a rate limit.
	#[error("Rate limit exceeded: {message}")]
	RateLimit {
		/// Human-readable description.
		message: String,
		/// Suggested wait duration before retrying, if the provider returned one.
		retry_after: Option<std::time::Duration>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Uncategorised / future-compatible bucket.
	#[error("{message}")]
	Other {
		/// Human-readable description.
		message: String,
		/// Underlying error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
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
	#[must_use]
	pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
		self.suggestions.push(suggestion.into());
		self
	}

	/// Mark as retryable
	#[must_use]
	pub fn retryable(mut self, retryable: bool) -> Self {
		self.retryable = retryable;
		self
	}

	/// Set retry delay
	#[must_use]
	pub fn retry_after(mut self, duration: std::time::Duration) -> Self {
		self.retry_after = Some(duration);
		self.retryable = true;
		self
	}

	/// Add documentation link
	#[must_use]
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

/// AWS-SDK-style metadata accessor for SDK errors.
///
/// This trait mirrors `aws_smithy_types::error::metadata::ProvideErrorMetadata`
/// from the AWS Rust SDK, giving callers a stable, vendor-neutral way to
/// extract a short error `code()` and human-readable `message()` for logging,
/// telemetry, and structured error reporting — without having to match on
/// every `#[non_exhaustive]` variant.
///
/// # Example
///
/// ```
/// use neo3::neo_error::unified::{NeoError, ProvideErrorMetadata};
///
/// let err = NeoError::network("connect", "boom");
/// assert_eq!(ProvideErrorMetadata::code(&err), Some("Network"));
/// assert!(ProvideErrorMetadata::message(&err).is_some());
/// ```
pub trait ProvideErrorMetadata {
	/// Returns a short, stable identifier for this error (e.g. `"Network"`,
	/// `"RateLimit"`). Returns `None` only if no classification is available
	/// — every variant of [`NeoError`] produces a `Some`.
	fn code(&self) -> Option<&str>;

	/// Returns the human-readable error message (without source chain or
	/// recovery hints). Returns `None` only if no message is available.
	fn message(&self) -> Option<&str>;
}

impl ProvideErrorMetadata for NeoError {
	fn code(&self) -> Option<&str> {
		Some(match self.kind() {
			NeoErrorKind::Network => "Network",
			NeoErrorKind::Wallet => "Wallet",
			NeoErrorKind::Contract => "Contract",
			NeoErrorKind::Transaction => "Transaction",
			NeoErrorKind::Configuration => "Configuration",
			NeoErrorKind::Validation => "Validation",
			NeoErrorKind::InsufficientFunds => "InsufficientFunds",
			NeoErrorKind::Timeout => "Timeout",
			NeoErrorKind::RateLimit => "RateLimit",
			NeoErrorKind::Other => "Other",
		})
	}

	fn message(&self) -> Option<&str> {
		Some(NeoError::message(self))
	}
}

/// Coarse classification of [`NeoError`] variants, modeled after the
/// AWS SDK's `ProvideErrorMetadata` / `ErrorKind` accessors.
///
/// Use this to route errors into telemetry buckets, structured logs, or
/// retry decisions without depending on the exact enum variant layout
/// (which is `#[non_exhaustive]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NeoErrorKind {
	/// Transport / RPC / connectivity failure.
	Network,
	/// Wallet, key material, or account-state problem.
	Wallet,
	/// Smart-contract invocation problem (VM fault, missing method, …).
	Contract,
	/// Transaction construction, signing, or submission problem.
	Transaction,
	/// Bad client/SDK configuration.
	Configuration,
	/// User-input validation failure.
	Validation,
	/// Account balance is too low for the requested operation.
	InsufficientFunds,
	/// Operation exceeded its deadline.
	Timeout,
	/// Provider applied a rate limit.
	RateLimit,
	/// Uncategorised / future-compatible bucket.
	Other,
}

impl NeoError {
	/// Returns the coarse [`NeoErrorKind`] for this error.
	///
	/// Stable across non-exhaustive variant additions — prefer this in
	/// `match` arms over destructuring the enum directly.
	pub fn kind(&self) -> NeoErrorKind {
		match self {
			NeoError::Network { .. } => NeoErrorKind::Network,
			NeoError::Wallet { .. } => NeoErrorKind::Wallet,
			NeoError::Contract { .. } => NeoErrorKind::Contract,
			NeoError::Transaction { .. } => NeoErrorKind::Transaction,
			NeoError::Configuration { .. } => NeoErrorKind::Configuration,
			NeoError::Validation { .. } => NeoErrorKind::Validation,
			NeoError::InsufficientFunds { .. } => NeoErrorKind::InsufficientFunds,
			NeoError::Timeout { .. } => NeoErrorKind::Timeout,
			NeoError::RateLimit { .. } => NeoErrorKind::RateLimit,
			NeoError::Other { .. } => NeoErrorKind::Other,
		}
	}

	/// Returns the recovery hints attached to the error, if any.
	pub fn recovery(&self) -> &ErrorRecovery {
		match self {
			NeoError::Network { recovery, .. }
			| NeoError::Wallet { recovery, .. }
			| NeoError::Contract { recovery, .. }
			| NeoError::Transaction { recovery, .. }
			| NeoError::Configuration { recovery, .. }
			| NeoError::Validation { recovery, .. }
			| NeoError::InsufficientFunds { recovery, .. }
			| NeoError::Timeout { recovery, .. }
			| NeoError::RateLimit { recovery, .. }
			| NeoError::Other { recovery, .. } => recovery,
		}
	}

	/// Whether the underlying operation can sensibly be retried.
	///
	/// Mirrors `aws-smithy-runtime-api::client::retries::classifiers::RetryAction`
	/// — returns `true` for transient failures (timeouts, rate limits, network
	/// blips, RPC 5xx), `false` for deterministic failures (validation, signing,
	/// insufficient funds).
	pub fn is_retryable(&self) -> bool {
		matches!(self, NeoError::RateLimit { .. } | NeoError::Timeout { .. })
			|| self.recovery().retryable
	}

	/// Suggested delay before retry, if the error carries one.
	pub fn retry_after(&self) -> Option<std::time::Duration> {
		match self {
			NeoError::RateLimit { retry_after, .. } => *retry_after,
			_ => self.recovery().retry_after,
		}
	}

	/// Returns the human-readable message portion of the error (no source / recovery).
	pub fn message(&self) -> &str {
		match self {
			NeoError::Network { message, .. }
			| NeoError::Wallet { message, .. }
			| NeoError::Contract { message, .. }
			| NeoError::Transaction { message, .. }
			| NeoError::Configuration { message, .. }
			| NeoError::Validation { message, .. }
			| NeoError::Timeout { operation: message, .. }
			| NeoError::RateLimit { message, .. }
			| NeoError::Other { message, .. } => message,
			NeoError::InsufficientFunds { token, .. } => token,
		}
	}
}

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

/// Convenience constructors used by the high-level `sdk` module to keep call
/// sites concise. Each helper attaches the most common recovery hints for the
/// situation so callers don't repeat the same boilerplate.
impl NeoError {
	/// Construct a [`NeoError::Network`] from any `Display`-able source error.
	///
	/// Marks the failure retryable and includes generic connectivity hints —
	/// matches the AWS SDK convention where transport failures are retryable
	/// by default.
	pub fn network<E: fmt::Display>(context: &str, err: E) -> Self {
		NeoError::Network {
			message: format!("{}: {}", context, err),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check network connectivity")
				.suggest("Verify the RPC endpoint is accessible")
				.retryable(true),
		}
	}

	/// Construct a [`NeoError::Transaction`] from any `Display`-able source error.
	pub fn transaction<E: fmt::Display>(context: &str, err: E) -> Self {
		NeoError::Transaction {
			message: format!("{}: {}", context, err),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new(),
		}
	}

	/// Construct a [`NeoError::Contract`] error tagged with the contract hash and method.
	pub fn contract<E: fmt::Display>(
		context: &str,
		contract: Option<String>,
		method: Option<String>,
		err: E,
	) -> Self {
		NeoError::Contract {
			message: format!("{}: {}", context, err),
			contract,
			method,
			source: None,
			recovery: ErrorRecovery::new(),
		}
	}

	/// Construct a [`NeoError::Validation`] error for an invalid input value.
	pub fn validation<V: Into<String>>(field: &str, value: Option<V>, message: &str) -> Self {
		NeoError::Validation {
			message: message.to_string(),
			field: field.to_string(),
			value: value.map(Into::into),
			recovery: ErrorRecovery::new().suggest("Check the input value"),
		}
	}

	/// Construct a [`NeoError::Wallet`] error from any `Display`-able source error.
	pub fn wallet<E: fmt::Display>(context: &str, err: E) -> Self {
		NeoError::Wallet {
			message: format!("{}: {}", context, err),
			source: None,
			recovery: ErrorRecovery::new(),
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
#[allow(dead_code)] // Variants used by ErrorBuilder; not all paths are wired up yet
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
#[allow(dead_code)] // Fields reserved for ErrorBuilder context; not all are consumed yet
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
	#[must_use]
	pub fn source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
		self.source = Some(Box::new(source));
		self
	}

	/// Add contract context
	#[must_use]
	pub fn with_contract(mut self, contract: impl Into<String>) -> Self {
		self.context.contract = Some(contract.into());
		self
	}

	/// Add method context
	#[must_use]
	pub fn with_method(mut self, method: impl Into<String>) -> Self {
		self.context.method = Some(method.into());
		self
	}

	/// Add recovery suggestion
	#[must_use]
	pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
		self.recovery = self.recovery.suggest(suggestion);
		self
	}

	/// Mark as retryable
	#[must_use]
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

impl From<crate::neo_fs::NeoFSError> for NeoError {
	fn from(err: crate::neo_fs::NeoFSError) -> Self {
		use crate::neo_fs::NeoFSError;
		match &err {
			NeoFSError::ConnectionError(msg) => NeoError::Network {
				message: format!("NeoFS connection error: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check NeoFS endpoint connectivity")
					.retryable(true),
			},
			NeoFSError::AuthenticationError(msg) => NeoError::Wallet {
				message: format!("NeoFS authentication error: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify NeoFS credentials")
					.suggest("Check that the session token is valid"),
			},
			NeoFSError::PermissionDenied(msg) => NeoError::Contract {
				message: format!("NeoFS permission denied: {}", msg),
				contract: Some("NeoFS".to_string()),
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check container ACL settings")
					.suggest("Verify the bearer token has sufficient permissions"),
			},
			NeoFSError::NotFound(msg) => NeoError::Other {
				message: format!("NeoFS resource not found: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the container or object ID is correct"),
			},
			NeoFSError::Timeout(msg) => NeoError::Timeout {
				duration: std::time::Duration::from_secs(0),
				operation: format!("NeoFS: {}", msg),
				recovery: ErrorRecovery::new()
					.suggest("Increase timeout or retry the operation")
					.retryable(true),
			},
			_ => NeoError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::neo_protocol::ProtocolError> for NeoError {
	fn from(err: crate::neo_protocol::ProtocolError) -> Self {
		use crate::neo_protocol::ProtocolError;
		match &err {
			ProtocolError::RpcResponse { error } => NeoError::Network {
				message: format!("RPC response error: {}", error),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the RPC request parameters")
					.retryable(true),
			},
			ProtocolError::InvocationFaultState { error } => NeoError::Contract {
				message: format!("VM fault: {}", error),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check contract parameters and state")
					.suggest("Verify the contract is deployed on the target network"),
			},
			ProtocolError::ClientConnection { message } => NeoError::Network {
				message: format!("Client connection error: {}", message),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check network connectivity")
					.retryable(true),
			},
			ProtocolError::IllegalState { message } => NeoError::Other {
				message: format!("Illegal state: {}", message),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			_ => NeoError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
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

	#[test]
	fn kind_returns_stable_variant_classification() {
		let err = NeoError::network("connect", "boom");
		assert_eq!(err.kind(), NeoErrorKind::Network);
		assert!(err.is_retryable());

		let err = NeoError::validation("amount", Some("-1"), "must be positive");
		assert_eq!(err.kind(), NeoErrorKind::Validation);
		assert!(!err.is_retryable());
	}

	#[test]
	fn retry_after_round_trips_for_rate_limit() {
		let err = NeoError::RateLimit {
			message: "throttled".to_string(),
			retry_after: Some(std::time::Duration::from_secs(2)),
			recovery: ErrorRecovery::new(),
		};
		assert!(err.is_retryable());
		assert_eq!(err.retry_after(), Some(std::time::Duration::from_secs(2)));
	}

	#[test]
	fn convenience_constructors_attach_recovery_hints() {
		let err = NeoError::contract(
			"invoke failed",
			Some("abc".into()),
			Some("transfer".into()),
			"vm fault",
		);
		match err {
			NeoError::Contract { contract, method, .. } => {
				assert_eq!(contract.as_deref(), Some("abc"));
				assert_eq!(method.as_deref(), Some("transfer"));
			},
			other => panic!("expected Contract variant, got {other:?}"),
		}
	}

	#[test]
	fn provide_error_metadata_exposes_code_and_message() {
		// Cover every kind end-to-end via the trait
		let cases: &[(&str, NeoError)] = &[
			("Network", NeoError::network("connect", "boom")),
			("Wallet", NeoError::wallet("decrypt", "bad key")),
			("Contract", NeoError::contract("invoke", None, None, "fault")),
			("Transaction", NeoError::transaction("sign", "no key")),
			(
				"Configuration",
				NeoError::Configuration {
					message: "missing endpoint".into(),
					field: None,
					recovery: ErrorRecovery::new(),
				},
			),
			("Validation", NeoError::validation("amount", Some("-1"), "must be positive")),
			(
				"InsufficientFunds",
				NeoError::InsufficientFunds {
					required: "1".into(),
					available: "0".into(),
					token: "GAS".into(),
					recovery: ErrorRecovery::new(),
				},
			),
			(
				"Timeout",
				NeoError::Timeout {
					duration: std::time::Duration::from_secs(1),
					operation: "wait".into(),
					recovery: ErrorRecovery::new(),
				},
			),
			(
				"RateLimit",
				NeoError::RateLimit {
					message: "throttled".into(),
					retry_after: None,
					recovery: ErrorRecovery::new(),
				},
			),
			(
				"Other",
				NeoError::Other {
					message: "misc".into(),
					source: None,
					recovery: ErrorRecovery::new(),
				},
			),
		];

		for (expected_code, err) in cases {
			assert_eq!(
				ProvideErrorMetadata::code(err),
				Some(*expected_code),
				"code mismatch for {expected_code}"
			);
			assert!(
				ProvideErrorMetadata::message(err).is_some(),
				"missing message for {expected_code}"
			);
		}
	}
}
