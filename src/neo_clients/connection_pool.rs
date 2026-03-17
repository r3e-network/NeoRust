#![allow(dead_code)]

use crate::{
	neo_clients::{APITrait, HttpProvider, RpcClient},
	neo_error::{Neo3Error, Neo3Result},
};
use std::{
	collections::VecDeque,
	sync::Arc,
	time::{Duration, Instant},
};
use tokio::sync::{RwLock, Semaphore};

/// Configuration for connection pool
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PoolConfig {
	/// Maximum number of concurrent connections
	pub max_connections: usize,
	/// Minimum number of idle connections to maintain
	pub min_idle: usize,
	/// Maximum time a connection can be idle before being closed
	pub max_idle_time: Duration,
	/// Connection timeout
	pub connection_timeout: Duration,
	/// Request timeout
	pub request_timeout: Duration,
	/// Maximum number of retries for failed requests
	pub max_retries: u32,
	/// Delay between retries
	pub retry_delay: Duration,
}

impl PoolConfig {
	/// Creates a new builder for the configuration
	pub fn builder() -> PoolConfigBuilder {
		PoolConfigBuilder::default()
	}
}

/// Builder for `PoolConfig`
#[derive(Debug, Default, Clone)]
pub struct PoolConfigBuilder {
	max_connections: Option<usize>,
	min_idle: Option<usize>,
	max_idle_time: Option<Duration>,
	connection_timeout: Option<Duration>,
	request_timeout: Option<Duration>,
	max_retries: Option<u32>,
	retry_delay: Option<Duration>,
}

impl PoolConfigBuilder {
	/// Sets the maximum number of concurrent connections
	pub fn max_connections(mut self, val: usize) -> Self {
		self.max_connections = Some(val);
		self
	}

	/// Sets the minimum number of idle connections
	pub fn min_idle(mut self, val: usize) -> Self {
		self.min_idle = Some(val);
		self
	}

	/// Sets the maximum idle time
	pub fn max_idle_time(mut self, val: Duration) -> Self {
		self.max_idle_time = Some(val);
		self
	}

	/// Sets the connection timeout
	pub fn connection_timeout(mut self, val: Duration) -> Self {
		self.connection_timeout = Some(val);
		self
	}

	/// Sets the request timeout
	pub fn request_timeout(mut self, val: Duration) -> Self {
		self.request_timeout = Some(val);
		self
	}

	/// Sets the maximum number of retries
	pub fn max_retries(mut self, val: u32) -> Self {
		self.max_retries = Some(val);
		self
	}

	/// Sets the retry delay
	pub fn retry_delay(mut self, val: Duration) -> Self {
		self.retry_delay = Some(val);
		self
	}

	/// Builds the `PoolConfig`
	pub fn build(self) -> PoolConfig {
		let default = PoolConfig::default();
		PoolConfig {
			max_connections: self.max_connections.unwrap_or(default.max_connections),
			min_idle: self.min_idle.unwrap_or(default.min_idle),
			max_idle_time: self.max_idle_time.unwrap_or(default.max_idle_time),
			connection_timeout: self.connection_timeout.unwrap_or(default.connection_timeout),
			request_timeout: self.request_timeout.unwrap_or(default.request_timeout),
			max_retries: self.max_retries.unwrap_or(default.max_retries),
			retry_delay: self.retry_delay.unwrap_or(default.retry_delay),
		}
	}
}

impl Default for PoolConfig {
	fn default() -> Self {
		Self {
			max_connections: 10,
			min_idle: 2,
			max_idle_time: Duration::from_secs(300), // 5 minutes
			connection_timeout: Duration::from_secs(30),
			request_timeout: Duration::from_secs(60),
			max_retries: 3,
			retry_delay: Duration::from_millis(1000),
		}
	}
}

/// A pooled connection wrapper
#[derive(Debug)]
struct PooledConnection {
	client: RpcClient<HttpProvider>,
	created_at: Instant,
	last_used: Instant,
	is_healthy: bool,
}

