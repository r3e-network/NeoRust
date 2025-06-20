/*!
# Basic Wallet Creation Example

This example demonstrates how to create and manage wallets in NeoRust.

## What you'll learn:
- Creating new wallets with random keys
- Loading existing wallets from WIF/private keys
- Wallet address generation and validation
- Basic wallet security practices

## Security Notes:
- Never hardcode private keys in production
- Always use secure random generation
- Store private keys encrypted (NEP-2)
*/

use colored::*;
use hex;
use neo3::{
	neo_crypto::KeyPair,
	neo_types::{ScriptHash, ScriptHashExtension},
	prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("{}", "💼 NeoRust Basic Wallet Creation Example".cyan().bold());
	println!("{}", "=".repeat(50));

	// Example 1: Create a new wallet with random keys
	println!("\n{}", "🔑 Creating New Wallet...".yellow().bold());
	let key_pair = KeyPair::new_random();
	let script_hash = key_pair.get_script_hash();
	let address = script_hash.to_address();

	println!("  📍 Address: {}", address.to_string().green());
	println!("  🔐 Public Key: {}", hex::encode(key_pair.public_key().get_encoded_point(false).as_bytes()).dimmed());
	println!("  🎯 Script Hash: {}", key_pair.get_script_hash().to_string().dimmed());

	// Example 2: Create wallet from WIF
	println!("\n{}", "📥 Loading Wallet from WIF...".yellow().bold());
	let wif = key_pair.export_as_wif();
	println!("  💾 WIF: {}", wif.dimmed());

	match KeyPair::from_wif(&wif) {
		Ok(loaded_key) => {
			let loaded_script_hash = loaded_key.get_script_hash();
			let loaded_address = loaded_script_hash.to_address();
			println!("  ✅ Loaded Address: {}", loaded_address.to_string().green());
			println!(
				"  🔍 Addresses Match: {}",
				if address == loaded_address { "Yes".green() } else { "No".red() }
			);
		},
		Err(e) => {
			println!("  ❌ Error loading from WIF: {}", e.to_string().red());
		},
	}

	// Example 3: Wallet validation
	println!("\n{}", "✅ Wallet Validation...".yellow().bold());
	validate_address(&address.to_string()).await;
	validate_address("NInvalidAddress123").await;

	// Example 4: Multiple wallet generation
	println!("\n{}", "🔄 Generating Multiple Wallets...".yellow().bold());
	for i in 1..=3 {
		let wallet_key = KeyPair::new_random();
		let wallet_script_hash = wallet_key.get_script_hash();
		let wallet_addr = wallet_script_hash.to_address();
		println!("  Wallet {}: {}", i.to_string().cyan(), wallet_addr.to_string().green());
	}

	println!("\n{}", "✅ Wallet creation examples completed!".green().bold());
	println!("\n{}", "🔒 Security Reminder:".yellow().bold());
	println!("  • Never share private keys or WIF strings");
	println!("  • Use NEP-2 encryption for key storage");
	println!("  • Always backup wallet files securely");

	Ok(())
}

async fn validate_address(address_str: &str) {
	print!("  🔍 Validating {}: ", address_str.cyan());

	match ScriptHash::from_address(address_str) {
		Ok(_) => {
			println!("{}", "✅ Valid".green());
		},
		Err(_) => {
			println!("{}", "❌ Invalid".red());
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_wallet_creation() {
		let key_pair = KeyPair::new_random();
		let script_hash = key_pair.get_script_hash();
		let address = script_hash.to_address();

		// Wallet should have valid address format
		assert!(address.to_string().starts_with('N'));
		assert_eq!(address.to_string().len(), 34);
	}

	#[test]
	fn test_wif_roundtrip() {
		let original_key = KeyPair::new_random();
		let wif = original_key.export_as_wif();
		let loaded_key = KeyPair::from_wif(&wif).unwrap();

		// Keys should be identical after WIF roundtrip
		assert_eq!(original_key.private_key().to_raw_bytes(), loaded_key.private_key().to_raw_bytes());
	}

	#[test]
	fn test_address_validation() {
		// Valid addresses should parse successfully
		let key_pair = KeyPair::new_random();
		let script_hash = key_pair.get_script_hash();
		let address = script_hash.to_address();
		let address_str = address.to_string();

		assert!(ScriptHash::from_address(&address_str).is_ok());

		// Invalid addresses should fail
		assert!(ScriptHash::from_address("InvalidAddress").is_err());
		assert!(ScriptHash::from_address("N").is_err());
	}
}
