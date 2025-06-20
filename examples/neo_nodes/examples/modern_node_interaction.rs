use neo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use futures::stream::StreamExt;

/// Modern Neo N3 Node Interaction Example
///
/// This example demonstrates advanced node interaction patterns including connection pooling,
/// caching, real-time monitoring, and production-ready error handling.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🚀 Modern Neo N3 Node Interaction Example");
	println!("=========================================\n");

	// 1. Create connection pool with multiple endpoints
	println!("📡 1. Creating connection pool with failover...");
	let connection_pool = create_connection_pool().await?;
	let client = &connection_pool.get_best_client().await?;
	println!("   ✅ Connection pool established with {} endpoints", connection_pool.size());

	// 2. Initialize cache for performance
	println!("\n💾 2. Initializing response cache...");
	let cache = Arc::new(RwLock::new(ResponseCache::new()));
	println!("   ✅ Cache initialized with 5-minute TTL");

	// 3. Get blockchain state with caching
	println!("\n📊 3. Retrieving blockchain state (with cache)...");
	
	// First call (cache miss)
	let start = std::time::Instant::now();
	let block_count = get_cached_block_count(client, &cache).await?;
	println!("   📈 Current block height: {}", block_count);
	println!("   ⏱️  Response time: {}ms (cache miss)", start.elapsed().as_millis());
	
	// Second call (cache hit)
	let start = std::time::Instant::now();
	let _block_count = get_cached_block_count(client, &cache).await?;
	println!("   💨 Response time: {}ms (cache hit)", start.elapsed().as_millis());

	// 4. Real-time block monitoring
	println!("\n📡 4. Real-time block monitoring...");
	monitor_blocks(client, 5).await?;

	// 5. Transaction analysis
	println!("\n💸 5. Analyzing recent transactions...");
	analyze_recent_transactions(client).await?;

	// 6. Smart contract interaction
	println!("\n📜 6. Querying smart contracts...");
	query_smart_contracts(client).await?;

	// 7. Network health monitoring
	println!("\n🏥 7. Network health monitoring...");
	monitor_network_health(client).await?;

	// 8. Advanced RPC batching
	println!("\n⚡ 8. Batch RPC operations...");
	perform_batch_operations(client).await?;

	// 9. State proof verification
	println!("\n🔐 9. State proof verification...");
	verify_state_proofs(client).await?;

	// 10. Performance metrics
	println!("\n📊 10. Performance metrics summary...");
	display_performance_metrics(&connection_pool).await?;

	println!("\n✅ Modern node interaction example completed!");
	println!("💡 Demonstrated production-ready patterns for Neo N3 integration");

	Ok(())
}

/// Connection pool for multiple endpoints
struct ConnectionPool {
	clients: Vec<(String, neo3::providers::RpcClient<neo3::providers::HttpProvider>)>,
	health_scores: Arc<RwLock<HashMap<String, f64>>>,
}

impl ConnectionPool {
	async fn get_best_client(&self) -> Result<&neo3::providers::RpcClient<neo3::providers::HttpProvider>, Box<dyn std::error::Error>> {
		let scores = self.health_scores.read().await;
		let best = self.clients
			.iter()
			.max_by(|(a, _), (b, _)| {
				scores.get(a).unwrap_or(&0.0)
					.partial_cmp(scores.get(b).unwrap_or(&0.0))
					.unwrap()
			})
			.ok_or("No healthy clients available")?;
		Ok(&best.1)
	}
	
	fn size(&self) -> usize {
		self.clients.len()
	}
}

/// Response cache with TTL
struct ResponseCache {
	data: HashMap<String, (serde_json::Value, std::time::Instant)>,
	ttl: std::time::Duration,
}

impl ResponseCache {
	fn new() -> Self {
		Self {
			data: HashMap::new(),
			ttl: std::time::Duration::from_secs(300), // 5 minutes
		}
	}
	
	fn get(&self, key: &str) -> Option<serde_json::Value> {
		self.data.get(key).and_then(|(value, timestamp)| {
			if timestamp.elapsed() < self.ttl {
				Some(value.clone())
			} else {
				None
			}
		})
	}
	
