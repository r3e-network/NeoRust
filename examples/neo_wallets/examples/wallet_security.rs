use neo3::{
	neo_protocol::{Account, AccountTrait},
	neo_wallets::Wallet,
	prelude::*,
};

/// This example demonstrates comprehensive wallet security features in Neo N3.
/// It covers encryption, password management, and secure key handling.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🔐 Neo N3 Wallet Security Example");
	println!("=================================");

	// 1. Create a wallet with multiple accounts
	println!("\n1. Creating wallet with multiple accounts...");
	let mut wallet = Wallet::new();
	wallet.set_name("SecureWallet".to_string());

	// Add multiple accounts
	for i in 1..=3 {
		let account = Account::create()?;
		println!("   Created account {}: {}", i, account.get_address());
		wallet.add_account(account);
	}

	println!("   ✅ Wallet created with {} accounts", wallet.accounts().len());

	// 2. Demonstrate password-based encryption
	println!("\n2. Encrypting wallet with password...");
	let password = "SuperSecurePassword123!@#";

	// Verify accounts have key pairs before encryption
	println!("   Before encryption:");
	for (i, account) in wallet.accounts().iter().enumerate() {
		let has_key_pair = account.key_pair().is_some();
		println!("     Account {}: Key pair present = {}", i + 1, has_key_pair);
	}

	// Encrypt all accounts
	wallet.encrypt_accounts(password);
	println!("   ✅ All accounts encrypted successfully");

	// Verify accounts no longer have unencrypted key pairs
	println!("   After encryption:");
	for (i, account) in wallet.accounts().iter().enumerate() {
		let has_key_pair = account.key_pair().is_some();
		let has_encrypted_key = account.encrypted_private_key().is_some();
		println!(
			"     Account {}: Key pair present = {}, Encrypted key present = {}",
			i + 1,
			has_key_pair,
			has_encrypted_key
		);
	}

	// 3. Password verification
	println!("\n3. Testing password verification...");

	// Test correct password
	let is_correct = wallet.verify_password(password);
	println!(
		"   Correct password verification: {}",
		if is_correct { "✅ PASS" } else { "❌ FAIL" }
	);

	// Test incorrect password
	let is_incorrect = wallet.verify_password("WrongPassword");
	println!(
		"   Incorrect password verification: {}",
		if !is_incorrect { "✅ PASS" } else { "❌ FAIL" }
	);

	// 4. Demonstrate password change
	println!("\n4. Changing wallet password...");
	let new_password = "NewSecurePassword456$%^";

	match wallet.change_password(password, new_password) {
		Ok(_) => {
			println!("   ✅ Password changed successfully");

			// Verify old password no longer works
			let old_works = wallet.verify_password(password);
			println!(
				"   Old password still works: {}",
				if !old_works { "✅ NO" } else { "❌ YES" }
			);

			// Verify new password works
			let new_works = wallet.verify_password(new_password);
			println!("   New password works: {}", if new_works { "✅ YES" } else { "❌ NO" });
		},
		Err(e) => println!("   ❌ Password change failed: {}", e),
	}

	// 5. Demonstrate account decryption for signing
	println!("\n5. Demonstrating secure account access...");

	if let Some(first_account) = wallet.accounts().first() {
		let account_address = first_account.get_address();
		println!("   Accessing account: {}", account_address);

		// This would be used for signing transactions
		match wallet.get_signing_account(&account_address, new_password) {
			Ok(signing_account) => {
				println!("   ✅ Account successfully unlocked for signing");
				println!("   Account has key pair: {}", signing_account.key_pair().is_some());
			},
			Err(e) => println!("   ❌ Failed to unlock account: {}", e),
		}
	}

	// 6. Security best practices summary
	println!("\n6. 🛡️ Security Best Practices:");
	println!("   • Use strong, unique passwords (12+ characters)");
	println!("   • Include uppercase, lowercase, numbers, and symbols");
	println!("   • Never store passwords in plain text");
	println!("   • Regularly change wallet passwords");
	println!("   • Keep encrypted wallet backups in multiple secure locations");
	println!("   • Never share private keys or encrypted key files");
	println!("   • Use hardware wallets for large amounts");
	println!("   • Verify wallet integrity after recovery");
	println!("   • Use multi-signature for critical operations");
	println!("   • Keep software updated");

	// 7. Demonstrate password strength validation
	println!("\n7. Password strength examples:");
	let test_passwords = vec![
		("weak", "Weak password"),
		("StrongPassword123!", "Strong password"),
		("VeryLongAndComplexPassword2024!@#$", "Very strong password"),
	];

	for (password, description) in test_passwords {
		let strength = evaluate_password_strength(password);
		println!("   {} ({}): {}", description, password.len(), strength);
	}

	println!("\n✅ Wallet security demonstration completed successfully!");
	println!("   Remember: Security is paramount in blockchain applications!");

	Ok(())
}

/// Simple password strength evaluation
fn evaluate_password_strength(password: &str) -> &'static str {
	let length = password.len();
	let has_upper = password.chars().any(|c| c.is_uppercase());
	let has_lower = password.chars().any(|c| c.is_lowercase());
	let has_digit = password.chars().any(|c| c.is_numeric());
	let has_special = password.chars().any(|c| !c.is_alphanumeric());

	let criteria_met =
		[has_upper, has_lower, has_digit, has_special].iter().filter(|&&x| x).count();

	match (length, criteria_met) {
		(0..=7, _) => "Very Weak",
		(8..=11, 0..=2) => "Weak",
		(8..=11, 3..=4) => "Medium",
		(12..=15, 3..=4) => "Strong",
		(16.., 4) => "Very Strong",
		_ => "Medium",
	}
}
