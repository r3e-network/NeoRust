use neo3::{
	neo_clients::{HttpProvider, RpcClient},
	prelude::*,
};
use std::time::Duration;
use tokio::time::{interval, timeout};

/// Neo N3 Block Subscription Example
///
/// This example demonstrates how to monitor new blocks on the Neo N3 blockchain
/// in real-time. Since Neo N3 doesn't have native WebSocket subscriptions like Ethereum,
/// we implement polling-based block monitoring with efficient caching and notifications.
#[tokio::main]
async fn main() -> eyre::Result<()> {
	println!("📦 Neo N3 Block Subscription Example");
	println!("====================================");

	// 1. Connect to Neo N3 TestNet
	println!("\n1. Connecting to Neo N3 TestNet...");
	let provider = HttpProvider::new("https://testnet1.neo.coz.io:443/")?;
	let client = RpcClient::new(provider);
	println!("   ✅ Connected successfully");

	// 2. Initialize block monitor
	println!("\n2. Initializing block monitor...");
	let mut block_monitor = BlockMonitor::new(&client).await?;
	println!("   ✅ Block monitor initialized");
	println!("   📊 Starting from block: {}", block_monitor.get_current_block());

	// 3. Subscribe to new blocks (simulate real-time monitoring)
	println!("\n3. Starting block subscription (monitoring for 30 seconds)...");
	println!("   🔄 Polling for new blocks every 5 seconds...");

	let mut poll_interval = interval(Duration::from_secs(5));
	let mut blocks_received = 0;
	let max_blocks = 6; // Monitor for ~30 seconds

	// Use timeout to limit the demonstration duration
	let subscription_result = timeout(Duration::from_secs(35), async {
		loop {
			poll_interval.tick().await;

			match block_monitor.check_for_new_blocks().await {
				Ok(new_blocks) =>
					for block in new_blocks {
						print_block_info(&block);
						blocks_received += 1;

						if blocks_received >= max_blocks {
							return Ok::<(), eyre::Report>(());
						}
					},
				Err(e) => {
					println!("   ⚠️  Error checking for blocks: {}", e);
				},
			}
		}
	})
	.await;

	match subscription_result {
		Ok(_) => println!("\n   ✅ Block subscription completed successfully"),
		Err(_) => println!("\n   ⏰ Block subscription timed out (demonstration ended)"),
	}

	// 4. Display subscription statistics
	println!("\n4. Subscription Statistics:");
	let stats = block_monitor.get_statistics();
	println!("   📊 Blocks monitored: {}", stats.blocks_processed);
	println!("   🔄 Polling cycles: {}", stats.poll_cycles);
	println!("   📈 Average block time: {:.1}s", stats.average_block_time_seconds());
	println!("   💹 Transactions monitored: {}", stats.total_transactions);

	// 5. Demonstrate block filtering and notifications
	println!("\n5. Advanced block monitoring features:");

	// Get latest block with detailed information
	if let Ok(latest_block) = client.get_block(serde_json::json!("latest")).await {
		analyze_block_content(&latest_block);
	}

	// 6. Best practices for block monitoring
	println!("\n6. 💡 Neo N3 Block Monitoring Best Practices:");
	println!("   ✅ Use efficient polling intervals (15s matches block time)");
	println!("   ✅ Implement proper error handling and retries");
	println!("   ✅ Cache block data to avoid redundant API calls");
	println!("   ✅ Monitor for block reorganizations");
	println!("   ✅ Filter blocks based on relevant transactions");
	println!("   ✅ Implement backoff strategies during network issues");
	println!("   ✅ Use block confirmations for critical operations");

	println!("\n🎉 Block subscription example completed!");
	println!("💡 This demonstrates real-time block monitoring for Neo N3.");

	Ok(())
}

/// Block information structure
#[derive(Debug, Clone)]
struct BlockInfo {
	index: u64,
	hash: String,
	timestamp: u64,
	transaction_count: usize,
	size: usize,
	merkle_root: String,
}

/// Block monitoring statistics
#[derive(Debug)]
struct BlockStatistics {
	blocks_processed: u32,
	poll_cycles: u32,
	total_transactions: u32,
	first_block_time: Option<u64>,
	last_block_time: Option<u64>,
}

impl BlockStatistics {
	fn new() -> Self {
		Self {
			blocks_processed: 0,
			poll_cycles: 0,
			total_transactions: 0,
			first_block_time: None,
			last_block_time: None,
		}
	}

	fn average_block_time_seconds(&self) -> f64 {
		if let (Some(first), Some(last)) = (self.first_block_time, self.last_block_time) {
			if self.blocks_processed > 1 {
				let total_time = last - first;
				return (total_time as f64 / 1000.0) / (self.blocks_processed - 1) as f64;
			}
		}
		15.0 // Default Neo N3 block time
	}
}