impl PooledConnection {
	fn new(endpoint: &str) -> Neo3Result<Self> {
		let provider = HttpProvider::new(endpoint).map_err(|e| {
			Neo3Error::Network(crate::neo_error::NetworkError::ConnectionFailed(e.to_string()))
		})?;
		let client = RpcClient::new(provider);

		Ok(Self { client, created_at: Instant::now(), last_used: Instant::now(), is_healthy: true })
	}

	fn is_expired(&self, max_idle_time: Duration) -> bool {
		self.last_used.elapsed() > max_idle_time
	}

	fn mark_used(&mut self) {
		self.last_used = Instant::now();
	}

	async fn health_check(&mut self) -> bool {
		match self.client.get_version().await {
			Ok(_) => {
				self.is_healthy = true;
				true
			},
			Err(_) => {
				self.is_healthy = false;
				false
			},
		}
	}
}

/// High-performance connection pool for Neo RPC clients
pub struct ConnectionPool {
	config: PoolConfig,
	endpoint: String,
	connections: Arc<RwLock<VecDeque<PooledConnection>>>,
	semaphore: Arc<Semaphore>,
	stats: Arc<RwLock<PoolStats>>,
}

/// Connection pool statistics
#[derive(Debug, Default)]
pub struct PoolStats {
	pub total_connections_created: u64,
	pub total_requests: u64,
	pub successful_requests: u64,
	pub failed_requests: u64,
	pub retried_requests: u64,
	pub current_active_connections: usize,
	pub current_idle_connections: usize,
}

impl ConnectionPool {
	/// Create a new connection pool
	pub fn new(endpoint: String, config: PoolConfig) -> Self {
		let semaphore = Arc::new(Semaphore::new(config.max_connections));

		Self {
			config,
			endpoint,
			connections: Arc::new(RwLock::new(VecDeque::new())),
			semaphore,
			stats: Arc::new(RwLock::new(PoolStats::default())),
		}
	}

	/// Execute a request with automatic retry and connection management
	pub async fn execute<F, T>(&self, operation: F) -> Neo3Result<T>
	where
		F: Fn(
				&RpcClient<HttpProvider>,
			)
				-> std::pin::Pin<Box<dyn std::future::Future<Output = Neo3Result<T>> + Send + '_>>
			+ Send
			+ Sync,
		T: Send,
	{
		let _permit = self.semaphore.acquire().await.map_err(|_| {
			Neo3Error::Network(crate::neo_error::NetworkError::ConnectionFailed(
				"Failed to acquire connection permit".to_string(),
			))
		})?;

		let mut retries = 0;
		loop {
			// Update stats
			{
				let mut stats = self.stats.write().await;
				stats.total_requests += 1;
			}

			// Get or create connection
			let mut connection = self.get_connection().await?;

			// Execute operation with timeout
			let result =
				tokio::time::timeout(self.config.request_timeout, operation(&connection.client))
					.await;

			match result {
				Ok(Ok(value)) => {
					// Success - return connection to pool and return result
					connection.mark_used();
					self.return_connection(connection).await;

					let mut stats = self.stats.write().await;
					stats.successful_requests += 1;

					return Ok(value);
				},
				Ok(Err(e)) => {
					// Mark connection as unhealthy
					connection.is_healthy = false;

					if retries < self.config.max_retries {
						retries += 1;

						let mut stats = self.stats.write().await;
						stats.retried_requests += 1;

						tokio::time::sleep(self.config.retry_delay * retries).await;
						continue;
					} else {
						let mut stats = self.stats.write().await;
						stats.failed_requests += 1;

						return Err(e);
					}
				},
				Err(_) => {
					// Timeout - mark connection as unhealthy
					connection.is_healthy = false;

					if retries < self.config.max_retries {
						retries += 1;

						let mut stats = self.stats.write().await;
						stats.retried_requests += 1;

						tokio::time::sleep(self.config.retry_delay * retries).await;
						continue;
					} else {
						let mut stats = self.stats.write().await;
						stats.failed_requests += 1;

						return Err(Neo3Error::Network(crate::neo_error::NetworkError::Timeout));
					}
				},
			}
		}
	}

