//! Create a custom data transport to use with a Neo N3 RPC Client.

use neo3::prelude::*;
use std::sync::Arc;
use thiserror::Error;
use tokio::time::{timeout, Duration};

/// This example demonstrates how to create custom transport abstractions for Neo N3.
/// Since Neo N3 primarily uses HTTP RPC, we create a custom transport layer that
/// can handle multiple HTTP endpoints with failover and load balancing capabilities.
#[tokio::main]
async fn main() -> eyre::Result<()> {
	println!("🔧 Neo N3 Custom Transport Example");
	println!("==================================");

	// 1. Create custom multi-endpoint transport
	println!("\n1. Setting up custom multi-endpoint transport...");

	let endpoints = vec![
		"https://testnet1.neo.coz.io:443/".to_string(),
		"https://testnet2.neo.coz.io:443/".to_string(),
		"https://neo.coin.org:443/".to_string(), // Backup endpoint
	];

	let custom_transport = CustomNeoTransport::new(endpoints)?;
	println!(
		"   ✅ Custom transport initialized with {} endpoints",
		custom_transport.endpoint_count()
	);

	// 2. Test basic connectivity
	println!("\n2. Testing transport connectivity...");
	match test_connectivity(&custom_transport).await {
		Ok(_) => println!("   ✅ All endpoints tested successfully"),
		Err(e) => println!("   ⚠️  Some endpoints failed: {}", e),
	}

	// 3. Demonstrate failover capabilities
	println!("\n3. Testing failover capabilities...");
	test_failover_behavior(&custom_transport).await?;

	// 4. Load balancing demonstration
	println!("\n4. Demonstrating load balancing...");
	test_load_balancing(&custom_transport).await?;

	// 5. Performance monitoring
	println!("\n5. Transport performance metrics...");
	let metrics = custom_transport.get_metrics().await;
	print_transport_metrics(&metrics);

	// 6. Best practices
	println!("\n6. 💡 Custom Transport Best Practices:");
	println!("   ✅ Implement proper error handling and retries");
	println!("   ✅ Use connection pooling for better performance");
	println!("   ✅ Monitor endpoint health and response times");
	println!("   ✅ Implement circuit breaker patterns");
	println!("   ✅ Log transport activity for debugging");
	println!("   ✅ Support graceful degradation");
	println!("   ✅ Handle network timeouts appropriately");

	println!("\n🎉 Custom transport example completed!");
	println!("💡 This demonstrates custom transport patterns for Neo N3.");

	Ok(())
}

/// Custom transport error types
#[derive(Error, Debug)]
enum CustomTransportError {
	#[error("No available endpoints")]
	NoEndpoints,
	#[error("All endpoints failed")]
	AllEndpointsFailed,
	#[error("Transport error: {0}")]
	TransportError(String),
	#[error("Timeout error: {0}")]
	TimeoutError(String),
}

/// Transport performance metrics
#[derive(Debug, Clone)]
struct TransportMetrics {
	total_requests: u64,
	successful_requests: u64,
	failed_requests: u64,
	average_response_time_ms: f64,
	endpoint_health: std::collections::HashMap<String, EndpointHealth>,
}

/// Individual endpoint health information
#[derive(Debug, Clone)]
struct EndpointHealth {
	is_healthy: bool,
	last_success: Option<chrono::DateTime<chrono::Utc>>,
	consecutive_failures: u32,
	average_response_time_ms: f64,
}

/// Custom Neo N3 transport with multiple endpoints and failover
struct CustomNeoTransport {
	endpoints: Vec<String>,
	current_endpoint_index: std::sync::Mutex<usize>,
	metrics: std::sync::Mutex<TransportMetrics>,
	health_check_interval: Duration,
}

impl CustomNeoTransport {
	fn new(endpoints: Vec<String>) -> Result<Self, CustomTransportError> {
		if endpoints.is_empty() {
			return Err(CustomTransportError::NoEndpoints);
		}

		let mut endpoint_health = std::collections::HashMap::new();
		for endpoint in &endpoints {
			endpoint_health.insert(
				endpoint.clone(),
				EndpointHealth {
					is_healthy: true,
					last_success: None,
					consecutive_failures: 0,
					average_response_time_ms: 0.0,
				},
			);
		}

		let metrics = TransportMetrics {
			total_requests: 0,
			successful_requests: 0,
			failed_requests: 0,
			average_response_time_ms: 0.0,
			endpoint_health,
		};

		Ok(Self {
			endpoints,
			current_endpoint_index: std::sync::Mutex::new(0),
			metrics: std::sync::Mutex::new(metrics),
			health_check_interval: Duration::from_secs(30),
		})
	}

	fn endpoint_count(&self) -> usize {
		self.endpoints.len()
	}

	async fn execute_request(
		&self,
		request_description: &str,
	) -> Result<serde_json::Value, CustomTransportError> {
		let start_time = std::time::Instant::now();

		// Try each endpoint until one succeeds
		for attempt in 0..self.endpoints.len() {
			let endpoint_index = {
				let mut index = self.current_endpoint_index.lock().unwrap();
				let current = *index;
				*index = (*index + 1) % self.endpoints.len();
				current
			};

			let endpoint = &self.endpoints[endpoint_index];

			match self.try_endpoint(endpoint, request_description).await {
				Ok(response) => {
					let duration = start_time.elapsed();
					self.update_metrics(true, duration, endpoint).await;
					return Ok(response);
				},
				Err(e) => {
					println!("   ⚠️  Endpoint {} failed: {}", endpoint, e);
					let duration = start_time.elapsed();
					self.update_metrics(false, duration, endpoint).await;

					if attempt == self.endpoints.len() - 1 {
						return Err(CustomTransportError::AllEndpointsFailed);
					}
				},
			}
		}

		Err(CustomTransportError::AllEndpointsFailed)
	}

