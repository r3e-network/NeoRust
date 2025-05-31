use base64;
use chrono;
use hex;
/// This example demonstrates the concept of message signing in Neo N3.
use neo3::prelude::*;
use neo3::{
	neo_crypto::hash::HashableForVec,
	neo_protocol::{Account, AccountTrait},
};
use serde_json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("🔐 Neo3 Message Signing Example");
	println!("==============================");

	// 1. Create or load an account
	println!("\n1. Creating account for message signing:");
	let wif = "L1WMhxazScMhUrdv34JqQb1HFSQmWeN2Kpc1R9JGKwL7CDNP21uR";
	let account = Account::from_wif(wif)?;
	println!("   ✅ Account loaded successfully");
	println!("   📍 Address: {}", account.get_address());
	println!("   🔑 Script Hash: {:?}", account.get_script_hash());

	// 2. Define the message to sign
	let message = "Hello, Neo N3 blockchain! This is a signed message.";
	println!("\n2. Message to sign:");
	println!("   📝 Message: \"{}\"", message);

	// 3. Sign the message
	println!("\n3. Signing the message:");
	let message_bytes = message.as_bytes();

	// Get the key pair from the account
	let key_pair = account
		.key_pair()
		.as_ref()
		.ok_or("Account does not have a key pair available")?;

	// Create a hash of the message (this is what we actually sign)
	let message_hash = message_bytes.hash256();
	println!("   🔐 Message hash: {}", hex::encode(&message_hash));

	// Sign the hash using the private key directly
	let signature = key_pair.private_key.sign_prehash(&message_hash)?;
	let signature_bytes = signature.to_bytes();
	println!("   ✅ Message signed successfully");
	println!("   📝 Signature: {}", hex::encode(&signature_bytes));
	println!("   📏 Signature length: {} bytes", signature_bytes.len());

	// 4. Verify the signature
	println!("\n4. Verifying the signature:");

	// Get the public key from the key pair
	let public_key = &key_pair.public_key;

	// Verify the signature
	let is_valid = public_key.verify(&message_hash, &signature).is_ok();

	if is_valid {
		println!("   ✅ Signature verification: VALID");
		println!("   🎉 The signature is authentic and was created by the account holder");
	} else {
		println!("   ❌ Signature verification: INVALID");
		println!("   ⚠️  The signature does not match the message or public key");
	}

	// 5. Demonstrate signature format variations
	println!("\n5. Signature format variations:");

	// Base64 encoding (common for web applications)
	let signature_base64 = base64::encode(&signature_bytes);
	println!("   📄 Base64: {}", signature_base64);

	// Hex encoding (common for blockchain applications)
	let signature_hex = hex::encode(&signature_bytes);
	println!("   🔢 Hex: {}", signature_hex);

	// 6. Create a verifiable message package
	println!("\n6. Creating verifiable message package:");
	let package = MessageSignaturePackage {
		message: message.to_string(),
		signature: signature_hex.clone(),
		public_key: hex::encode(public_key.get_encoded(true)),
		address: account.get_address(),
		timestamp: chrono::Utc::now().timestamp(),
	};

	let package_json = serde_json::to_string_pretty(&package)?;
	println!("   📦 Verifiable package:");
	println!("{}", package_json);

	// 7. Verify the package
	println!("\n7. Verifying the package:");
	let verification_result = verify_message_package(&package)?;

	if verification_result {
		println!("   ✅ Package verification: VALID");
		println!("   🔒 The message package is authentic and complete");
	} else {
		println!("   ❌ Package verification: INVALID");
		println!("   ⚠️  The package has been tampered with or is malformed");
	}

	// 8. Demonstrate different message types
	println!("\n8. Signing different message types:");

	// JSON message
	let json_message =
		r#"{"action":"transfer","amount":100,"to":"NXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx"}"#;
	let json_hash = json_message.as_bytes().hash256();
	let json_signature = key_pair.private_key.sign_prehash(&json_hash)?;
	println!("   📋 JSON message signed: {}", hex::encode(json_signature.to_bytes()));

	// Binary data
	let binary_data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
	let binary_hash = binary_data.hash256();
	let binary_signature = key_pair.private_key.sign_prehash(&binary_hash)?;
	println!("   💾 Binary data signed: {}", hex::encode(binary_signature.to_bytes()));

	// Timestamp-based message
	let timestamp_message = format!("Action performed at {}", chrono::Utc::now().to_rfc3339());
	let timestamp_hash = timestamp_message.as_bytes().hash256();
	let timestamp_signature = key_pair.private_key.sign_prehash(&timestamp_hash)?;
	println!("   ⏰ Timestamped message signed: {}", hex::encode(timestamp_signature.to_bytes()));

	println!("\n🎯 Message signing example completed successfully!");
	println!("   This demonstrates how to:");
	println!("   • Sign arbitrary messages with Neo accounts");
	println!("   • Verify message signatures");
	println!("   • Create verifiable message packages");
	println!("   • Handle different message formats");
	println!("   • Use proper cryptographic practices");

	Ok(())
}

/// A complete message signature package for verification
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MessageSignaturePackage {
	message: String,
	signature: String,
	public_key: String,
	address: String,
	timestamp: i64,
}

/// Verify a message signature package
fn verify_message_package(package: &MessageSignaturePackage) -> Result<bool, Box<dyn Error>> {
	// Decode the public key
	let public_key = neo3::neo_crypto::Secp256r1PublicKey::from_encoded(&package.public_key)
		.ok_or("Invalid public key format")?;

	// Decode the signature
	let signature_bytes = hex::decode(&package.signature)?;
	let signature = neo3::neo_crypto::Secp256r1Signature::from_bytes(&signature_bytes)?;

	// Hash the message
	let message_hash = package.message.as_bytes().hash256();

	// Verify the signature
	let is_valid = public_key.verify(&message_hash, &signature).is_ok();

	// Additional verification: check if the public key matches the address
	let derived_address = neo3::neo_clients::public_key_to_address(&public_key);
	let address_matches = derived_address == package.address;

	Ok(is_valid && address_matches)
}
