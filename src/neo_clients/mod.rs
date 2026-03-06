#![allow(clippy::type_complexity)]
#![allow(missing_docs)]
#![deny(unsafe_code)]

//! # Neo Clients
//!
//! Client interfaces for interacting with Neo N3 blockchain nodes.
//!
//! ## Overview
//!
//! The neo_clients module provides a comprehensive set of client interfaces for connecting to
//! and interacting with Neo N3 blockchain nodes. It includes:
//!
//! - RPC clients for making JSON-RPC calls to Neo nodes
//! - Multiple transport providers (HTTP, WebSocket, IPC)
//! - Subscription support for real-time blockchain events
//! - Mock clients for testing
//! - Extension traits for domain-specific functionality
//! - Error handling for network and protocol issues
//!
//! The module is designed to be flexible, allowing developers to choose the appropriate
//! client implementation and transport mechanism for their specific use case.
//!
//! ## Examples
//!
//! ### Connecting to a Neo N3 node using HTTP
//!
//! ```rust,no_run
//! use neo3::neo_clients::{HttpProvider, RpcClient, APITrait};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an HTTP provider connected to a Neo N3 TestNet node
//!     let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
//!     
//!     // Create an RPC client with the provider
//!     let client = RpcClient::new(provider);
//!     
//!     // Get the current block count
//!     let block_count = client.get_block_count().await?;
//!     println!("Current block count: {}", block_count);
//!     
//!     // Get information about the blockchain
//!     let version = client.get_version().await?;
//!     println!("Node version: {}", version.user_agent);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Using WebSocket for real-time updates
//!
//! ```ignore
//! # #[cfg(feature = "ws")]
//! use neo3::neo_clients::{Ws, RpcClient, APITrait};
//!
//! # #[cfg(feature = "ws")]
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to a Neo N3 node using WebSocket
//!     let ws = Ws::connect("wss://testnet1.neo.org:443/ws").await?;
//!     let client = RpcClient::new(ws);
//!     
//!     // Get basic blockchain information over WebSocket
//!     let block_count = client.get_block_count().await?;
//!     println!("Current block count: {}", block_count);
//!     
//!     let version = client.get_version().await?;
//!     println!("Node version: {}", version.user_agent);
//!     
//!     Ok(())
//! }
//! ```

use lazy_static::lazy_static;

use crate::config::NeoConstants;

/// Common interface for JSON-RPC transport errors so callers can inspect structured responses.
pub trait RpcError: std::error::Error + Send + Sync {
	/// Returns the underlying JSON-RPC error if available.
	fn as_error_response(&self) -> Option<&rpc::JsonRpcError> {
		None
	}

	/// Returns the underlying serde_json error if available.
	fn as_serde_error(&self) -> Option<&serde_json::Error> {
		None
	}
}
pub use api_trait::*;
pub use cache::{Cache, CacheConfig, CacheStats, RpcCache};
pub use circuit_breaker::{
	CircuitBreaker, CircuitBreakerConfig, CircuitBreakerStats, CircuitState,
};
pub use connection_pool::{ConnectionPool, PoolConfig, PoolStats};
pub use errors::ProviderError;
pub use ext::*;
#[cfg(any(test, feature = "mock"))]
pub use mock_client::MockClient;
pub use production_client::{ProductionClientConfig, ProductionClientStats, ProductionRpcClient};
pub use rate_limiter::{RateLimitPermit, RateLimiter, RateLimiterBuilder, RateLimiterPresets};
pub use rpc::*;
#[deprecated(
	note = "Use `HttpProvider::new(...)`/`RpcClient::new(...)` or the high-level `sdk::Neo` builder instead.",
	since = "1.0.1"
)]
pub use test_provider::{MAINNET, TESTNET};
pub use utils::*;

mod api_trait;
mod cache;
mod circuit_breaker;
mod connection_pool;
/// Errors
mod errors;
mod ext;
mod mock_blocks;
#[cfg(any(test, feature = "mock"))]
mod mock_client;
mod production_client;
mod rate_limiter;
mod rpc;
mod rx;
/// Crate utilities and type aliases
mod utils;

fn rpc_client_from_parsed_url(url: url::Url) -> RpcClient<Http> {
	let http_provider = match Http::new(url) {
		Ok(provider) => provider,
		Err(never) => match never {},
	};
	RpcClient::new(http_provider)
}

fn default_http_provider_client() -> RpcClient<Http> {
	let url =
		url::Url::parse(NeoConstants::SEED_1).expect("NeoConstants::SEED_1 must be a valid URL");
	rpc_client_from_parsed_url(url)
}