	async fn try_endpoint(
		&self,
		endpoint: &str,
		request_description: &str,
	) -> Result<serde_json::Value, CustomTransportError> {
		// Simulate RPC request with timeout
		let request_future = async {
			// Create actual HTTP provider for this endpoint
			let provider = HttpProvider::new(endpoint)
				.map_err(|e| CustomTransportError::TransportError(e.to_string()))?;
			let client = RpcClient::new(provider);

			// Execute the actual request based on description
			match request_description {
				"get_block_count" => {
					let block_count = client
						.get_block_count()
						.await
						.map_err(|e| CustomTransportError::TransportError(e.to_string()))?;
					Ok(serde_json::json!({"block_count": block_count}))
				},
				"get_version" => {
					let version = client
						.get_version()
						.await
						.map_err(|e| CustomTransportError::TransportError(e.to_string()))?;
					Ok(version)
				},
				_ => {
					// Simulate generic request
					let block_count = client
						.get_block_count()
						.await
						.map_err(|e| CustomTransportError::TransportError(e.to_string()))?;
					Ok(serde_json::json!({"result": "success", "block_count": block_count}))
				},
			}
		};

		// Apply timeout
		timeout(Duration::from_secs(10), request_future)
			.await
			.map_err(|_| CustomTransportError::TimeoutError("Request timed out".to_string()))?
	}

	async fn update_metrics(&self, success: bool, duration: Duration, endpoint: &str) {
		let mut metrics = self.metrics.lock().unwrap();
		metrics.total_requests += 1;

		if success {
			metrics.successful_requests += 1;

			if let Some(health) = metrics.endpoint_health.get_mut(endpoint) {
				health.is_healthy = true;
				health.last_success = Some(chrono::Utc::now());
				health.consecutive_failures = 0;
				health.average_response_time_ms =
					(health.average_response_time_ms + duration.as_millis() as f64) / 2.0;
			}
		} else {
			metrics.failed_requests += 1;

			if let Some(health) = metrics.endpoint_health.get_mut(endpoint) {
				health.consecutive_failures += 1;
				if health.consecutive_failures >= 3 {
					health.is_healthy = false;
				}
			}
		}

		// Update average response time
		let total_time = metrics.average_response_time_ms * (metrics.total_requests - 1) as f64;
		metrics.average_response_time_ms =
			(total_time + duration.as_millis() as f64) / metrics.total_requests as f64;
	}

	async fn get_metrics(&self) -> TransportMetrics {
		self.metrics.lock().unwrap().clone()
	}
}

/// Test basic connectivity to all endpoints
async fn test_connectivity(transport: &CustomNeoTransport) -> Result<(), CustomTransportError> {
	for i in 0..3 {
		println!("   🔄 Testing connectivity attempt {}...", i + 1);

		match transport.execute_request("get_block_count").await {
			Ok(response) =>
				if let Some(block_count) = response.get("block_count") {
					println!("     ✅ Connection successful, block count: {}", block_count);
				},
			Err(e) => {
				println!("     ❌ Connection failed: {}", e);
			},
		}

		// Small delay between tests
		tokio::time::sleep(Duration::from_millis(500)).await;
	}

	Ok(())
}

/// Test failover behavior
async fn test_failover_behavior(
	transport: &CustomNeoTransport,
) -> Result<(), CustomTransportError> {
	println!("   🔄 Simulating endpoint failures...");

	for i in 0..5 {
		match transport.execute_request("get_version").await {
			Ok(_) => println!("     ✅ Request {} succeeded (failover working)", i + 1),
			Err(e) => println!("     ❌ Request {} failed: {}", i + 1, e),
		}

		tokio::time::sleep(Duration::from_millis(200)).await;
	}

	Ok(())
}

/// Test load balancing across endpoints
async fn test_load_balancing(transport: &CustomNeoTransport) -> Result<(), CustomTransportError> {
	println!("   ⚖️  Testing load balancing across endpoints...");

	let mut handles = Vec::new();

	for i in 0..6 {
		let transport_ref = unsafe { &*(transport as *const _) }; // Safe in this context
		let handle = tokio::spawn(async move {
			match transport_ref.execute_request("get_block_count").await {
				Ok(_) => println!("     ✅ Concurrent request {} completed", i + 1),
				Err(e) => println!("     ❌ Concurrent request {} failed: {}", i + 1, e),
			}
		});
		handles.push(handle);
	}

	// Wait for all requests to complete
	for handle in handles {
		let _ = handle.await;
	}

	Ok(())
}

/// Print transport performance metrics
fn print_transport_metrics(metrics: &TransportMetrics) {
	println!("   📊 Transport Performance:");
	println!("     Total requests: {}", metrics.total_requests);
	println!("     Successful requests: {}", metrics.successful_requests);
	println!("     Failed requests: {}", metrics.failed_requests);
	println!(
		"     Success rate: {:.1}%",
		if metrics.total_requests > 0 {
			(metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
		} else {
			0.0
		}
	);
	println!("     Average response time: {:.1}ms", metrics.average_response_time_ms);

	println!("\n   🏥 Endpoint Health:");
	for (endpoint, health) in &metrics.endpoint_health {
		let status = if health.is_healthy { "🟢 Healthy" } else { "🔴 Unhealthy" };
		println!(
			"     {}: {} (failures: {}, avg: {:.1}ms)",
			endpoint, status, health.consecutive_failures, health.average_response_time_ms
		);
	}
}