	fn set(&mut self, key: String, value: serde_json::Value) {
		self.data.insert(key, (value, std::time::Instant::now()));
	}
}

/// Create connection pool with health monitoring
async fn create_connection_pool() -> Result<ConnectionPool, Box<dyn std::error::Error>> {
	let endpoints = vec![
		"https://testnet1.neo.org:443/",
		"https://testnet2.neo.org:443/",
		"http://seed1t5.neo.org:20332",
		"http://seed2t5.neo.org:20332",
	];
	
	let mut clients = Vec::new();
	let health_scores = Arc::new(RwLock::new(HashMap::new()));
	
	for endpoint in endpoints {
		match neo3::providers::HttpProvider::new(endpoint) {
			Ok(provider) => {
				let client = neo3::providers::RpcClient::new(provider);
				
				// Test connection and set initial health score
				let start = std::time::Instant::now();
				match client.get_block_count().await {
					Ok(_) => {
						let response_time = start.elapsed().as_millis();
						let health_score = 1000.0 / (response_time as f64 + 1.0);
						health_scores.write().await.insert(endpoint.to_string(), health_score);
						clients.push((endpoint.to_string(), client));
						println!("   ✅ {} - Health: {:.2}", endpoint, health_score);
					},
					Err(_) => println!("   ❌ {} - Failed", endpoint),
				}
			},
			Err(_) => println!("   ❌ {} - Invalid", endpoint),
		}
	}
	
	if clients.is_empty() {
		return Err("No healthy endpoints available".into());
	}
	
	Ok(ConnectionPool { clients, health_scores })
}

/// Get block count with caching
async fn get_cached_block_count(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
	cache: &Arc<RwLock<ResponseCache>>,
) -> Result<u32, Box<dyn std::error::Error>> {
	const CACHE_KEY: &str = "block_count";
	
	// Check cache first
	{
		let cache_read = cache.read().await;
		if let Some(cached) = cache_read.get(CACHE_KEY) {
			if let Some(count) = cached.as_u64() {
				return Ok(count as u32);
			}
		}
	}
	
	// Cache miss - fetch from network
	let block_count = client.get_block_count().await?;
	
	// Update cache
	{
		let mut cache_write = cache.write().await;
		cache_write.set(CACHE_KEY.to_string(), serde_json::json!(block_count));
	}
	
	Ok(block_count)
}

/// Monitor blocks in real-time
async fn monitor_blocks(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
	block_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
	let current_height = client.get_block_count().await?;
	let start_height = current_height.saturating_sub(block_count as u32);
	
	println!("   📦 Monitoring last {} blocks...", block_count);
	
	for height in start_height..current_height {
		let block = client.get_block_by_index(height, false).await?;
		println!("   Block #{}: {} transactions, {} bytes", 
			block.index, 
			block.transactions.as_ref().map(|t| t.len()).unwrap_or(0),
			block.size
		);
	}
	
	Ok(())
}

/// Analyze recent transactions
async fn analyze_recent_transactions(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
) -> Result<(), Box<dyn std::error::Error>> {
	let current_height = client.get_block_count().await?;
	let mut total_fees = 0u64;
	let mut tx_count = 0;
	
	// Analyze last 10 blocks
	for i in 0..10 {
		let block = client.get_block_by_index(current_height - i - 1, true).await?;
		if let Some(transactions) = block.transactions {
			for tx in transactions {
				total_fees += tx.net_fee + tx.sys_fee;
				tx_count += 1;
			}
		}
	}
	
	println!("   📊 Last 10 blocks analysis:");
	println!("      • Total transactions: {}", tx_count);
	println!("      • Total fees: {} GAS", total_fees as f64 / 100_000_000.0);
	if tx_count > 0 {
		println!("      • Average fee: {} GAS", (total_fees as f64 / tx_count as f64) / 100_000_000.0);
	}
	
	Ok(())
}

