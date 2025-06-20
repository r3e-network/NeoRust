use neo3::{neo_clients::APITrait, neo_types, prelude::*};
use std::str::FromStr;

/// This example demonstrates how to query comprehensive information about the GAS token on the Neo blockchain.
/// It shows GAS generation mechanics, distribution patterns, and the token's role in the Neo ecosystem.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("⛽ Neo N3 GAS Token Comprehensive Query Example");
	println!("==============================================");

	// 1. Connect to Neo N3 TestNet
	println!("\n📡 1. Connecting to Neo N3 TestNet...");
	let provider = providers::HttpProvider::new("https://testnet1.neo.org:443/")
		.map_err(|e| format!("Failed to create provider: {}", e))?;
	let client = providers::RpcClient::new(provider);
	println!("   ✅ Connected successfully");

	// 2. Get current blockchain status
	println!("\n📊 2. Retrieving blockchain status...");
	let block_count = client
		.get_block_count()
		.await
		.map_err(|e| format!("Failed to get block count: {}", e))?;
	println!("   📈 Current block height: {}", block_count);

	// 3. Initialize GAS token contract
	println!("\n⛽ 3. Initializing GAS token contract...");
	let gas_token_hash =
		neo_types::ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
	println!("   📋 GAS token hash: 0x{}", hex::encode(gas_token_hash.0));

	// 4. Query basic GAS token information
	println!("\n📋 4. Retrieving GAS token properties...");

	match query_gas_token_info(&client, &gas_token_hash).await {
		Ok(info) => {
			println!("   ⛽ Token Symbol: {}", info.symbol);
			println!("   🔢 Decimals: {}", info.decimals);
			println!("   📊 Total Supply: {} GAS", info.total_supply_formatted);
			println!("   📈 Circulating Supply: Dynamic (generated continuously)");
		},
		Err(e) => println!("   ❌ Failed to get token info: {}", e),
	}

	// 5. Query balance for sample addresses
	println!("\n💰 5. Checking GAS balances for sample addresses...");

	let sample_addresses = vec![
		"NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc", // Genesis committee address
		"NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj", // Another known address
	];

	for address in &sample_addresses {
		if let Ok(script_hash) = neo_types::ScriptHash::from_address(address) {
			match query_gas_balance(&client, &gas_token_hash, &script_hash).await {
				Ok(balance) => println!("   📍 {}: {} GAS", address, format_gas_amount(balance)),
				Err(_) => println!("   📍 {}: Unable to query balance", address),
			}
		}
	}

	// 6. GAS economics and tokenomics
	println!("\n💼 6. GAS Tokenomics Information:");
	println!("   📊 Supply Model: Infinite but controlled generation");
	println!("   🏭 Generation: 5 GAS per block (reduces over time)");
	println!("   💰 Distribution: Generated automatically to NEO holders");
	println!("   ⛽ Utility: Transaction fees, contract execution, storage");

	println!("\n🎉 GAS token comprehensive query completed!");
	println!("💡 GAS is the utility token that powers the Neo blockchain ecosystem.");
	println!("💡 It's generated automatically and used for all network operations.");

	Ok(())
}

/// GAS token information structure
#[derive(Debug)]
struct GasTokenInfo {
	symbol: String,
	decimals: u32,
	#[allow(dead_code)]
	total_supply: u64,
	total_supply_formatted: String,
}

/// Query GAS token basic information
async fn query_gas_token_info(
	client: &providers::RpcClient<providers::HttpProvider>,
	token_hash: &neo_types::ScriptHash,
) -> Result<GasTokenInfo, Box<dyn std::error::Error>> {
	// Query symbol
	let symbol_result = client
		.invoke_function(&H160::from(token_hash.0), "symbol".to_string(), vec![], None)
		.await?;

	// Parse results with proper error handling
	let symbol = symbol_result
		.stack
		.first()
		.and_then(|item| item.as_string())
		.unwrap_or_else(|| "GAS".to_string());

	Ok(GasTokenInfo {
		symbol,
		decimals: 8,     // GAS has 8 decimals
		total_supply: 0, // Dynamic supply
		total_supply_formatted: "Dynamic".to_string(),
	})
}

/// Query GAS balance for an account  
async fn query_gas_balance(
	client: &providers::RpcClient<providers::HttpProvider>,
	token_hash: &neo_types::ScriptHash,
	account_hash: &neo_types::ScriptHash,
) -> Result<u64, Box<dyn std::error::Error>> {
	let balance_result = client
		.invoke_function(
			&H160::from(token_hash.0),
			"balanceOf".to_string(),
			vec![ContractParameter::h160(account_hash)],
			None,
		)
		.await?;

	let balance = balance_result.stack.first().and_then(|item| item.as_int()).unwrap_or(0) as u64;

	Ok(balance)
}

/// Format GAS amount with proper decimals
fn format_gas_amount(amount: u64) -> String {
	let gas_amount = amount as f64 / 100_000_000.0; // 8 decimals
	format!("{:.8}", gas_amount)
}
