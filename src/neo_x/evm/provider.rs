use ethers::providers::{Http, Middleware, Provider};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::{
	neo_clients::{JsonRpcProvider, RpcClient},
	neo_contract::ContractError,
};

/// Known Anti-MEV endpoints for Neo X (placeholder/example for MEV-protected RPCs)
pub const NEO_X_MAINNET_MEV_RPC: &str = "https://rpc.neo-x.org/mempool"; // Example URL

/// Neo X EVM provider for interacting with the Neo X EVM-compatible chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoXProvider<'a, P: JsonRpcProvider> {
	rpc_url: String,
	#[serde(skip)]
	#[allow(dead_code)]
	provider: Option<&'a RpcClient<P>>,
	#[serde(skip)]
	evm_provider: Option<Arc<Provider<Http>>>,
}

impl<'a, P: JsonRpcProvider + 'static> NeoXProvider<'a, P> {
	/// Creates a new NeoXProvider instance with the specified RPC URL
	///
	/// # Arguments
	///
	/// * `rpc_url` - The RPC URL for the Neo X chain
	/// * `provider` - An optional reference to an RPC client (Neo N3)
	///
	/// # Returns
	///
	/// A new NeoXProvider instance
	pub fn new(rpc_url: &str, provider: Option<&'a RpcClient<P>>) -> Self {
		let evm_provider = Provider::<Http>::try_from(rpc_url).map(Arc::new).ok();
		Self { rpc_url: rpc_url.to_string(), provider, evm_provider }
	}

	/// Creates a new NeoXProvider instance configured for Anti-MEV
	/// Uses a protected RPC endpoint that obfuscates transaction ordering.
	pub fn new_anti_mev(provider: Option<&'a RpcClient<P>>) -> Self {
		Self::new(NEO_X_MAINNET_MEV_RPC, provider)
	}

	/// Gets the RPC URL for the Neo X chain
	///
	/// # Returns
	///
	/// The RPC URL as a string
	pub fn rpc_url(&self) -> &str {
		&self.rpc_url
	}

	/// Sets the RPC URL for the Neo X chain
	///
	/// # Arguments
	///
	/// * `rpc_url` - The new RPC URL
	pub fn set_rpc_url(&mut self, rpc_url: &str) {
		self.rpc_url = rpc_url.to_string();
		self.evm_provider = Provider::<Http>::try_from(rpc_url).map(Arc::new).ok();
	}

	/// Returns the underlying Ethers provider for advanced EVM operations
	pub fn evm_provider(&self) -> Option<Arc<Provider<Http>>> {
		self.evm_provider.clone()
	}

	/// Gets the chain ID for the Neo X chain
	///
	/// # Returns
	///
	/// The chain ID as a u64
	///
	/// # Errors
	///
	/// Returns ContractError if the RPC call fails or if no provider is configured
	pub async fn chain_id(&self) -> Result<u64, ContractError> {
		// Prefer standard EVM RPC if evm_provider is available
		if let Some(evm) = &self.evm_provider {
			if let Ok(id) = evm.get_chainid().await {
				return Ok(id.as_u64());
			}
		}

		let provider = self.provider.ok_or_else(|| {
			ContractError::ProviderNotSet(
				"Provider is required to query Neo X chain ID".to_string(),
			)
		})?;

		let value: Value =
			provider.request("neo_chainId", ()).await.map_err(ContractError::from)?;

		Self::parse_chain_id_value(value)
	}

	fn parse_chain_id_value(value: Value) -> Result<u64, ContractError> {
		match value {
			Value::String(raw) => {
				let trimmed = raw.trim();
				if let Some(hex) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X"))
				{
					u64::from_str_radix(hex, 16).map_err(|_| {
						ContractError::InvalidArgError(format!(
							"Invalid Neo X chain ID hex value: {}",
							trimmed
						))
					})
				} else {
					trimmed.parse::<u64>().map_err(|_| {
						ContractError::InvalidArgError(format!(
							"Invalid Neo X chain ID value: {}",
							trimmed
						))
					})
				}
			},
			Value::Number(number) => number.as_u64().ok_or_else(|| {
				ContractError::InvalidArgError(
					"Neo X chain ID number is not representable as u64".to_string(),
				)
			}),
			other => Err(ContractError::InvalidArgError(format!(
				"Unexpected Neo X chain ID response: {}",
				other
			))),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::neo_clients::{MockProvider, RpcClient};

	#[tokio::test]
	async fn chain_id_queries_provider() {
		let provider = MockProvider::new();
		provider.push_result("neo_chainId", Value::String("0xba93".to_string()));
		let client = RpcClient::new(provider);
		let provider = NeoXProvider::new("https://rpc.neo-x.org", Some(&client));

		let chain_id = provider.chain_id().await.unwrap();
		assert_eq!(chain_id, 47763);
	}

	#[tokio::test]
	async fn chain_id_requires_provider() {
		let provider: NeoXProvider<'_, MockProvider> = NeoXProvider::new("invalid://url", None);

		let err = provider.chain_id().await.unwrap_err();
		assert!(
			matches!(err, ContractError::ProviderNotSet(message) if message.contains("chain ID"))
		);
	}

	#[test]
	fn parse_chain_id_value_rejects_invalid_strings() {
		let err =
			NeoXProvider::<MockProvider>::parse_chain_id_value(Value::String("wat".to_string()))
				.unwrap_err();
		assert!(
			matches!(err, ContractError::InvalidArgError(message) if message.contains("Invalid Neo X chain ID"))
		);
	}
}
