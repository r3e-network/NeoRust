use neo3::prelude::*;
use std::time::{Duration, Instant};

/// This example demonstrates comprehensive Neo N3 node connectivity and blockchain querying.
/// It includes connection handling, failover, performance monitoring, and various RPC operations.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🌐 Neo N3 Node Connection Example");
	println!("==================================\n");

	// 1. Multi-endpoint connection with failover
	println!("📡 1. Establishing connection with failover support...");
	let testnet_endpoints = vec![
		"https://testnet1.neo.org:443/",
		"https://testnet2.neo.org:443/",
		"http://seed1t5.neo.org:20332",
		"http://seed2t5.neo.org:20332",
		"http://seed3t5.neo.org:20332",
	];
	
	let client = connect_with_failover(testnet_endpoints).await?;
	println!("   ✅ Connected successfully!");

	// 2. Get comprehensive node information
	println!("\n📊 2. Retrieving node information...");
	let start = Instant::now();
	
	// Get version info
	let version = client.get_version().await?;
	println!("   🏷️  Node version: {}", version.user_agent);
	if let Some(protocol) = &version.protocol {
		println!("   🌐 Network: {}", protocol.network);
		println!("   ⏱️  Block time: {}ms", protocol.ms_per_block);
		println!("   📦 Max traceable blocks: {}", protocol.max_traceable_blocks);
		println!("   🔢 Address version: {}", protocol.address_version);
	}
	
	println!("   ⚡ Response time: {}ms", start.elapsed().as_millis());

	// 3. Query current blockchain state
	println!("\n🔗 3. Querying blockchain state...");
	
	// Get block count
	let block_count = client.get_block_count().await?;
	println!("   📦 Current block height: {}", block_count);
	
	// Get best block hash
	let best_hash = client.get_best_block_hash().await?;
	println!("   🔝 Best block hash: 0x{}", best_hash);
	
	// Get state height
	match client.get_state_height().await {
		Ok(state_height) => {
			println!("   📊 Local state height: {}", state_height.local_root_index);
			println!("   🌐 Validated state height: {}", state_height.validated_root_index);
		},
		Err(_) => println!("   ⚠️  State service not available"),
	}

	// 4. Examine latest block details
	println!("\n📦 4. Examining latest block...");
	let latest_block = client.get_block_by_index(block_count - 1, true).await?;
	
	println!("   🔢 Block #{}", latest_block.index);
	println!("   📅 Timestamp: {}", chrono::DateTime::from_timestamp(latest_block.time as i64, 0)
		.map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
		.unwrap_or_else(|| "Unknown".to_string()));
	println!("   📏 Size: {} bytes", latest_block.size);
	println!("   🔐 Hash: 0x{}", latest_block.hash);
	println!("   ⬅️  Previous: 0x{}", latest_block.prev_block_hash);
	println!("   🌳 Merkle root: 0x{}", latest_block.merkle_root_hash);
	
	if let Some(witness) = &latest_block.witness {
		println!("   ✍️  Witness: {} bytes", witness.invocation.len());
	}
	
	// Transaction details
	if let Some(transactions) = &latest_block.transactions {
		println!("   💸 Transactions: {}", transactions.len());
		if !transactions.is_empty() {
			println!("\n   📋 Sample transactions:");
			for (idx, tx) in transactions.iter().take(3).enumerate() {
				println!("      {}. Hash: 0x{}", idx + 1, tx.hash);
				println!("         Size: {} bytes", tx.size);
				println!("         Network fee: {} GAS", tx.net_fee as f64 / 100_000_000.0);
				println!("         System fee: {} GAS", tx.sys_fee as f64 / 100_000_000.0);
			}
		}
	}

	// 5. Network connectivity analysis
	println!("\n🌐 5. Analyzing network connectivity...");
	let peers = client.get_peers().await?;
	
	println!("   📊 Network statistics:");
	println!("      • Connected peers: {}", peers.connected.len());
	println!("      • Unconnected peers: {}", peers.unconnected.len());
	println!("      • Bad peers: {}", peers.bad.len());
	
	if !peers.connected.is_empty() {
		println!("\n   🔗 Sample connected peers:");
		for (idx, peer) in peers.connected.iter().take(5).enumerate() {
			println!("      {}. {}", idx + 1, peer.address);
			if let Some(last_seen) = peer.last_seen {
				println!("         Last seen: {} seconds ago", 
					(chrono::Utc::now().timestamp() - last_seen as i64).abs());
			}
		}
	}

	// 6. Memory pool status
	println!("\n🏊 6. Checking memory pool...");
	match client.get_raw_mempool().await {
		Ok(mempool) => {
			println!("   📊 Mempool size: {} transactions", mempool.len());
			if !mempool.is_empty() {
				println!("   📋 Sample pending transactions:");
				for (idx, tx_hash) in mempool.iter().take(3).enumerate() {
					println!("      {}. 0x{}", idx + 1, tx_hash);
				}
			}
		},
		Err(e) => println!("   ⚠️  Mempool unavailable: {}", e),
	}

	// 7. Native contract states
	println!("\n📜 7. Querying native contracts...");
	let native_contracts = vec![
		("ContractManagement", "fffdc93764dbaddd97c48f252a53ea4643faa3fd"),
		("NeoToken", "ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"),
		("GasToken", "d2a4cff31913016155e38e474a2c06d08be276cf"),
		("PolicyContract", "cc5e4edd9f5f8dba8bb65734541df7a1c081c67b"),
		("RoleManagement", "49cf4e5378ffcd4dec034fd98a174c5491e395e2"),
		("Oracle", "fe924b7cfe89ddd271abaf7210a80a7e11178758"),
		("NameService", "50ac1c37690cc2cfc594472833cf57505d5f46de"),
	];
	
	for (name, hash) in native_contracts.iter() {
		match client.get_contract_state(&neo3::neo_types::ScriptHash::from_str(hash)?).await {
			Ok(state) => {
				println!("   ✅ {}: v{}", name, state.manifest.as_ref()
					.and_then(|m| m.extra.as_ref())
					.and_then(|e| e.get("version"))
					.and_then(|v| v.as_str())
					.unwrap_or("unknown"));
			},
			Err(_) => println!("   ❌ {} not found", name),
		}
	}

	// 8. Performance benchmarking
	println!("\n⚡ 8. Performance benchmarking...");
	let iterations = 10;
	let mut total_time = Duration::ZERO;
	
	for i in 0..iterations {
		let start = Instant::now();
		let _ = client.get_block_count().await?;
		let elapsed = start.elapsed();
		total_time += elapsed;
		
		if i == 0 {
			println!("   🏃 Running {} iterations...", iterations);
		}
	}
	
	let avg_time = total_time / iterations as u32;
	println!("   ⏱️  Average response time: {}ms", avg_time.as_millis());
	println!("   📊 Requests per second: {:.1}", 1000.0 / avg_time.as_millis() as f64);

	// 9. Test multiple networks
	println!("\n🌍 9. Testing multiple networks...");
	
	// Test MainNet
	print!("   🔷 MainNet: ");
	match test_network("https://mainnet1.neo.org:443/").await {
		Ok((height, time)) => println!("✅ Height: {}, Response: {}ms", height, time),
		Err(e) => println!("❌ Failed: {}", e),
	}
	
	// Test TestNet (already connected)
	print!("   🔶 TestNet: ");
	println!("✅ Height: {}, Connected", block_count);
	
	// Test local node (if available)
	print!("   💻 Local node: ");
	match test_network("http://localhost:10332").await {
		Ok((height, time)) => println!("✅ Height: {}, Response: {}ms", height, time),
		Err(_) => println!("❌ Not available"),
	}

	// 10. Advanced RPC operations
	println!("\n🔧 10. Advanced RPC operations...");
	
	// Get validators
	match client.get_next_block_validators().await {
		Ok(validators) => {
			println!("   👥 Active validators: {}", validators.len());
			for (idx, validator) in validators.iter().take(3).enumerate() {
				println!("      {}. Public key: {}", idx + 1, validator.public_key);
				println!("         Votes: {}", validator.votes);
			}
		},
		Err(_) => println!("   ⚠️  Validator info unavailable"),
	}
	
	// Get committee
	match client.get_committee().await {
		Ok(committee) => {
			println!("   🏛️  Committee size: {} members", committee.len());
		},
		Err(_) => println!("   ⚠️  Committee info unavailable"),
	}

	println!("\n✅ Node connection example completed!");
	println!("💡 Successfully demonstrated comprehensive node connectivity and querying");

	Ok(())
}

