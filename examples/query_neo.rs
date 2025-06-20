use neo3::{neo_clients::APITrait, prelude::*};
use std::str::FromStr;

/// This example demonstrates how to query information about the NEO token on the Neo blockchain.
/// It shows comprehensive NEO token analysis including governance features and committee information.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🔷 Neo N3 NEO Token Comprehensive Query Example");
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

	// 3. Initialize NEO token contract
	println!("\n🔷 3. Initializing NEO token contract...");
	let neo_token_hash = ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
	println!("   📋 NEO token hash: 0x{}", hex::encode(neo_token_hash.0));

	// 4. Query basic NEO token information
	println!("\n📋 4. Retrieving NEO token properties...");

	match query_neo_token_info(&client, &neo_token_hash).await {
		Ok(info) => {
			println!("   🔷 Token Symbol: {}", info.symbol);
			println!("   🔢 Decimals: {}", info.decimals);
			println!("   📊 Total Supply: {} NEO", info.total_supply_formatted);
			println!("   💎 Max Supply: 100,000,000 NEO (fixed)");
		},
		Err(e) => println!("   ❌ Failed to get token info: {}", e),
	}

	// 5. Query balance for sample addresses
	println!("\n💰 5. Checking NEO balances for sample addresses...");

	let sample_addresses = vec![
		"NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc", // Genesis committee address
		"NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj", // Another known address
	];

	for address in &sample_addresses {
		if let Ok(script_hash) = ScriptHash::from_address(address) {
			match query_neo_balance(&client, &neo_token_hash, &script_hash).await {
				Ok(balance) => println!("   📍 {}: {} NEO", address, balance),
				Err(_) => println!("   📍 {}: Unable to query balance", address),
			}
		}
	}

	// 6. Query NEO governance information
	println!("\n🏛️ 6. Retrieving NEO governance information...");

	// Get current committee members
	match query_neo_committee(&client).await {
		Ok(committee) => {
			println!("   👥 Committee members: {}", committee.len());
			if !committee.is_empty() {
				println!("   📝 Sample committee member:");
				println!("     Public Key: {}", committee[0]);
			}
		},
		Err(e) => println!("   ❌ Failed to get committee: {}", e),
	}

	// Get candidates
	match query_neo_candidates(&client, &neo_token_hash).await {
		Ok(candidates_count) => {
			println!("   🗳️ Total candidates: {}", candidates_count);
		},
		Err(e) => println!("   ❌ Failed to get candidates: {}", e),
	}

	// 7. Query next block validators
	println!("\n⚡ 7. Retrieving next block validators...");

	match client.get_next_block_validators().await {
		Ok(validators) => {
			println!("   🎯 Next block validators: {}", validators.len());
			if !validators.is_empty() {
				println!("   📝 First validator:");
				println!("     Public Key: {}", validators[0].public_key);
				println!("     Votes: {}", validators[0].votes);
			}
		},
		Err(e) => println!("   ❌ Failed to get validators: {}", e),
	}

	// 8. NEO economics and tokenomics
	println!("\n💼 8. NEO Tokenomics Information:");
	println!("   📊 Total Supply: 100,000,000 NEO (fixed)");
	println!("   🔒 Indivisible: NEO cannot be divided (0 decimals)");
	println!("   🗳️ Governance: NEO holders vote for committee members");
	println!("   ⛽ Utility: NEO generates GAS automatically");
	println!("   🏛️ Committee: 21 members govern the network");
	println!("   🎯 Consensus: 7 validators per block (selected from committee)");

	// 9. Security and best practices
	println!("\n🔐 9. NEO Security Best Practices:");
	println!("   🛡️ Network Security:");
	println!("     • NEO uses dBFT 2.0 consensus mechanism");
	println!("     • Committee members are voted by NEO holders");
	println!("     • Network decisions require 2/3+ committee consensus");

	println!("   💼 Holder Security:");
	println!("     • Store NEO in secure wallets with backup");
	println!("     • Participate in governance by voting");
	println!("     • Monitor GAS generation from holdings");
	println!("     • Be aware of network governance proposals");

	println!("\n🎉 NEO token comprehensive query completed!");
	println!("💡 NEO is the governance token of the Neo blockchain ecosystem.");
	println!("💡 It enables voting, consensus participation, and network governance.");

	Ok(())
}

/// NEO token information structure
#[derive(Debug)]
struct NeoTokenInfo {
	symbol: String,
	decimals: u32,
	#[allow(dead_code)]
	total_supply: u64,
	total_supply_formatted: f64,
}

/// Query NEO token basic information
async fn query_neo_token_info(
	client: &providers::RpcClient<providers::HttpProvider>,
	token_hash: &ScriptHash,
) -> Result<NeoTokenInfo, Box<dyn std::error::Error>> {
	// Query symbol
	let symbol_result = client
		.invoke_function(
			&primitive_types::H160::from(token_hash.0),
			"symbol".to_string(),
			vec![],
			None,
		)
		.await?;

	// Query decimals
	let decimals_result = client
		.invoke_function(
			&primitive_types::H160::from(token_hash.0),
			"decimals".to_string(),
			vec![],
			None,
		)
		.await?;

	// Query total supply
	let supply_result = client
		.invoke_function(
			&primitive_types::H160::from(token_hash.0),
			"totalSupply".to_string(),
			vec![],
			None,
		)
		.await?;

	// Parse results
	let symbol = symbol_result
		.stack
		.first()
		.and_then(|item| item.as_string())
		.unwrap_or_else(|| "NEO".to_string());

	let decimals = decimals_result.stack.first().and_then(|item| item.as_int()).unwrap_or(0) as u32;

	let total_supply =
		supply_result.stack.first().and_then(|item| item.as_int()).unwrap_or(0) as u64;

	let total_supply_formatted = total_supply as f64 / 10f64.powi(decimals as i32);

	Ok(NeoTokenInfo { symbol, decimals, total_supply, total_supply_formatted })
}

/// Query NEO balance for an account
async fn query_neo_balance(
	client: &providers::RpcClient<providers::HttpProvider>,
	token_hash: &ScriptHash,
	account_hash: &ScriptHash,
) -> Result<u64, Box<dyn std::error::Error>> {
	let balance_result = client
		.invoke_function(
			&primitive_types::H160::from(token_hash.0),
			"balanceOf".to_string(),
			vec![ContractParameter::h160(account_hash)],
			None,
		)
		.await?;

	let balance = balance_result.stack.first().and_then(|item| item.as_int()).unwrap_or(0) as u64;

	Ok(balance)
}

/// Query NEO committee members
async fn query_neo_committee(
	client: &providers::RpcClient<providers::HttpProvider>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let committee = client.get_committee().await?;
	Ok(committee)
}

/// Query NEO candidates count
async fn query_neo_candidates(
	client: &providers::RpcClient<providers::HttpProvider>,
	token_hash: &ScriptHash,
) -> Result<usize, Box<dyn std::error::Error>> {
	let candidates_result = client
		.invoke_function(
			&primitive_types::H160::from(token_hash.0),
			"getCandidates".to_string(),
			vec![],
			None,
		)
		.await?;

	// Try to get candidates count from the result
	let count = candidates_result
		.stack
		.first()
		.and_then(|item| item.as_array())
		.map(|arr| arr.len())
		.unwrap_or(0);

	Ok(count)
}