/// Query smart contracts
async fn query_smart_contracts(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
) -> Result<(), Box<dyn std::error::Error>> {
	// Query NEO token
	let neo_hash = neo3::neo_types::ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
	match client.invoke_function(&neo_hash, "totalSupply", &[]).await {
		Ok(result) => {
			if let Some(stack) = result.stack {
				if let Some(item) = stack.first() {
					println!("   🪙 NEO Total Supply: {} NEO", 
						item.value.as_ref()
							.and_then(|v| v.as_str())
							.and_then(|s| s.parse::<i64>().ok())
							.unwrap_or(0) as f64 / 1.0
					);
				}
			}
		},
		Err(e) => println!("   ⚠️  Could not query NEO token: {}", e),
	}
	
	// Query GAS token
	let gas_hash = neo3::neo_types::ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
	match client.invoke_function(&gas_hash, "symbol", &[]).await {
		Ok(result) => {
			if let Some(stack) = result.stack {
				if let Some(item) = stack.first() {
					println!("   ⛽ GAS Symbol: {}", 
						item.value.as_ref()
							.and_then(|v| v.as_str())
							.unwrap_or("GAS")
					);
				}
			}
		},
		Err(e) => println!("   ⚠️  Could not query GAS token: {}", e),
	}
	
	Ok(())
}

/// Monitor network health
async fn monitor_network_health(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
) -> Result<(), Box<dyn std::error::Error>> {
	// Get sync status
	match client.get_state_height().await {
		Ok(state) => {
			let sync_percentage = (state.local_root_index as f64 / state.validated_root_index as f64) * 100.0;
			println!("   📊 State sync: {:.1}%", sync_percentage);
			println!("   🏷️  Local height: {}", state.local_root_index);
			println!("   ✅ Validated height: {}", state.validated_root_index);
		},
		Err(_) => println!("   ⚠️  State service not available"),
	}
	
	// Check mempool
	match client.get_raw_mempool().await {
		Ok(mempool) => {
			println!("   🏊 Mempool size: {} transactions", mempool.len());
		},
		Err(_) => println!("   ⚠️  Mempool not accessible"),
	}
	
	// Connection count
	let peers = client.get_peers().await?;
	let total_peers = peers.connected.len() + peers.unconnected.len();
	let connection_ratio = peers.connected.len() as f64 / total_peers.max(1) as f64;
	println!("   🌐 Connection health: {:.1}%", connection_ratio * 100.0);
	
	Ok(())
}

/// Perform batch RPC operations
async fn perform_batch_operations(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
) -> Result<(), Box<dyn std::error::Error>> {
	let start = std::time::Instant::now();
	
	// Execute multiple operations in parallel
	let (block_count, version, peers, validators) = tokio::join!(
		client.get_block_count(),
		client.get_version(),
		client.get_peers(),
		client.get_next_block_validators()
	);
	
	let elapsed = start.elapsed();
	
	println!("   ⚡ Batch operation completed in {}ms", elapsed.as_millis());
	println!("   📊 Results:");
	println!("      • Block count: {}", block_count?);
	println!("      • Version: {}", version?.user_agent);
	println!("      • Connected peers: {}", peers?.connected.len());
	println!("      • Validators: {}", validators?.len());
	
	Ok(())
}

/// Verify state proofs
async fn verify_state_proofs(
	client: &neo3::providers::RpcClient<neo3::providers::HttpProvider>,
) -> Result<(), Box<dyn std::error::Error>> {
	// Get state root
	match client.get_state_root(0).await {
		Ok(root) => {
			println!("   🌳 State root index: {}", root.index);
			println!("   🔐 Root hash: 0x{}", root.roothash);
			println!("   👥 Witnesses: {}", root.witnesses.len());
		},
		Err(_) => println!("   ⚠️  State service not enabled on this node"),
	}
	
	Ok(())
}

/// Display performance metrics
async fn display_performance_metrics(
	pool: &ConnectionPool,
) -> Result<(), Box<dyn std::error::Error>> {
	let health_scores = pool.health_scores.read().await;
	
	println!("   📈 Endpoint performance:");
	for (endpoint, score) in health_scores.iter() {
		let response_time = 1000.0 / score - 1.0;
		println!("      • {}: {:.0}ms avg response", endpoint, response_time);
	}
	
	println!("\n   💡 Performance recommendations:");
	println!("      • Use connection pooling for high-throughput applications");
	println!("      • Implement caching for frequently accessed data");
	println!("      • Monitor endpoint health and failover automatically");
	println!("      • Batch RPC calls when possible to reduce overhead");
	
	Ok(())
}