/// Connect to nodes with automatic failover
async fn connect_with_failover(endpoints: Vec<&str>) -> Result<neo3::providers::RpcClient<neo3::providers::HttpProvider>, Box<dyn std::error::Error>> {
	for (idx, endpoint) in endpoints.iter().enumerate() {
		print!("   Trying {}: {} ... ", idx + 1, endpoint);
		
		match neo3::providers::HttpProvider::new(endpoint) {
			Ok(provider) => {
				let client = neo3::providers::RpcClient::new(provider);
				
				// Test the connection
				match client.get_block_count().await {
					Ok(_) => {
						println!("✅ Connected!");
						return Ok(client);
					},
					Err(_) => println!("❌ Failed"),
				}
			},
			Err(_) => println!("❌ Invalid endpoint"),
		}
	}
	
	Err("Failed to connect to any endpoint".into())
}

/// Test network connectivity and measure response time
async fn test_network(endpoint: &str) -> Result<(u32, u128), Box<dyn std::error::Error>> {
	let start = Instant::now();
	let provider = neo3::providers::HttpProvider::new(endpoint)?;
	let client = neo3::providers::RpcClient::new(provider);
	let height = client.get_block_count().await?;
	let elapsed = start.elapsed().as_millis();
	Ok((height, elapsed))
}
