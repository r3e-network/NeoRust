/// Neo N3 Retry Strategy Example
///
/// This example demonstrates how to implement retry patterns for
/// Neo N3 blockchain RPC calls to handle network issues gracefully.
use std::time::Duration;
use tokio::time::sleep;

/// Retry policy configuration
#[derive(Clone, Debug)]
pub struct RetryPolicy {
	pub max_retries: u32,
	pub initial_delay: Duration,
	pub max_delay: Duration,
	pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
	fn default() -> Self {
		Self {
			max_retries: 3,
			initial_delay: Duration::from_millis(100),
			max_delay: Duration::from_secs(30),
			backoff_multiplier: 2.0,
		}
	}
}

/// Error types that can occur during RPC calls
#[derive(Debug)]
pub enum RpcError {
	NetworkError(String),
	TimeoutError,
	RateLimitError,
	ServerError(u16),
	ParseError(String),
}

impl std::fmt::Display for RpcError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RpcError::NetworkError(msg) => write!(f, "Network error: {}", msg),
			RpcError::TimeoutError => write!(f, "Request timeout"),
			RpcError::RateLimitError => write!(f, "Rate limit exceeded"),
			RpcError::ServerError(code) => write!(f, "Server error: {}", code),
			RpcError::ParseError(msg) => write!(f, "Parse error: {}", msg),
		}
	}
}

impl std::error::Error for RpcError {}

/// Retry client wrapper for Neo N3 RPC calls
pub struct RetryClient {
	policy: RetryPolicy,
}

impl RetryClient {
	pub fn new(policy: RetryPolicy) -> Self {
		Self { policy }
	}

	/// Execute an operation with retry logic
	pub async fn execute_with_retry<F, T, E>(&self, operation: F) -> Result<T, RpcError>
	where
		F: Fn() -> Result<T, E> + Send + Sync,
		E: std::error::Error + Send + Sync + 'static,
	{
		let mut attempts = 0;
		let mut delay = self.policy.initial_delay;

		loop {
			match operation() {
				Ok(result) => return Ok(result),
				Err(e) => {
					attempts += 1;

					if attempts > self.policy.max_retries {
						return Err(RpcError::NetworkError(format!(
							"Max retries exceeded: {}",
							e
						)));
					}

					// Check if error is retryable
					if !self.is_retryable_error(&e) {
						return Err(RpcError::NetworkError(format!("Non-retryable error: {}", e)));
					}

					println!(
						"   ⚠️  Attempt {} failed: {}. Retrying in {:?}...",
						attempts, e, delay
					);

					sleep(delay).await;

					// Exponential backoff
					delay = std::cmp::min(
						Duration::from_secs_f64(delay.as_secs_f64() * self.policy.backoff_multiplier),
						self.policy.max_delay,
					);
				}
			}
		}
	}

	/// Determine if an error is retryable
	fn is_retryable_error<E: std::error::Error>(&self, error: &E) -> bool {
		let error_str = error.to_string().to_lowercase();

		// Common retryable error patterns
		error_str.contains("timeout")
			|| error_str.contains("connection")
			|| error_str.contains("network")
			|| error_str.contains("temporary")
			|| error_str.contains("rate limit")
			|| error_str.contains("502")
			|| error_str.contains("503")
			|| error_str.contains("504")
	}
}

/// Mock RPC operations for demonstration
struct MockRpcOperation {
	success_rate: f64,
	call_count: std::sync::Mutex<u32>,
}

impl MockRpcOperation {
	fn new(success_rate: f64) -> Self {
		Self { success_rate, call_count: std::sync::Mutex::new(0) }
	}

