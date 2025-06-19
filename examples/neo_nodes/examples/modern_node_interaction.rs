use neo3::prelude::*;

/// Modern Neo N3 Node Interaction Example
///
/// This example demonstrates how to interact with Neo N3 nodes using the production-ready SDK.
/// It shows proper error handling, connection management, and comprehensive blockchain queries.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🚀 Modern Neo N3 Node Interaction Example");
	println!("=========================================");

	// 1. Connect to TestNet with proper error handling
	println!("\n📡 1. Establishing TestNet connection...");

	let provider = providers::HttpProvider::new("https://testnet1.neo.org:443/")
		.expect("Failed to create HTTP provider");
	let client = providers::RpcClient::new(provider);

	println!("   ✅ Connected to Neo N3 TestNet");

	// 2. Basic blockchain information
	println!("\n📊 2. Retrieving blockchain information...");

	// Get current block count
	println!("   Getting current block height...");
	let block_count = client.get_block_count().await?;
	println!("   📈 Current block height: {}", block_count);

	// Get protocol version
	println!("   Getting protocol information...");
	let version = client.get_version().await?;
	println!("   🔧 Protocol version: {}", version.protocol.version);
	println!("   🌐 Network: {}", version.protocol.network);
	println!("   📝 User agent: {}", version.user_agent);

	// 3. Block exploration
	println!("\n🔍 3. Exploring recent blocks...");

	let latest_block_index = block_count - 1;
	let latest_block = client.get_block_by_index(latest_block_index, Some(true)).await?;

	println!("   📦 Latest block details:");
	println!("     Index: {}", latest_block.index);
	println!("     Hash: {}", latest_block.hash);
	println!("     Size: {} bytes", latest_block.size);
	println!("     Transactions: {}", latest_block.tx.len());
	println!("     Timestamp: {}", latest_block.time);

	// Show recent transaction activity
	if !latest_block.tx.is_empty() {
		println!("   📋 Recent transaction:");
		let tx = &latest_block.tx[0];
		println!("     Hash: {}", tx.hash);
		println!("     Sender: {}", tx.sender);
		println!("     System fee: {} GAS", tx.sys_fee as f64 / 100_000_000.0);
		println!("     Network fee: {} GAS", tx.net_fee as f64 / 100_000_000.0);
	}

	// 4. Network status
	println!("\n🌐 4. Network connectivity status...");

	let peers = client.get_peers().await?;
	println!("   👥 Connected peers: {}", peers.connected.len());
	println!("   ⏳ Unconnected peers: {}", peers.unconnected.len());
	println!("   ❌ Bad peers: {}", peers.bad.len());

	// Show some connected peer details
	if !peers.connected.is_empty() {
		println!("   🔗 Sample connected peer:");
		let peer = &peers.connected[0];
		println!("     Address: {}:{}", peer.address, peer.port);
	}

	// 5. Token information
	println!("\n💰 5. Native token information...");

	// NEO token details
	let neo_hash = neo_types::ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
	println!("   🪙 NEO Token:");
	println!("     Contract hash: 0x{}", hex::encode(neo_hash.0));

	// GAS token details
	let gas_hash = neo_types::ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
	println!("   ⛽ GAS Token:");
	println!("     Contract hash: 0x{}", hex::encode(gas_hash.0));

	// 6. Memory pool status
	println!("\n🔄 6. Memory pool status...");

	match client.get_raw_mempool(Some(false)).await {
		Ok(mempool) =>
			if let Some(tx_array) = mempool.as_array() {
				println!("   📨 Pending transactions: {}", tx_array.len());
			} else {
				println!("   📨 Pending transactions: 0");
			},
		Err(e) => println!("   ⚠️ Could not get mempool: {}", e),
	}

	// 7. Test multiple endpoints for reliability
	println!("\n🔧 7. Testing endpoint reliability...");

	let test_endpoints = vec![
		"https://testnet1.neo.org:443/",
		"https://testnet2.neo.org:443/",
		"https://testnet.rpc.n3.nspcc.ru:2331/",
	];

	for (i, endpoint) in test_endpoints.iter().enumerate() {
		print!("   Testing endpoint {}: {}... ", i + 1, endpoint);

		match test_endpoint_performance(endpoint).await {
			Ok(response_time) => println!("✅ {}ms", response_time),
			Err(_) => println!("❌ Failed"),
		}
	}

	// 8. Summary and best practices
	println!("\n📋 8. Summary and best practices:");
	println!("   ✅ Successfully connected to Neo N3 TestNet");
	println!("   ✅ Retrieved comprehensive blockchain information");
	println!("   ✅ Analyzed network connectivity");
	println!("   ✅ Tested multiple endpoints for reliability");

	println!("\n💡 Best practices for node interaction:");
	println!("   • Use multiple endpoints for redundancy");
	println!("   • Implement proper error handling and retries");
	println!("   • Monitor network health and peer connectivity");
	println!("   • Cache frequently accessed data when appropriate");
	println!("   • Use WebSocket connections for real-time updates");

	println!("\n🎉 Modern node interaction example completed successfully!");
	Ok(())
}

/// Test endpoint performance
async fn test_endpoint_performance(endpoint: &str) -> Result<u128, Box<dyn std::error::Error>> {
	let start = std::time::Instant::now();

	let provider = providers::HttpProvider::new(endpoint)?;
	let client = providers::RpcClient::new(provider);

	// Simple connectivity test
	let _block_count = client.get_block_count().await?;

	Ok(start.elapsed().as_millis())
}
