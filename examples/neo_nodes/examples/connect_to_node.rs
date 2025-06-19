use neo3::{
	neo_clients::{APITrait, HttpProvider, JsonRpcProvider, RpcClient},
	prelude::*,
};

/// This example demonstrates how to connect to Neo N3 nodes and retrieve basic blockchain information.
/// It shows connection to both MainNet and TestNet and how to query node status.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Neo N3 Node Connection Example");
	println!("==============================");

	// Connect to Neo N3 TestNet
	println!("\nConnecting to Neo N3 TestNet...");
	let provider = providers::HttpProvider::new("https://testnet1.neo.org:443/")
		.map_err(|e| format!("Failed to create provider: {}", e))?;
	let client = providers::RpcClient::new(provider);

	// Get basic blockchain information
	println!("\nRetrieving blockchain information...");

	// Get current block count
	let block_count = client
		.get_block_count()
		.await
		.map_err(|e| format!("Failed to get block count: {}", e))?;
	println!("Current block count: {}", block_count);

	// Get node version information
	let version = client
		.get_version()
		.await
		.map_err(|e| format!("Failed to get version: {}", e))?;
	println!("Node version: {}", version.user_agent);
	if let Some(protocol) = &version.protocol {
		println!("Network: {}", protocol.network);
		println!("Max traceable blocks: {}", protocol.max_traceable_blocks);
		println!("MS per block: {}", protocol.ms_per_block);
	}

	// Get latest block hash
	let latest_hash = client
		.get_best_block_hash()
		.await
		.map_err(|e| format!("Failed to get latest block hash: {}", e))?;
	println!("Latest block hash: {}", latest_hash);

	// Get latest block details
	let latest_block = client
		.get_block_by_index(block_count - 1, true)
		.await
		.map_err(|e| format!("Failed to get latest block: {}", e))?;
	println!("\nLatest block details:");
	println!("  Block index: {}", latest_block.index);
	println!("  Block size: {} bytes", latest_block.size);
	if let Some(transactions) = &latest_block.transactions {
		println!("  Transaction count: {}", transactions.len());
	} else {
		println!("  Transaction count: 0");
	}
	println!("  Merkle root: {}", latest_block.merkle_root_hash);
	println!("  Previous block: {}", latest_block.prev_block_hash);

	// Get network peer information
	let peers = client.get_peers().await.map_err(|e| format!("Failed to get peers: {}", e))?;
	println!("\nNetwork connectivity:");
	println!("  Connected peers: {}", peers.connected.len());
	println!("  Unconnected peers: {}", peers.unconnected.len());
	println!("  Bad peers: {}", peers.bad.len());

	// Test connectivity to MainNet as well
	println!("\nTesting MainNet connectivity...");
	match test_mainnet_connection().await {
		Ok(block_count) => println!("✅ MainNet responsive - Block count: {}", block_count),
		Err(e) => println!("❌ MainNet connection failed: {}", e),
	}

	println!("\nNode connection example completed successfully!");
	Ok(())
}

/// Test connection to MainNet
async fn test_mainnet_connection() -> Result<u32, Box<dyn std::error::Error>> {
	let provider = providers::HttpProvider::new("https://mainnet1.neo.org:443/")?;
	let client = providers::RpcClient::new(provider);
	let block_count = client.get_block_count().await?;
	Ok(block_count)
}