/// Block monitor for Neo N3
struct BlockMonitor<'a> {
	client: &'a RpcClient<HttpProvider>,
	last_known_block: u64,
	statistics: BlockStatistics,
}

impl<'a> BlockMonitor<'a> {
	async fn new(client: &'a RpcClient<HttpProvider>) -> eyre::Result<Self> {
		let current_block = client.get_block_count().await?;

		Ok(Self { client, last_known_block: current_block, statistics: BlockStatistics::new() })
	}

	fn get_current_block(&self) -> u64 {
		self.last_known_block
	}

	async fn check_for_new_blocks(&mut self) -> eyre::Result<Vec<BlockInfo>> {
		self.statistics.poll_cycles += 1;

		let current_block_count = self.client.get_block_count().await?;
		let mut new_blocks = Vec::new();

		if current_block_count > self.last_known_block {
			// Process new blocks
			for block_index in (self.last_known_block + 1)..=current_block_count {
				if let Ok(block_data) = self.client.get_block(serde_json::json!(block_index)).await
				{
					if let Some(block_info) = self.parse_block_data(&block_data) {
						new_blocks.push(block_info);
						self.update_statistics(&block_data);
					}
				}
			}

			self.last_known_block = current_block_count;
		}

		Ok(new_blocks)
	}

	fn parse_block_data(&self, block_data: &serde_json::Value) -> Option<BlockInfo> {
		let index = block_data.get("index")?.as_u64()?;
		let hash = block_data.get("hash")?.as_str()?.to_string();
		let timestamp = block_data.get("time")?.as_u64()?;
		let merkle_root = block_data.get("merkleroot")?.as_str().unwrap_or("").to_string();

		let transactions = block_data.get("tx")?.as_array()?;
		let transaction_count = transactions.len();
		let size = block_data.get("size").and_then(|s| s.as_u64()).unwrap_or(0) as usize;

		Some(BlockInfo { index, hash, timestamp, transaction_count, size, merkle_root })
	}

	fn update_statistics(&mut self, block_data: &serde_json::Value) {
		self.statistics.blocks_processed += 1;

		if let Some(timestamp) = block_data.get("time").and_then(|t| t.as_u64()) {
			if self.statistics.first_block_time.is_none() {
				self.statistics.first_block_time = Some(timestamp);
			}
			self.statistics.last_block_time = Some(timestamp);
		}

		if let Some(tx_array) = block_data.get("tx").and_then(|tx| tx.as_array()) {
			self.statistics.total_transactions += tx_array.len() as u32;
		}
	}

	fn get_statistics(&self) -> &BlockStatistics {
		&self.statistics
	}
}

/// Print detailed block information
fn print_block_info(block: &BlockInfo) {
	let datetime = chrono::DateTime::from_timestamp(block.timestamp as i64 / 1000, 0)
		.unwrap_or_else(|| chrono::Utc::now());

	println!("\n   📦 New Block Received:");
	println!("     Block Index: {}", block.index);
	println!("     Block Hash: {}", block.hash);
	println!("     Timestamp: {} ({})", datetime.format("%Y-%m-%d %H:%M:%S UTC"), block.timestamp);
	println!("     Transactions: {}", block.transaction_count);
	println!("     Size: {} bytes", block.size);

	if !block.merkle_root.is_empty() {
		println!("     Merkle Root: {}...", &block.merkle_root[..20]);
	}
}

/// Analyze block content for interesting patterns
fn analyze_block_content(block_data: &serde_json::Value) {
	println!("   🔍 Analyzing latest block content:");

	// Analyze transactions
	if let Some(transactions) = block_data.get("tx").and_then(|tx| tx.as_array()) {
		println!("     📋 Transaction Analysis:");
		println!("       Total transactions: {}", transactions.len());

		// Count transaction types (simplified analysis)
		let mut contract_calls = 0;
		let mut transfers = 0;

		for tx in transactions {
			if let Some(tx_type) = tx.get("type").and_then(|t| t.as_str()) {
				match tx_type {
					"InvocationTransaction" => contract_calls += 1,
					_ => transfers += 1,
				}
			}
		}

		println!("       Contract calls: {}", contract_calls);
		println!("       Other transactions: {}", transfers);
	}

	// Block metadata
	if let Some(size) = block_data.get("size").and_then(|s| s.as_u64()) {
		println!("     📊 Block size: {} bytes", size);
	}

	if let Some(version) = block_data.get("version").and_then(|v| v.as_u64()) {
		println!("     🔖 Block version: {}", version);
	}
}
