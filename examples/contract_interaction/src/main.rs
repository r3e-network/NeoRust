/// This example demonstrates real smart contract interaction on the Neo N3 blockchain.
/// It includes actual contract calls, parameter encoding, and response processing.
use neo3::prelude::*;
use neo3::neo_contract::{NeoToken, GasToken, FungibleTokenContract};
use std::str::FromStr;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔗 Neo N3 Smart Contract Interaction Example");
    println!("============================================");
    
    // 1. Setup RPC client
    println!("\n1. Setting up RPC client...");
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443/")?;
    let client = RpcClient::new(provider);
    let block_count = client.get_block_count().await?;
    println!("   ✅ Connected to Neo N3 TestNet at block height: {}", block_count);
    
    // 2. Define contract addresses and test account
    println!("\n2. Contract and account setup...");
    
    // NEO token contract hash
    let neo_hash = ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
    println!("   📝 NEO Token: 0x{}", neo_hash);
    
    // GAS token contract hash
    let gas_hash = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
    println!("   ⛽ GAS Token: 0x{}", gas_hash);
    
    // Test account for demonstrations
    let test_address = "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc";
    let test_script_hash = ScriptHash::from_address(test_address)?;
    println!("   👤 Test Account: {}", test_address);
    
    // 3. Basic contract method invocation - symbol
    println!("\n3. Invoking contract methods...");
    
    // Get NEO token symbol
    println!("   🔍 Getting NEO token symbol...");
    match client.invoke_function(&neo_hash, "symbol", vec![], vec![]).await {
        Ok(result) => {
            println!("     ✅ NEO symbol call result:");
            println!("       State: {}", result.state);
            println!("       Gas Consumed: {}", result.gas_consumed);
            
            if let Some(stack) = &result.stack {
                for (i, item) in stack.iter().enumerate() {
                    println!("       Stack[{}]: {:?}", i, item);
                    
                    // Try to decode as string if it's a byte string
                    if let StackItem::ByteString(bytes) = item {
                        if let Ok(symbol) = String::from_utf8(bytes.clone()) {
                            println!("       Decoded Symbol: {}", symbol);
                        }
                    }
                }
            }
        }
        Err(e) => println!("     ❌ Error: {}", e),
    }

    // Get GAS token decimals
    println!("\n   🔍 Getting GAS token decimals...");
    match client.invoke_function(&gas_hash, "decimals", vec![], vec![]).await {
        Ok(result) => {
            println!("     ✅ GAS decimals call result:");
            println!("       State: {}", result.state);
            
            if let Some(stack) = &result.stack {
                for item in stack {
                    if let StackItem::Integer(value) = item {
                        println!("       Decimals: {}", value);
                    }
                }
            }
        }
        Err(e) => println!("     ❌ Error: {}", e),
    }

    // 4. Contract calls with parameters
    println!("\n4. Contract calls with parameters...");
    
    // Get NEO balance for test account
    println!("   💰 Getting NEO balance for test account...");
    let balance_params = vec![
        ContractParameter::hash160(&test_script_hash)
    ];
    
    match client.invoke_function(&neo_hash, "balanceOf", balance_params, vec![]).await {
        Ok(result) => {
            println!("     ✅ Balance query result:");
            println!("       State: {}", result.state);
            
            if let Some(stack) = &result.stack {
                for item in stack {
                    if let StackItem::Integer(balance) = item {
                        println!("       NEO Balance: {} (raw)", balance);
                        println!("       NEO Balance: {} NEO", balance); // NEO is indivisible
                    }
                }
            }
        }
        Err(e) => println!("     ❌ Error: {}", e),
    }

    // Get GAS balance for test account
    println!("\n   ⛽ Getting GAS balance for test account...");
    let gas_balance_params = vec![
        ContractParameter::hash160(&test_script_hash)
    ];
    
    match client.invoke_function(&gas_hash, "balanceOf", gas_balance_params, vec![]).await {
        Ok(result) => {
            println!("     ✅ GAS balance query result:");
            
            if let Some(stack) = &result.stack {
                for item in stack {
                    if let StackItem::Integer(balance) = item {
                        let gas_amount = *balance as f64 / 100_000_000.0; // 8 decimals
                        println!("       GAS Balance: {} (raw)", balance);
                        println!("       GAS Balance: {:.8} GAS", gas_amount);
                    }
                }
            }
        }
        Err(e) => println!("     ❌ Error: {}", e),
    }

    // 5. Multi-parameter contract calls
    println!("\n5. Multi-parameter contract calls...");
    
    // Example: Check allowance (though this would return 0 for most cases)
    println!("   🔍 Checking token allowance...");
    let owner_hash = test_script_hash;
    let spender_hash = ScriptHash::from_str("0000000000000000000000000000000000000000")?;
    
    let allowance_params = vec![
        ContractParameter::hash160(&owner_hash),
        ContractParameter::hash160(&spender_hash),
    ];
    
    match client.invoke_function(&gas_hash, "allowance", allowance_params, vec![]).await {
        Ok(result) => {
            println!("     ✅ Allowance query result:");
            println!("       State: {}", result.state);
            
            if let Some(stack) = &result.stack {
                for item in stack {
                    if let StackItem::Integer(allowance) = item {
                        let gas_allowance = *allowance as f64 / 100_000_000.0;
                        println!("       Allowance: {:.8} GAS", gas_allowance);
                    }
                }
            }
        }
        Err(e) => println!("     ❌ Error: {}", e),
    }

    // 6. Contract information queries
    println!("\n6. Contract information queries...");
    
    // Get contract manifest for NEO token
    println!("   📋 Getting NEO contract manifest...");
    match client.get_contract_state(&neo_hash).await {
        Ok(contract_state) => {
            println!("     ✅ NEO Contract State:");
            println!("       ID: {}", contract_state.id);
            println!("       Hash: 0x{}", contract_state.hash);
            println!("       Update Counter: {}", contract_state.update_counter);
            
            // Show manifest methods
            if let Some(manifest) = &contract_state.manifest {
                println!("       Methods:");
                for method in &manifest.abi.methods {
                    println!("         • {} ({})", method.name, method.safe);
                    for param in &method.parameters {
                        println!("           - {}: {}", param.name, param.type_name);
                    }
                }
                
                println!("       Events:");
                for event in &manifest.abi.events {
                    println!("         • {}", event.name);
                    for param in &event.parameters {
                        println!("           - {}: {}", param.name, param.type_name);
                    }
                }
            }
        }
        Err(e) => println!("     ❌ Error getting contract state: {}", e),
    }

    // 7. Transaction construction (demonstration without sending)
    println!("\n7. Transaction construction demonstration...");
    
    // Create a mock transfer transaction structure
    println!("   🏗️  Constructing transfer transaction...");
    
    let from_account = test_script_hash;
    let to_account = ScriptHash::from_str("abcdef1234567890abcdef1234567890abcdef12")?;
    let transfer_amount = 100_000_000i64; // 1 GAS
    
    // Build transfer script
    let mut script_builder = ScriptBuilder::new();
    script_builder.contract_call(
        &gas_hash,
        "transfer",
        &[
            ContractParameter::hash160(&from_account),
            ContractParameter::hash160(&to_account),
            ContractParameter::integer(transfer_amount),
            ContractParameter::any(None),
        ],
        None,
    )?;
    
    let transfer_script = script_builder.to_bytes();
    println!("     ✅ Transfer script built ({} bytes)", transfer_script.len());
    println!("     Script: 0x{}", transfer_script.iter().map(|b| format!("{:02x}", b)).collect::<String>());

    // Test the transfer script (without actually sending)
    println!("\n   🧪 Testing transfer script...");
    match client.invoke_script(&transfer_script, vec![]).await {
        Ok(result) => {
            println!("     ✅ Script test result:");
            println!("       State: {}", result.state);
            println!("       Gas Consumed: {}", result.gas_consumed);
            
            if result.state == "HALT" {
                println!("       ✅ Transaction would succeed");
            } else {
                println!("       ❌ Transaction would fail");
                if let Some(exception) = &result.exception {
                    println!("       Exception: {}", exception);
                }
            }
        }
        Err(e) => println!("     ❌ Script test error: {}", e),
    }

    // 8. Advanced contract interactions
    println!("\n8. Advanced contract interaction patterns...");
    
    // Multi-call transaction example
    println!("   🔄 Multi-call transaction example...");
    let mut multi_script_builder = ScriptBuilder::new();
    
    // First call: Check balance
    multi_script_builder.contract_call(
        &gas_hash,
        "balanceOf",
        &[ContractParameter::hash160(&from_account)],
        None,
    )?;
    
    // Second call: Get symbol
    multi_script_builder.contract_call(
        &gas_hash,
        "symbol",
        &[],
        None,
    )?;
    
    let multi_script = multi_script_builder.to_bytes();
    println!("     ✅ Multi-call script built ({} bytes)", multi_script.len());
    
    match client.invoke_script(&multi_script, vec![]).await {
        Ok(result) => {
            println!("     ✅ Multi-call result:");
            println!("       State: {}", result.state);
            
            if let Some(stack) = &result.stack {
                println!("       Stack items: {}", stack.len());
                for (i, item) in stack.iter().enumerate() {
                    println!("       [{}]: {:?}", i, item);
                }
            }
        }
        Err(e) => println!("     ❌ Multi-call error: {}", e),
    }

    // 9. Error handling and validation
    println!("\n9. Error handling examples...");
    
    // Try to call a non-existent method
    println!("   ❌ Testing invalid method call...");
    match client.invoke_function(&neo_hash, "nonexistent_method", vec![], vec![]).await {
        Ok(result) => {
            println!("     ⚠️  Unexpected success: {:?}", result);
        }
        Err(e) => {
            println!("     ✅ Expected error: {}", e);
        }
    }

    // Try to call with wrong parameters
    println!("\n   ❌ Testing invalid parameters...");
    let wrong_params = vec![
        ContractParameter::string("invalid_address")
    ];
    
    match client.invoke_function(&neo_hash, "balanceOf", wrong_params, vec![]).await {
        Ok(result) => {
            println!("     Result with invalid params:");
            println!("       State: {}", result.state);
            if let Some(exception) = &result.exception {
                println!("       Exception: {}", exception);
            }
        }
        Err(e) => {
            println!("     Error with invalid params: {}", e);
        }
    }

    // 10. Best practices summary
    println!("\n10. 💡 Contract interaction best practices:");
    println!("     ✅ Always test contracts calls with invoke_script before sending");
    println!("     ✅ Validate all parameters before contract invocation");
    println!("     ✅ Handle both success and error cases appropriately");
    println!("     ✅ Parse return values according to contract specifications");
    println!("     ✅ Use proper script hash formats for addresses");
    println!("     ✅ Consider gas costs for complex operations");
    println!("     ✅ Implement retry logic for network issues");

    // 11. Integration patterns
    println!("\n11. Integration patterns demonstrated:");
    println!("     🔗 Basic method invocation (symbol, decimals)");
    println!("     💰 Balance queries with parameter encoding");
    println!("     📋 Contract state and manifest inspection");
    println!("     🏗️  Transaction script construction");
    println!("     🧪 Script testing before execution");
    println!("     🔄 Multi-call transaction patterns");
    println!("     ❌ Error handling and validation");

    println!("\n🎉 Smart contract interaction example completed!");
    println!("💡 This example demonstrates real contract interaction capabilities:");
    println!("   • Direct method invocation with parameters");
    println!("   • Balance and token information queries");
    println!("   • Transaction script building and testing");
    println!("   • Multi-call transaction construction");
    println!("   • Error handling and validation patterns");
    println!("   • Contract state inspection");

    Ok(())
}