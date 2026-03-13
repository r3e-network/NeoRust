use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
	neo_clients::{JsonRpcProvider, RpcClient},
	neo_contract::ContractError,
};

/// Neo X EVM provider for interacting with the Neo X EVM-compatible chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoXProvider<'a, P: JsonRpcProvider> {
	rpc_url: String,
	#[serde(skip)]
	#[allow(dead_code)]
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, P: JsonRpcProvider + 'static> NeoXProvider<'a, P> {
	/// Creates a new NeoXProvider instance with the specified RPC URL
	///
	/// # Arguments
	///
	/// * `rpc_url` - The RPC URL for the Neo X chain
	/// * `provider` - An optional reference to an RPC client
	///
	/// # Returns
	///
	/// A new NeoXProvider instance
	pub fn new(rpc_url: &str, provider: Option<&'a RpcClient<P>>) -> Self {
		Self { rpc_url: rpc_url.to_string(), provider }
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
		let provider = self.provider.ok_or_else(|| {
			ContractError::ProviderNotSet(
				"Provider is required to query Neo X chain ID".to_string(),
			)
		})?;

		let value: Value = provider
			.request("neo_chainId", ())
			.await
			.map_err(ContractError::from)?;

		Self::parse_chain_id_value(value)
	}

	fn parse_chain_id_value(value: Value) -> Result<u64, ContractError> {
		match value {
			Value::String(raw) => {
				let trimmed = raw.trim();
				if let Some(hex) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
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
		let provider: NeoXProvider<'_, MockProvider> =
			NeoXProvider::new("https://rpc.neo-x.org", None);

		let err = provider.chain_id().await.unwrap_err();
		assert!(matches!(err, ContractError::ProviderNotSet(message) if message.contains("chain ID")));
	}

	#[test]
	fn parse_chain_id_value_rejects_invalid_strings() {
		let err = NeoXProvider::<MockProvider>::parse_chain_id_value(Value::String("wat".to_string()))
			.unwrap_err();
		assert!(matches!(err, ContractError::InvalidArgError(message) if message.contains("Invalid Neo X chain ID")));
	}
}
