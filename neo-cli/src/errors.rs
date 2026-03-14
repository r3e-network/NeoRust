use thiserror::Error;

/// Unified error type for the Neo CLI.
///
/// Each variant represents a distinct error category. Previously there were
/// duplicate pairs (e.g. `Network` / `NetworkError`, `Rpc` / `RpcError`,
/// `Io` / `IoError`). These have been consolidated into single canonical
/// variants to keep pattern matching unambiguous.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CliError {
	#[error("Invalid argument: {0} - {1}")]
	InvalidArgument(String, String),

	#[error("Wallet error: {0}")]
	Wallet(String),

	#[error("File system error: {0}")]
	FileSystem(String),

	#[error("JSON error: {0}")]
	Json(String),

	#[error("Network error: {0}")]
	Network(String),

	#[error("Transaction error: {0}")]
	Transaction(String),

	#[error("RPC error: {0}")]
	Rpc(String),

	#[error("NeoFS error: {0}")]
	NeoFS(String),

	#[error("Contract error: {0}")]
	Contract(String),

	#[error("Authentication error: {0}")]
	Authentication(String),

	#[error("Security error: {0}")]
	Security(String),

	#[error("Timeout error: {0}")]
	Timeout(String),

	#[error("Not implemented: {0}")]
	NotImplemented(String),

	#[error("Configuration error: {0}")]
	Config(String),

	#[error("Input error: {0}")]
	Input(String),

	#[error("SDK error: {0}")]
	Sdk(String),

	#[error("Builder error: {0}")]
	Builder(String),

	#[error("Unknown error: {0}")]
	Unknown(String),

	#[error("Invalid input: {0}")]
	InvalidInput(String),

	#[error("Invalid format: {0}")]
	InvalidFormat(String),

	#[error("Provider error: {0}")]
	Provider(String),

	#[error("External error: {0}")]
	External(String),

	#[error("User cancelled: {0}")]
	UserCancelled(String),

	#[error("Transaction failed: {0}")]
	TransactionFailed(String),

	#[error("No RPC client connected")]
	NoRpcClient,

	#[error("No account loaded")]
	NoAccount,

	#[error("No wallet loaded")]
	NoWallet,

	#[error("Wallet not loaded: {0}")]
	WalletNotLoaded(String),

	#[error("Wallet operation failed: {0}")]
	WalletOperation(String),

	#[error("Invalid operation: {0}")]
	InvalidOperation(String),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Serde JSON error: {0}")]
	SerdeJson(#[from] serde_json::Error),

	#[error("Reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("Tokio join error: {0}")]
	TokioJoin(#[from] tokio::task::JoinError),

	#[error("Error: {0}")]
	Other(String),
}

// Implement From trait for String
impl From<String> for CliError {
	fn from(error: String) -> Self {
		CliError::Other(error)
	}
}

// Implement From trait for &str
impl From<&str> for CliError {
	fn from(error: &str) -> Self {
		CliError::Other(error.to_string())
	}
}

// Implement From trait for BuilderError
impl From<neo3::neo_builder::BuilderError> for CliError {
	fn from(error: neo3::neo_builder::BuilderError) -> Self {
		CliError::Builder(error.to_string())
	}
}

// Implement From trait for TransactionError
impl From<neo3::neo_builder::TransactionError> for CliError {
	fn from(error: neo3::neo_builder::TransactionError) -> Self {
		CliError::Transaction(error.to_string())
	}
}

// Implement From trait for ProviderError
impl From<neo3::neo_clients::ProviderError> for CliError {
	fn from(error: neo3::neo_clients::ProviderError) -> Self {
		CliError::Sdk(error.to_string())
	}
}
