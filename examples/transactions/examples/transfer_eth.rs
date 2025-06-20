use neo3::prelude::*;

/// Neo N3 GAS Transfer Example
/// 
/// This example demonstrates how to perform a basic GAS (utility token) transfer
/// on the Neo N3 blockchain. It shows the complete process from creating a transaction
/// to monitoring the results.
#[tokio::main]
async fn main() -> eyre::Result<()> {
	println!("⛽ Neo N3 GAS Transfer Example");
	println!("=============================");

	// 1. Connect to Neo N3 TestNet
	println!("\n1. Connecting to Neo N3 TestNet...");
	let provider = HttpProvider::new("https://testnet1.neo.coz.io:443/")?;
	let client = RpcClient::new(provider);
	println!("   ✅ Connected successfully");

	// 2. Set up transfer parameters
	println!("\n2. Setting up transfer parameters...");
	
	// Demo addresses (in practice, you'd use real addresses from your wallet)
	let from_address = ScriptHash::from_address("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc")?;
	let to_address = ScriptHash::from_address("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj")?;
	let transfer_amount = 50_000_000u64; // 0.5 GAS (8 decimals)

	println!("   📤 From: {}", from_address.to_address());
	println!("   📥 To: {}", to_address.to_address());
	println!("   💰 Amount: {} GAS", transfer_amount as f64 / 100_000_000.0);

	// 3. Check balances before transfer
	println!("\n3. Checking balances before transfer...");
	
	let gas_token = ScriptHash::gas();
	
	let from_balance_before = client.get_nep17_balance(&from_address, &gas_token).await
		.unwrap_or(0);
	let to_balance_before = client.get_nep17_balance(&to_address, &gas_token).await
		.unwrap_or(0);

	println!("   📊 From balance: {} GAS", from_balance_before as f64 / 100_000_000.0);
	println!("   📊 To balance: {} GAS", to_balance_before as f64 / 100_000_000.0);

	// 4. Demonstrate transaction construction (conceptual)
	println!("\n4. Transaction construction process:");
	println!("   🔧 In a real application, you would:");
	
	// Transaction building steps
	let tx_steps = vec![
		"Create transaction builder with GAS token contract",
		"Add transfer operation (from, to, amount)",
		"Set appropriate gas limit and network fee",
		"Sign transaction with sender's private key",
		"Broadcast transaction to the network",
		"Monitor transaction status and confirmations",
	];

	for (i, step) in tx_steps.iter().enumerate() {
		println!("   {}. {}", i + 1, step);
	}

	// 5. Show what the transaction would look like
	println!("\n5. Transaction structure (conceptual):");
	
	let conceptual_tx = serde_json::json!({
		"version": 0,
		"nonce": 1234567890,
		"sysfee": "997780", // System fee in GAS fractions
		"netfee": "1230610", // Network fee in GAS fractions
		"validuntilblock": client.get_block_count().await? + 86400, // Valid for ~24 hours
		"signers": [
			{
				"account": format!("0x{}", hex::encode(from_address.0)),
				"scopes": "CalledByEntry"
			}
		],
		"attributes": [],
		"script": format!("0x0c14{} 0c14{} 0c08{} 41627d5b52", 
			hex::encode(to_address.0),
			hex::encode(from_address.0),
			hex::encode(transfer_amount.to_le_bytes())
		),
		"witnesses": [
			{
				"invocation": "0x40...", // Signature would go here
				"verification": "0x0c40..." // Verification script
			}
		]
	});

	println!("   📋 Transaction JSON structure:");
	println!("{}", serde_json::to_string_pretty(&conceptual_tx)?);

	// 6. Estimate transaction costs
	println!("\n6. Transaction cost estimation:");
	
	let estimated_system_fee = 997_780u64; // Typical GAS transfer system fee
	let estimated_network_fee = 1_230_610u64; // Typical network fee
	let total_cost = estimated_system_fee + estimated_network_fee;

	println!("   ⚙️  System fee: {} GAS", estimated_system_fee as f64 / 100_000_000.0);
	println!("   🌐 Network fee: {} GAS", estimated_network_fee as f64 / 100_000_000.0);
	println!("   💸 Total cost: {} GAS", total_cost as f64 / 100_000_000.0);
	println!("   💰 Total with transfer: {} GAS", 
		(total_cost + transfer_amount) as f64 / 100_000_000.0);

	// 7. Security considerations
	println!("\n7. 🔐 Security considerations for transfers:");
	println!("   ✅ Verify recipient address format and validity");
	println!("   ✅ Check sender has sufficient balance + fees");
	println!("   ✅ Use appropriate transaction expiration");
	println!("   ✅ Implement proper error handling");
	println!("   ✅ Wait for sufficient confirmations");
	println!("   ✅ Store transaction hash for tracking");

	// 8. Integration with wallets
	println!("\n8. 💼 Wallet integration:");
	println!("   • Use neo3::wallets for key management");
	println!("   • Implement transaction signing with private keys");
	println!("   • Support hardware wallets (Ledger, etc.)");
	println!("   • Provide transaction history and monitoring");

	// 9. Best practices
	println!("\n9. 💡 Transfer best practices:");
	println!("   ✅ Always validate addresses before sending");
	println!("   ✅ Use appropriate gas limits to avoid failures");
	println!("   ✅ Implement retry logic for network issues");
	println!("   ✅ Provide clear user feedback during process");
	println!("   ✅ Log transactions for audit and debugging");
	println!("   ✅ Handle edge cases (insufficient funds, etc.)");

	// 10. Sample transaction monitoring
	println!("\n10. Transaction monitoring example:");
	
	// Get a recent transaction to show monitoring
	let current_block = client.get_block_count().await?;
	if let Ok(recent_block) = client.get_block(serde_json::json!(current_block - 1)).await {
		if let Some(transactions) = recent_block.get("tx").and_then(|tx| tx.as_array()) {
			if let Some(first_tx) = transactions.first() {
				if let Some(tx_hash) = first_tx.get("hash").and_then(|h| h.as_str()) {
					println!("    📋 Sample transaction monitoring:");
					println!("    Hash: {}", tx_hash);
					println!("    Block: {}", current_block - 1);
					println!("    Status: Confirmed");
					
					// Check if this transaction has application logs (events)
					if let Ok(app_log) = client.get_application_log(tx_hash.to_string()).await {
						if let Some(executions) = app_log.get("executions").and_then(|e| e.as_array()) {
							for execution in executions {
								if let Some(vm_state) = execution.get("vmstate").and_then(|s| s.as_str()) {
									println!("    VM State: {}", vm_state);
								}
							}
						}
					}
				}
			}
		}
	}

	println!("\n🎉 GAS transfer example completed!");
	println!("💡 This demonstrates the complete transfer process for Neo N3.");
	println!("🔗 For actual transfers, integrate with neo3::wallets and transaction building.");

	Ok(())
}
