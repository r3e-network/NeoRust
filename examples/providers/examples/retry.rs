//! The RetryClient is a type that wraps around a JsonRpcClient and automatically retries failed
//! requests using an exponential backoff and filtering based on a RetryPolicy. It presents as a
//! JsonRpcClient, but with additional functionality for retrying requests.
//!
//! The RetryPolicy can be customized for specific applications and endpoints, mainly to handle
//! rate-limiting errors. In addition to the RetryPolicy, errors caused by connectivity issues such
//! as timed out connections or responses in the 5xx range can also be retried separately.

use neo3::prelude::*;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// A retry policy that determines whether an error should be retried
#[derive(Debug, Clone)]
pub struct RetryPolicy {
	pub max_retries: u32,
	pub initial_backoff: Duration,
	pub max_backoff: Duration,
	pub backoff_multiplier: f64,
	pub timeout_retries: bool,
	pub rate_limit_retries: bool,
}

impl Default for RetryPolicy {
	fn default() -> Self {
		Self {
			max_retries: 3,
			initial_backoff: Duration::from_millis(500),
			max_backoff: Duration::from_secs(30),
			backoff_multiplier: 2.0,
			timeout_retries: true,
			rate_limit_retries: true,
		}
	}
}

/// Retry client wrapper for Neo N3 RPC operations
pub struct RetryClient {
	client: RpcClient<HttpProvider>,
	policy: RetryPolicy,
}

impl RetryClient {
	pub fn new(client: RpcClient<HttpProvider>, policy: RetryPolicy) -> Self {
		Self { client, policy }
	}

	/// Classify error to determine if it should be retried
	fn should_retry(&self, error: &str, attempt: u32) -> bool {
		if attempt >= self.policy.max_retries {
			return false;
		}

		// Check for retryable errors
		let error_lower = error.to_lowercase();

		// Network/connection errors
		if error_lower.contains("timeout")
			|| error_lower.contains("connection")
			|| error_lower.contains("network")
			|| error_lower.contains("dns")
		{
			return self.policy.timeout_retries;
		}

		// Rate limiting errors
		if error_lower.contains("rate limit")
			|| error_lower.contains("too many requests")
			|| error_lower.contains("429")
		{
			return self.policy.rate_limit_retries;
		}

		// Server errors (5xx)
		if error_lower.contains("internal server error")
			|| error_lower.contains("service unavailable")
			|| error_lower.contains("502")
			|| error_lower.contains("503")
			|| error_lower.contains("504")
		{
			return true;
		}

		false
	}

	/// Calculate backoff duration with exponential growth
	fn calculate_backoff(&self, attempt: u32) -> Duration {
		let backoff_ms = self.policy.initial_backoff.as_millis() as f64
			* self.policy.backoff_multiplier.powi(attempt as i32);

		let backoff_duration = Duration::from_millis(backoff_ms as u64);

		// Cap at max backoff
		if backoff_duration > self.policy.max_backoff {
			self.policy.max_backoff
		} else {
			backoff_duration
		}
	}

	/// Retry wrapper for RPC operations
	pub async fn retry_operation<T, F, Fut>(
		&self,
		operation: F,
	) -> Result<T, Box<dyn std::error::Error>>
	where
		F: Fn() -> Fut,
		Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
	{
		let mut attempt = 0;
		let start_time = Instant::now();

		loop {
			match operation().await {
				Ok(result) => {
					if attempt > 0 {
						println!("   ✅ Operation succeeded after {} retries", attempt);
					}
					return Ok(result);
				},
				Err(error) => {
					attempt += 1;
					let error_str = error.to_string();

					if !self.should_retry(&error_str, attempt) {
						println!(
							"   ❌ Operation failed after {} attempts: {}",
							attempt, error_str
						);
						return Err(error);
					}

					let backoff = self.calculate_backoff(attempt - 1);
					println!(
						"   🔄 Attempt {} failed: {}. Retrying in {:?}...",
						attempt, error_str, backoff
					);

					sleep(backoff).await;
				},
			}
		}
	}

	/// Get block count with retry logic
	pub async fn get_block_count_with_retry(&self) -> Result<u64, Box<dyn std::error::Error>> {
		self.retry_operation(|| async {
			self.client
				.get_block_count()
				.await
				.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
		})
		.await
	}

	/// Get balance with retry logic
	pub async fn get_balance_with_retry(
		&self,
		address: &ScriptHash,
		token: &ScriptHash,
	) -> Result<u64, Box<dyn std::error::Error>> {
		self.retry_operation(|| async {
			self.client
				.get_nep17_balance(address, token)
				.await
				.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
		})
		.await
	}
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
	println!("🔄 Neo N3 Retry Client Example");
	println!("==============================");

	// 1. Create base RPC client
	println!("\n1. Setting up base RPC client...");

	let provider = match HttpProvider::new("https://testnet1.neo.coz.io:443/") {
		Ok(p) => p,
		Err(e) => {
			println!("   ❌ Failed to create provider: {}", e);
			// Continue with demonstration using mock errors
			return demonstrate_retry_logic().await;
		},
	};

	let base_client = RpcClient::new(provider);
	println!("   ✅ Base client created");

	// 2. Configure retry policy
	println!("\n2. Configuring retry policy...");

	let retry_policy = RetryPolicy {
		max_retries: 5,
		initial_backoff: Duration::from_millis(1000),
		max_backoff: Duration::from_secs(10),
		backoff_multiplier: 2.0,
		timeout_retries: true,
		rate_limit_retries: true,
	};

