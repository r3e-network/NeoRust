use neo3::{
	neo_clients::{APITrait, HttpProvider, RpcClient},
	neo_crypto::KeyPair,
	neo_protocol::{Account, AccountTrait},
	neo_types::{ContractParameter, ScriptHash},
	prelude::*,
};
use std::{collections::HashMap, str::FromStr};

/// This example demonstrates smart contract deployment on the Neo N3 blockchain.
/// It shows how to create NEF files, prepare manifests, build deployment transactions,
/// and handle the complete deployment process.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🚀 Neo N3 Smart Contract Deployment Example");
	println!("==========================================");

	// 1. Connect to Neo N3 TestNet
	println!("\n📡 1. Connecting to Neo N3 TestNet...");
	let endpoints = vec![
		"https://testnet1.neo.org:443/",
		"https://testnet2.neo.org:443/",
		"http://seed1t5.neo.org:20332",
		"http://seed2t5.neo.org:20332",
		"http://seed3t5.neo.org:20332",
	];

	let mut client = None;
	for endpoint in endpoints {
		match HttpProvider::new(endpoint) {
			Ok(provider) => {
				let test_client = RpcClient::new(provider);
				// Test connection with a simple call
				match test_client.get_block_count().await {
					Ok(count) => {
						println!("   ✅ Connected to: {}", endpoint);
						println!("   📦 Current block height: {}", count);
						client = Some(test_client);
						break;
					},
					Err(_) => continue,
				}
			},
			Err(_) => continue,
		}
	}

	let client = client.ok_or("Failed to connect to any endpoint")?;

	// 2. Load deployer account (using dummy account for example)
	println!("\n🔐 2. Loading deployer account...");
	// In a real deployment, load from WIF or keystore
	let deployer_wif = "KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr"; // Example WIF
	let deployer_account = Account::from_wif(deployer_wif)?;
	let deployer_address = deployer_account.get_address();
	println!("   📍 Deployer address: {}", deployer_address);

	// Check deployer GAS balance (needed for deployment fees)
	let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
	let deployer_script_hash = deployer_account.get_script_hash();

	match client
		.invoke_function(
			&gas_token,
			"balanceOf".to_string(),
			vec![ContractParameter::h160(&deployer_script_hash)],
			None,
		)
		.await
	{
		Ok(result) =>
			if let Some(balance_item) = result.stack.first() {
				let balance = balance_item.as_int().unwrap_or(0);
				println!("   💰 Deployer GAS balance: {} GAS", balance as f64 / 100_000_000.0);
			},
		Err(e) => {
			println!("   ⚠️  Could not fetch GAS balance: {}", e);
			println!("   💡 Make sure the account has GAS for deployment");
		},
	}

	// 3. Create NEF file for a simple contract
	println!("\n📦 3. Creating NEF file...");
	let nef_file = create_sample_nef_file()?;
	println!("   ✅ NEF file created");
	// Note: NEF file script field is private, showing concept only
	println!("   📏 NEF file created with embedded script");
	println!("   🔢 Checksum: 0x{:08x}", nef_file.checksum);

	// 4. Create contract manifest
	println!("\n📜 4. Creating contract manifest...");
	let manifest = create_sample_manifest()?;
	println!("   ✅ Manifest created");
	println!("   📝 Contract name: {}", manifest.name.as_ref().unwrap_or(&"Unknown".to_string()));
	println!("   🎯 Supported standards: {:?}", manifest.supported_standards);

	// 5. Calculate deployment costs
	println!("\n💰 5. Calculating deployment costs...");
	let deployment_cost = calculate_deployment_cost(&nef_file, &manifest);
	println!("   💵 Estimated deployment cost: {} GAS", deployment_cost);
	println!("   📏 NEF size: {} bytes", nef_file.script.len());
	println!("   📜 Manifest size: {} bytes", serde_json::to_string(&manifest)?.len());

	// 6. Build deployment transaction
	println!("\n🔨 6. Building deployment transaction...");

	// Get contract management hash
	let contract_mgmt = neo3::neo_contract::ContractManagement::new();
	let mgmt_hash = contract_mgmt.hash();
	println!("   📋 ContractManagement: 0x{}", mgmt_hash);

	// Build deployment script
	let mut script_builder = neo3::neo_builder::ScriptBuilder::new();

	// Serialize NEF and manifest
	let nef_bytes = serialize_nef(&nef_file)?;
	let manifest_json = serde_json::to_string(&manifest)?;

	// Push parameters for deploy method
	script_builder.push(manifest_json.as_bytes()); // manifest as string
	script_builder.push(&nef_bytes); // NEF file bytes

	// Call deploy method
	script_builder.contract_call(mgmt_hash, "deploy", &[], neo3::neo_builder::CallFlags::All)?;

	let deployment_script = script_builder.to_bytes();
	println!("   ✅ Deployment script created ({} bytes)", deployment_script.len());

	// 7. Create and configure transaction
	println!("\n📝 7. Creating deployment transaction...");

	// Build transaction
	let mut tx_builder = neo3::neo_builder::TransactionBuilder::with_client(&client);
	tx_builder.set_script(Some(deployment_script));
	// valid_until_block will be set automatically

	// Add signer for the deployer account
	let signer = neo3::neo_builder::AccountSigner::called_by_entry_hash160(
		deployer_account.get_script_hash(),
	)?;
	tx_builder.set_signers(vec![neo3::neo_builder::Signer::AccountSigner(signer)])?;

	// Get current block for valid_until_block
	match client.get_block_count().await {
		Ok(height) => {
			tx_builder.valid_until_block(height + 1000); // Valid for ~250 minutes
			println!("   ⏰ Transaction valid until block: {}", height + 1000);
		},
		Err(e) => {
			println!("   ⚠️  Could not get block height: {}", e);
			tx_builder.valid_until_block(1000000); // Use a far future block
		},
	}

	// Calculate network fee
	println!("\n⛽ 8. Calculating network fee...");
	let base_fee = 0.001; // Base network fee
	let size_fee = deployment_script.len() as f64 * 0.00001; // Fee per byte
	let network_fee = base_fee + size_fee + deployment_cost;
	tx_builder.network_fee((network_fee * 100_000_000.0) as u64); // Convert to GAS fractions
	println!("   💵 Network fee: {} GAS", network_fee);
	println!("   💵 Total cost: {} GAS", network_fee + deployment_cost);

	// 9. Sign transaction (would be done with real key)
	println!("\n✍️ 9. Signing transaction...");
	println!("   ⚠️  In production, sign with secure key management");
	println!("   🔐 Using witness scope: CalledByEntry");

	// Create witness (simplified for example)
	let witness = neo3::neo_builder::Witness {
		invocation_script: vec![0x0C, 0x40], // Placeholder signature
		verification_script: deployer_account.get_verification_script()?,
	};
	tx_builder.add_witness(witness);

	// 10. Deployment simulation
	println!("\n🚀 10. Deployment Process (Simulation)...");
	println!("   ⚠️  Actual deployment requires:");
	println!("      • Valid private key for signing");
	println!("      • Sufficient GAS balance");
	println!("      • Network connectivity");

	// Show what would happen
	println!("\n   📋 Deployment steps that would execute:");
	println!("   1. Transaction would be broadcast to network");
	println!("   2. Validators would verify the transaction");
	println!("   3. Contract would be stored on blockchain");
	println!("   4. Contract hash would be calculated as:");

	// Calculate expected contract hash
	let expected_hash =
		calculate_contract_hash(&deployer_account.get_script_hash()?, &nef_file, &manifest)?;
	println!("      🔑 Expected contract hash: 0x{}", expected_hash);

	println!("   5. Contract would be immediately available for invocation");

	// 11. Post-deployment verification
	println!("\n✅ 11. Post-Deployment Verification Steps...");
	println!("   After successful deployment:");
	println!("   • Query contract state: client.get_contract_state(hash)");
	println!("   • Test contract methods: client.invoke_function(hash, method, params)");
	println!("   • Monitor contract events: client.get_application_log(tx_id)");
	println!("   • Verify storage: client.find_states(hash, prefix)");

	// 12. Example contract invocation
	println!("\n📞 12. Example Contract Invocation...");
	println!("   Once deployed, invoke methods like:");
	println!("   ```rust");
	println!("   let result = client.invoke_function(");
	println!("       &contract_hash,");
	println!("       \"getValue\",");
	println!("       &[],");
	println!("   ).await?;");
	println!("   ```");

	println!("\n🎉 Contract deployment example completed!");
	println!("💡 This example demonstrated the complete deployment process");
	println!("💡 For production deployments, ensure proper key management and testing");

	Ok(())
}