	fn call(&self) -> Result<String, RpcError> {
		let mut count = self.call_count.lock().unwrap();
		*count += 1;

		// Simulate random failures based on success rate
		let random_value = (*count as f64 * 0.3) % 1.0; // Deterministic "random"

		if random_value < self.success_rate {
			Ok(format!("Success after {} attempts", *count))
		} else if random_value < self.success_rate + 0.3 {
			Err(RpcError::NetworkError("Connection refused".to_string()))
		} else if random_value < self.success_rate + 0.5 {
			Err(RpcError::TimeoutError)
		} else {
			Err(RpcError::RateLimitError)
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🔄 Neo N3 Retry Strategy Example");
	println!("================================");

	// 1. Basic retry configuration
	println!("\n1. Retry Policy Configuration:");

	let policies = vec![
		("Conservative", RetryPolicy {
			max_retries: 2,
			initial_delay: Duration::from_millis(500),
			max_delay: Duration::from_secs(5),
			backoff_multiplier: 1.5,
		}),
		("Aggressive", RetryPolicy {
			max_retries: 5,
			initial_delay: Duration::from_millis(100),
			max_delay: Duration::from_secs(10),
			backoff_multiplier: 2.0,
		}),
		("Gentle", RetryPolicy {
			max_retries: 3,
			initial_delay: Duration::from_secs(1),
			max_delay: Duration::from_secs(30),
			backoff_multiplier: 3.0,
		}),
	];

	for (name, policy) in &policies {
		println!("   🎛️  {} Policy:", name);
		println!("     Max retries: {}", policy.max_retries);
		println!("     Initial delay: {:?}", policy.initial_delay);
		println!("     Max delay: {:?}", policy.max_delay);
		println!("     Backoff multiplier: {:.1}x", policy.backoff_multiplier);
	}

	// 2. Demonstrate retry scenarios
	println!("\n2. Retry Scenarios:");

	let scenarios = vec![
		("High success rate (80%)", 0.8),
		("Medium success rate (50%)", 0.5),
		("Low success rate (20%)", 0.2),
		("Always fails (0%)", 0.0),
	];

	for (description, success_rate) in scenarios {
		println!("\n   📊 Testing: {}", description);

		let mock_op = MockRpcOperation::new(success_rate);
		let retry_client = RetryClient::new(RetryPolicy::default());

		match retry_client
			.execute_with_retry(|| mock_op.call())
			.await
		{
			Ok(result) => println!("     ✅ {}", result),
			Err(error) => println!("     ❌ Final failure: {}", error),
		}
	}

	// 3. Real-world integration patterns
	println!("\n3. Integration Patterns:");

	println!("   🔗 HTTP Client Integration:");
	println!("   ```rust");
	println!("   let retry_client = RetryClient::new(RetryPolicy::default());");
	println!("   let result = retry_client.execute_with_retry(|| {{");
	println!("       http_client.post(rpc_url)");
	println!("           .json(&rpc_request)");
	println!("           .send()");
	println!("           .await");
	println!("   }}).await?;");
	println!("   ```");

	println!("\n   🌐 Neo N3 RPC Integration:");
	println!("   ```rust");
	println!("   let result = retry_client.execute_with_retry(|| {{");
	println!("       client.get_block_count().await");
	println!("   }}).await?;");
	println!("   ```");

	// 4. Advanced retry strategies
	println!("\n4. Advanced Retry Strategies:");

	let strategies = vec![
		("Circuit Breaker", "Stop retrying after consecutive failures"),
		("Jittered Backoff", "Add randomness to prevent thundering herd"),
		("Per-Endpoint Policies", "Different retry logic per RPC method"),
		("Adaptive Timeouts", "Adjust timeouts based on response times"),
		("Health Check Integration", "Skip retries when service is known to be down"),
	];

	for (strategy, description) in strategies {
		println!("   ⚡ {}: {}", strategy, description);
	}

	// 5. Error classification examples
	println!("\n5. Error Classification:");

	let error_examples = vec![
		("Network timeout", "Retryable", "Temporary network issue"),
		("Invalid JSON", "Non-retryable", "Request format error"),
		("Rate limit (429)", "Retryable", "Server temporarily overloaded"),
		("Authentication (401)", "Non-retryable", "Invalid credentials"),
		("Server error (500)", "Retryable", "Internal server error"),
		("Not found (404)", "Non-retryable", "Resource doesn't exist"),
	];

	for (error, classification, reason) in error_examples {
		let icon = if classification == "Retryable" { "🔄" } else { "🚫" };
		println!("   {} {}: {} ({})", icon, error, classification, reason);
	}

	// 6. Performance considerations
	println!("\n6. Performance Considerations:");

	let considerations = vec![
		("Retry Budget", "Limit total retry time across all requests"),
		("Concurrency Limits", "Control parallel requests during retries"),
		("Metrics Collection", "Track retry success rates and latencies"),
		("Failure Prediction", "Use ML to predict when retries will succeed"),
		("Resource Management", "Monitor memory and connection usage"),
	];

	for (topic, description) in considerations {
		println!("   📊 {}: {}", topic, description);
	}

	// 7. Configuration best practices
	println!("\n7. Configuration Best Practices:");

	println!("   ⚙️  Environment-specific settings:");
	println!("     • Development: Fast retries for quick feedback");
	println!("     • Production: Conservative retries to avoid amplification");
	println!("     • Testing: Predictable retries for consistent tests");
	println!("     • Monitoring: Detailed logging and metrics collection");

	println!("\n   🎯 Method-specific policies:");
	println!("     • Read operations: More aggressive retries");
	println!("     • Write operations: Conservative retries");
	println!("     • Query operations: Balanced approach");
	println!("     • Subscription operations: Immediate retry with backoff");

	// 8. Monitoring and observability
	println!("\n8. Monitoring Integration:");

	println!("   📈 Key Metrics:");
	println!("     • Retry rate by operation type");
	println!("     • Success rate after retries");
	println!("     • Average retry latency");
	println!("     • Circuit breaker state changes");
	println!("     • Error distribution patterns");

	println!("\n   🚨 Alerting Thresholds:");
	println!("     • Retry rate > 10% sustained");
	println!("     • Success rate < 95% after retries");
	println!("     • Circuit breaker open for > 1 minute");
	println!("     • Retry latency > 95th percentile");

	println!("\n🎉 Neo N3 retry strategy example completed!");
	println!("💡 Key takeaways for production systems:");
	println!("   • Design retry policies based on operation criticality");
	println!("   • Implement proper error classification");
	println!("   • Monitor retry patterns and adjust policies");
	println!("   • Use circuit breakers to prevent cascade failures");
	println!("   • Test retry behavior under various failure conditions");

	Ok(())
}