	println!("   📋 Retry Policy Configuration:");
	println!("     Max Retries: {}", retry_policy.max_retries);
	println!("     Initial Backoff: {:?}", retry_policy.initial_backoff);
	println!("     Max Backoff: {:?}", retry_policy.max_backoff);
	println!("     Backoff Multiplier: {}", retry_policy.backoff_multiplier);
	println!("     Timeout Retries: {}", retry_policy.timeout_retries);
	println!("     Rate Limit Retries: {}", retry_policy.rate_limit_retries);

	// 3. Create retry client
	println!("\n3. Creating retry client...");
	let retry_client = RetryClient::new(base_client, retry_policy);
	println!("   ✅ Retry client created with policy");

	// 4. Test successful operation
	println!("\n4. Testing successful operation...");
	match retry_client.get_block_count_with_retry().await {
		Ok(block_count) => {
			println!("   ✅ Block count: {}", block_count);
		},
		Err(e) => {
			println!("   ❌ Error getting block count: {}", e);
		},
	}

	// 5. Test balance query with retry
	println!("\n5. Testing balance query with retry...");
	let test_address = match ScriptHash::from_address("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc") {
		Ok(addr) => addr,
		Err(e) => {
			println!("   ❌ Invalid address: {}", e);
			return demonstrate_retry_logic().await;
		},
	};

	let gas_token = ScriptHash::gas();

	match retry_client.get_balance_with_retry(&test_address, &gas_token).await {
		Ok(balance) => {
			let gas_amount = balance as f64 / 100_000_000.0;
			println!("   ✅ GAS Balance: {:.8} GAS", gas_amount);
		},
		Err(e) => {
			println!("   ❌ Error getting balance: {}", e);
		},
	}

	// 6. Demonstrate error classification and retry logic
	demonstrate_retry_logic().await?;

	println!("\n🎉 Retry client example completed!");
	println!("💡 This example demonstrates real retry capabilities:");
	println!("   • Exponential backoff with configurable parameters");
	println!("   • Error classification for different retry strategies");
	println!("   • Network failure and timeout handling");
	println!("   • Rate limiting and server error recovery");
	println!("   • Production-ready retry patterns");

	Ok(())
}

async fn demonstrate_retry_logic() -> eyre::Result<()> {
	println!("\n6. Demonstrating retry error classification...");

	let retry_policy = RetryPolicy::default();
	let errors_and_expected = vec![
		("Connection timeout", true),
		("Network unreachable", true),
		("DNS resolution failed", true),
		("Rate limit exceeded", true),
		("Too many requests (429)", true),
		("Internal server error (500)", true),
		("Service unavailable (503)", true),
		("Bad gateway (502)", true),
		("Gateway timeout (504)", true),
		("Invalid parameters", false),
		("Unauthorized (401)", false),
		("Forbidden (403)", false),
		("Not found (404)", false),
		("Method not allowed (405)", false),
	];

	for (error_msg, should_retry) in errors_and_expected {
		let retry_client =
			RetryClient { client: create_mock_client(), policy: retry_policy.clone() };

		let will_retry = retry_client.should_retry(error_msg, 1);
		let status = if will_retry == should_retry { "✅" } else { "❌" };

		println!(
			"   {} '{}': {} (expected: {})",
			status,
			error_msg,
			if will_retry { "Will retry" } else { "Won't retry" },
			if should_retry { "retry" } else { "no retry" }
		);
	}

	// 7. Demonstrate backoff calculation
	println!("\n7. Demonstrating exponential backoff...");

	let retry_client = RetryClient {
		client: create_mock_client(),
		policy: RetryPolicy {
			initial_backoff: Duration::from_millis(500),
			backoff_multiplier: 2.0,
			max_backoff: Duration::from_secs(8),
			..Default::default()
		},
	};

	for attempt in 0..6 {
		let backoff = retry_client.calculate_backoff(attempt);
		println!("   Attempt {}: Backoff = {:?}", attempt + 1, backoff);
	}

	// 8. Simulate retry with mock failures
	println!("\n8. Simulating retry scenario...");

	let mut failure_count = 0;
	let max_failures = 2;

	let result = retry_client
		.retry_operation(|| async {
			failure_count += 1;
			if failure_count <= max_failures {
				Err(Box::new(std::io::Error::new(
					std::io::ErrorKind::TimedOut,
					"Connection timeout",
				)) as Box<dyn std::error::Error>)
			} else {
				Ok("Operation succeeded!")
			}
		})
		.await;

	match result {
		Ok(success_msg) => {
			println!("   ✅ {}", success_msg);
		},
		Err(e) => {
			println!("   ❌ Final failure: {}", e);
		},
	}

	// 9. Performance metrics simulation
	println!("\n9. Performance metrics demonstration...");

	let operations = vec![
		("Fast operation", Duration::from_millis(100), true),
		("Slow operation", Duration::from_millis(500), true),
		("Timeout operation", Duration::from_millis(5000), false),
		("Rate limited operation", Duration::from_millis(200), false),
	];

	for (name, duration, success) in operations {
		let start = std::time::Instant::now();

		if success {
			tokio::time::sleep(duration).await;
			let elapsed = start.elapsed();
			println!("   ✅ {}: completed in {:?}", name, elapsed);
		} else {
			println!("   ⚠️  {}: would require retry logic", name);
		}
	}

	Ok(())
}

fn create_mock_client() -> RpcClient<HttpProvider> {
	// Create a mock client for demonstration purposes
	let provider = HttpProvider::new("http://localhost:1234").unwrap();
	RpcClient::new(provider)
}
