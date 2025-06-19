//! Neo N3 Smart Contract Interaction Example
//!
//! This example demonstrates how to interact with Neo N3 smart contracts
//! using the NeoRust SDK.

use neo3::{neo_clients::APITrait, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("📜 Neo N3 Smart Contract Interaction Example");
	println!("=============================================");

	// Connect to Neo N3 TestNet
	let provider = neo3::neo_clients::HttpProvider::new("https://testnet1.neo.org:443/")?;
	let client = neo3::neo_clients::RpcClient::new(provider);

	println!("✅ Connected to Neo N3 TestNet");

	// Example contract hash (NEO token contract)
	let neo_token_hash = "ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5";
	println!("📋 Interacting with NEO token contract: {}", neo_token_hash);

	// Get contract state
	match client.get_contract_state(neo_token_hash).await {
		Ok(contract_state) => {
			println!("✅ Contract found:");
			println!("   ID: {}", contract_state.id);
			println!("   Hash: {}", contract_state.hash);
			println!("   Manifest: {}", contract_state.manifest.name);
		},
		Err(e) => {
			println!("❌ Failed to get contract state: {}", e);
		},
	}

	println!("\n🔧 Contract interaction features:");
	println!("• Contract state queries");
	println!("• Method invocation");
	println!("• Event monitoring");
	println!("• Transaction building");

	Ok(())
}