pub fn try_http_provider_from_endpoint(endpoint: &str) -> Result<RpcClient<Http>, ProviderError> {
	let url = url::Url::parse(endpoint).map_err(|e| {
		ProviderError::ParseError(format!("Failed to parse endpoint URL '{}': {}", endpoint, e))
	})?;
	Ok(rpc_client_from_parsed_url(url))
}

pub fn try_http_provider_from_env() -> Result<RpcClient<Http>, ProviderError> {
	let endpoint = std::env::var("ENDPOINT").unwrap_or_else(|_| NeoConstants::SEED_1.to_string());
	try_http_provider_from_endpoint(&endpoint)
}

lazy_static! {
	pub static ref HTTP_PROVIDER: RpcClient<Http> =
		try_http_provider_from_env().unwrap_or_else(|err| {
			tracing::warn!(
				error = %err,
				"Failed to create HTTP provider from ENDPOINT; falling back to default seed URL"
			);
			default_http_provider_client()
		});
}

#[allow(missing_docs)]
/// Deprecated pre-instantiated providers for quick-start/testing.
///
/// Prefer explicit configuration via `HttpProvider::new(...)`, `RpcClient::new(...)`,
/// or the high-level `sdk::Neo` builder.
mod test_provider {
	use std::{iter::Cycle, slice::Iter, sync::Mutex};

	use once_cell::sync::Lazy;

	use super::*;

	// Public endpoints to rotate through to reduce rate limit pressure.
	//
	// These are best-effort defaults intended for local development and examples.
	const MAINNET_ENDPOINTS: &[&str] = &[
		NeoConstants::SEED_1,
		NeoConstants::SEED_2,
		NeoConstants::SEED_3,
		NeoConstants::SEED_4,
		NeoConstants::SEED_5,
	];
	const TESTNET_ENDPOINTS: &[&str] =
		&["https://testnet1.neo.org:443", "https://testnet2.neo.org:443"];

	pub static MAINNET: Lazy<TestProvider> =
		Lazy::new(|| TestProvider::new(MAINNET_ENDPOINTS, "mainnet"));

	pub static TESTNET: Lazy<TestProvider> =
		Lazy::new(|| TestProvider::new(TESTNET_ENDPOINTS, "testnet"));

	#[derive(Debug)]
	pub struct TestProvider {
		network: String,
		endpoints: Mutex<Cycle<Iter<'static, &'static str>>>,
	}

	impl TestProvider {
		pub fn new(endpoints: &'static [&'static str], network: impl Into<String>) -> Self {
			Self { endpoints: endpoints.iter().cycle().into(), network: network.into() }
		}

		pub fn url(&self) -> String {
			let Self { network, endpoints } = self;
			let endpoint = endpoints.lock().unwrap_or_else(|e| e.into_inner()).next().copied();
			match endpoint {
				Some(endpoint) => endpoint.to_string(),
				None => {
					tracing::warn!(
						network = %network,
						"Endpoint list is empty; falling back to default seed URL"
					);
					NeoConstants::SEED_1.to_string()
				},
			}
		}

		pub fn provider(&self) -> RpcClient<Http> {
			let url_str = self.url();
			try_http_provider_from_endpoint(&url_str).unwrap_or_else(|err| {
				tracing::warn!(
					error = %err,
					endpoint = %url_str,
					"Failed to create HTTP provider; falling back to default seed URL"
				);
				default_http_provider_client()
			})
		}

		#[cfg(feature = "ws")]
		pub async fn ws(&self) -> Result<RpcClient<Ws>, ProviderError> {
			let url_str = self.url();
			let mut url = url::Url::parse(&url_str).unwrap_or_else(|e| {
				tracing::warn!(
					error = %e,
					endpoint = %url_str,
					"Failed to parse endpoint URL; falling back to default seed URL"
				);
				url::Url::parse(NeoConstants::SEED_1)
					.expect("NeoConstants::SEED_1 must be a valid URL")
			});

			let scheme = match url.scheme() {
				"https" => "wss",
				"http" => "ws",
				"wss" => "wss",
				"ws" => "ws",
				_ => "ws",
			};
			url.set_scheme(scheme).map_err(|_| {
				ProviderError::ParseError(format!(
					"Unsupported URL scheme in endpoint: {}",
					url_str
				))
			})?;
			if url.path() == "/" {
				url.set_path("/ws");
			}

			RpcClient::connect(url.as_str()).await
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_try_http_provider_from_endpoint_rejects_invalid_url() {
		assert!(matches!(
			try_http_provider_from_endpoint("not-a-url"),
			Err(ProviderError::ParseError(message)) if message.contains("not-a-url")
		));
	}

	#[test]
	fn test_try_http_provider_from_endpoint_accepts_valid_url() {
		let client = try_http_provider_from_endpoint("http://localhost:10332/").unwrap();
		assert_eq!(client.url().as_str(), "http://localhost:10332/");
	}
}