/// Create sample contract bytecode that returns 42
fn create_sample_contract_bytecode() -> Vec<u8> {
	// This is a simple Neo VM script that pushes 42 and returns
	// OpCode structure: PUSH1 (0x51) + RET (0x40)
	vec![
		0x15, // PUSH 21 (decimal 21)
		0x15, // PUSH 21 (decimal 21)
		0x93, // ADD operation (21 + 21 = 42)
		0x40, // RET
	]
}

/// Create a sample NEF file
fn create_sample_nef_file() -> Result<neo3::neo_types::NefFile, Box<dyn std::error::Error>> {
	use neo3::neo_types::*;

	let script = create_sample_contract_bytecode();

	// Calculate checksum (simplified - real checksum would use CRC32)
	let checksum = script.iter().fold(0u32, |acc, &byte| acc.wrapping_add(byte as u32));

	Ok(NefFile {
		magic: 0x3346454E, // "NEF3" in hex
		compiler: "NeoRust Example Compiler v1.0".to_string(),
		source: "https://github.com/R3E-Network/NeoRust".to_string(),
		tokens: vec![],
		script,
		checksum,
	})
}

/// Create a sample contract manifest
fn create_sample_manifest() -> Result<neo3::neo_types::ContractManifest, Box<dyn std::error::Error>>
{
	use neo3::neo_types::*;

	Ok(ContractManifest {
		name: Some("SampleContract".to_string()),
		groups: vec![],
		features: HashMap::new(),
		supported_standards: vec![],
		abi: Some(ContractABI {
			methods: vec![ContractMethod {
				name: "getValue".to_string(),
				parameters: vec![],
				return_type: ContractParameterType::Integer,
				offset: 0,
				safe: true,
			}],
			events: vec![ContractEvent {
				name: "Deployed".to_string(),
				parameters: vec![ContractParameter::string("deployer".to_string())],
			}],
		}),
		permissions: vec![ContractPermission {
			contract: ContractPermissionDescriptor::Any,
			methods: vec!["*".to_string()],
		}],
		trusts: vec![],
		extra: None,
	})
}