	/// Get a connection from the pool or create a new one
	async fn get_connection(&self) -> Neo3Result<PooledConnection> {
		// Try to get an existing connection
		{
			let mut connections = self.connections.write().await;
			while let Some(mut conn) = connections.pop_front() {
				if !conn.is_expired(self.config.max_idle_time) && conn.is_healthy {
					conn.mark_used();
					return Ok(conn);
				}
			}
		}

		// Create new connection
		let connection = PooledConnection::new(&self.endpoint)?;

		let mut stats = self.stats.write().await;
		stats.total_connections_created += 1;
		stats.current_active_connections += 1;

		Ok(connection)
	}

	/// Return a connection to the pool
	async fn return_connection(&self, connection: PooledConnection) {
		if connection.is_healthy && !connection.is_expired(self.config.max_idle_time) {
			let mut connections = self.connections.write().await;
			connections.push_back(connection);

			let mut stats = self.stats.write().await;
			stats.current_active_connections = stats.current_active_connections.saturating_sub(1);
			stats.current_idle_connections = connections.len();
		} else {
			let mut stats = self.stats.write().await;
			stats.current_active_connections = stats.current_active_connections.saturating_sub(1);
		}
	}

	/// Perform health checks on idle connections
	pub async fn health_check(&self) {
		// Drain connections while holding the lock briefly
		let pending: VecDeque<PooledConnection> = {
			let mut connections = self.connections.write().await;
			std::mem::take(&mut *connections)
		};

		// Run health checks without holding the pool lock
		let mut healthy_connections = VecDeque::new();
		for mut conn in pending {
			if conn.health_check().await {
				healthy_connections.push_back(conn);
			}
		}

		// Re-acquire lock to store healthy connections
		let idle_count = healthy_connections.len();
		{
			let mut connections = self.connections.write().await;
			*connections = healthy_connections;
		}

		let mut stats = self.stats.write().await;
		stats.current_idle_connections = idle_count;
	}

	/// Get current pool statistics
	pub async fn get_stats(&self) -> PoolStats {
		let stats = self.stats.read().await;
		PoolStats {
			total_connections_created: stats.total_connections_created,
			total_requests: stats.total_requests,
			successful_requests: stats.successful_requests,
			failed_requests: stats.failed_requests,
			retried_requests: stats.retried_requests,
			current_active_connections: stats.current_active_connections,
			current_idle_connections: stats.current_idle_connections,
		}
	}

	/// Close all connections and clean up the pool
	pub async fn close(&self) {
		let mut connections = self.connections.write().await;
		connections.clear();

		let mut stats = self.stats.write().await;
		stats.current_active_connections = 0;
		stats.current_idle_connections = 0;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::neo_protocol::NeoVersion;

	#[tokio::test]
	async fn test_pool_creation() {
		let config = PoolConfig::default();
		let pool = ConnectionPool::new("https://testnet.neo.org:443".to_string(), config);

		let stats = pool.get_stats().await;
		assert_eq!(stats.total_connections_created, 0);
		assert_eq!(stats.current_active_connections, 0);
	}

	#[tokio::test]
	async fn test_pool_stats() {
		let config = PoolConfig { max_connections: 2, ..Default::default() };
		let pool = ConnectionPool::new("https://testnet.neo.org:443".to_string(), config);

		// Execute a simple operation
		let _result =
			pool.execute(|_client| Box::pin(async move { Ok(NeoVersion::default()) })).await;

		// Check that stats were updated
		let stats = pool.get_stats().await;
		assert!(stats.total_requests > 0);
	}
}