/// Calculate deployment cost based on NEF and manifest size
fn calculate_deployment_cost(
	nef: &neo3::neo_types::NefFile,
	manifest: &neo3::neo_types::ContractManifest,
) -> f64 {
	let base_deployment_fee = 10.0; // Base fee in GAS
	let nef_size = nef.script.len();
	let manifest_size = serde_json::to_string(manifest).unwrap_or_default().len();

	// Fee calculation: base + size-based fees
	let size_fee = ((nef_size + manifest_size) as f64) * 0.001; // 0.001 GAS per byte

	base_deployment_fee + size_fee
}

/// Serialize NEF file to bytes
fn serialize_nef(nef: &neo3::neo_types::NefFile) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let mut bytes = Vec::new();

	// Magic number (4 bytes)
	bytes.extend_from_slice(&nef.magic.to_le_bytes());

	// Compiler (64 bytes padded)
	let compiler_bytes = nef.compiler.as_bytes();
	let mut compiler_padded = vec![0u8; 64];
	compiler_padded[..compiler_bytes.len().min(64)]
		.copy_from_slice(&compiler_bytes[..compiler_bytes.len().min(64)]);
	bytes.extend_from_slice(&compiler_padded);

	// Source URL (variable length string)
	bytes.push(nef.source.len() as u8);
	bytes.extend_from_slice(nef.source.as_bytes());

	// Reserved byte
	bytes.push(0x00);

	// Method tokens (empty for our example)
	bytes.push(0x00); // Empty array

	// Reserved bytes
	bytes.extend_from_slice(&[0x00, 0x00]);

	// Script
	bytes.push(nef.script.len() as u8);
	bytes.extend_from_slice(&nef.script);

	// Checksum
	bytes.extend_from_slice(&nef.checksum.to_le_bytes());

	Ok(bytes)
}

/// Calculate expected contract hash after deployment
fn calculate_contract_hash(
	sender: &neo3::neo_types::ScriptHash,
	nef: &neo3::neo_types::NefFile,
	manifest: &neo3::neo_types::ContractManifest,
) -> Result<neo3::neo_types::ScriptHash, Box<dyn std::error::Error>> {
	use neo3::neo_crypto::Hash;

	// Contract hash = SHA256(sender + nef_checksum + manifest_name)
	let mut data = Vec::new();
	data.extend_from_slice(&sender.as_bytes());
	data.extend_from_slice(&nef.checksum.to_le_bytes());
	data.extend_from_slice(manifest.name.as_ref().unwrap_or(&"".to_string()).as_bytes());

	let hash = data.hash256();
	Ok(neo3::neo_types::ScriptHash::from_slice(&hash[..20])?)
}
