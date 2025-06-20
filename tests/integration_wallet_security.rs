use neo3::{
	neo_protocol::{Account, AccountTrait},
	neo_wallets::{Wallet, WalletBackup, WalletTrait},
};
use tempfile::TempDir;

#[tokio::test]
async fn test_complete_wallet_lifecycle() {
	// 1. Create wallet with multiple accounts
	let mut wallet = Wallet::new();
	wallet.set_name("Integration Test Wallet".to_string());

	let mut accounts = Vec::new();
	for _i in 0..5 {
		let account = Account::create().expect("Should create account");
		accounts.push(account.get_address());
		wallet.add_account(account);
	}

	assert_eq!(wallet.accounts().len(), 6); // 5 created + 1 default account

	// 2. Test password operations
	let password = "integration_test_password_123!";
	wallet.encrypt_accounts(password);

	// Verify all accounts are encrypted
	for account in wallet.accounts() {
		assert!(account.encrypted_private_key().is_some(), "Account should be encrypted");
		assert!(account.key_pair().is_none(), "Account should not have unencrypted key pair");
	}

	// 3. Test password verification
	assert!(wallet.verify_password(password), "Correct password should verify");
	assert!(!wallet.verify_password("wrong_password"), "Wrong password should not verify");

	// 4. Test backup and recovery
	let temp_dir = TempDir::new().expect("Should create temp dir");
	let backup_path = temp_dir.path().join("integration_test_wallet.json");

	WalletBackup::backup(&wallet, backup_path.clone()).expect("Should backup wallet");

	assert!(backup_path.exists(), "Backup file should exist");

	// 5. Test recovery
	let recovered_wallet =
		WalletBackup::recover(backup_path.clone()).expect("Should recover wallet");

	assert_eq!(wallet.name(), recovered_wallet.name());
	assert_eq!(wallet.accounts().len(), recovered_wallet.accounts().len());

	// Verify all accounts are recovered correctly
	let original_addresses: Vec<String> =
		wallet.accounts().iter().map(neo3::neo_protocol::Account::get_address).collect();
	let recovered_addresses: Vec<String> = recovered_wallet
		.accounts()
		.iter()
		.map(neo3::neo_protocol::Account::get_address)
		.collect();

	for addr in &original_addresses {
		assert!(recovered_addresses.contains(addr), "Address {addr} should be recovered");
	}

	// 6. Test password change
	let new_password = "new_integration_password_456!";
	let mut test_wallet = wallet.clone();
	test_wallet
		.change_password(password, new_password)
		.expect("Should change password");

	assert!(test_wallet.verify_password(new_password), "New password should verify");
	assert!(!test_wallet.verify_password(password), "Old password should not verify");
}

#[tokio::test]
async fn test_wallet_security_edge_cases() {
	let mut wallet = Wallet::new();
	let account = Account::create().expect("Should create account");
	wallet.add_account(account);

	// Test short password (empty passwords are not allowed)
	let mut test_wallet = wallet.clone();
	test_wallet.encrypt_accounts("a");
	assert!(test_wallet.verify_password("a"), "Short password should work");

	// Test very long password
	let long_password = "a".repeat(1000);
	let mut test_wallet2 = wallet.clone();
	test_wallet2.encrypt_accounts(&long_password);
	assert!(test_wallet2.verify_password(&long_password), "Long password should work");

	// Test special characters in password
	let special_password = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
	let mut test_wallet3 = wallet.clone();
	test_wallet3.encrypt_accounts(special_password);
	assert!(
		test_wallet3.verify_password(special_password),
		"Special character password should work"
	);
}

#[tokio::test]
async fn test_large_wallet_performance() {
	let mut wallet = Wallet::new();

	// Create wallet with many accounts
	let account_count = 50;
	for _ in 0..account_count {
		let account = Account::create().expect("Should create account");
		wallet.add_account(account);
	}

	assert_eq!(wallet.accounts().len(), account_count + 1); // +1 for default account

	// Test encryption performance
	let start = std::time::Instant::now();
	wallet.encrypt_accounts("performance_test_password");
	let encryption_time = start.elapsed();

	println!("Encrypted {account_count} accounts in {encryption_time:?}");
	assert!(encryption_time.as_secs() < 15, "Encryption should complete within 15 seconds");

	// Test backup performance
	let temp_dir = TempDir::new().expect("Should create temp dir");
	let backup_path = temp_dir.path().join("performance_test_wallet.json");

	let start = std::time::Instant::now();
	WalletBackup::backup(&wallet, backup_path.clone()).expect("Should backup wallet");
	let backup_time = start.elapsed();

	println!("Backed up {account_count} accounts in {backup_time:?}");
	assert!(backup_time.as_secs() < 2, "Backup should complete within 2 seconds");

	// Test recovery performance
	let start = std::time::Instant::now();
	let _recovered_wallet = WalletBackup::recover(backup_path).expect("Should recover wallet");
	let recovery_time = start.elapsed();

	println!("Recovered {account_count} accounts in {recovery_time:?}");
	assert!(recovery_time.as_secs() < 2, "Recovery should complete within 2 seconds");
}

#[tokio::test]
async fn test_concurrent_wallet_operations() {
	use std::sync::Arc;
	use tokio::sync::Mutex;

	let wallet = Arc::new(Mutex::new(Wallet::new()));

	// Test concurrent account creation
	let mut handles = Vec::new();
	for i in 0..10 {
		let wallet_clone = Arc::clone(&wallet);
		let handle = tokio::spawn(async move {
			let account = Account::create().expect("Should create account");
			let mut wallet_guard = wallet_clone.lock().await;
			wallet_guard.add_account(account);
			i
		});
		handles.push(handle);
	}

	// Wait for all tasks to complete
	for handle in handles {
		handle.await.expect("Task should complete");
	}

	let wallet_guard = wallet.lock().await;
	assert_eq!(wallet_guard.accounts().len(), 11, "Should have 10 created + 1 default account");
}

#[tokio::test]
async fn test_backup_file_integrity() {
	let mut wallet = Wallet::new();
	let account = Account::create().expect("Should create account");
	wallet.add_account(account);
	wallet.encrypt_accounts("integrity_test_password");

	let temp_dir = TempDir::new().expect("Should create temp dir");
	let backup_path = temp_dir.path().join("integrity_test_wallet.json");

	// Create backup
	WalletBackup::backup(&wallet, backup_path.clone()).expect("Should backup wallet");

	// Verify backup file is valid JSON
	let backup_content = std::fs::read_to_string(&backup_path).expect("Should read backup file");

	let _json_value: serde_json::Value =
		serde_json::from_str(&backup_content).expect("Backup should be valid JSON");

	// Test recovery from the JSON
	let recovered_wallet = WalletBackup::recover(backup_path).expect("Should recover from backup");

	assert_eq!(wallet.accounts().len(), recovered_wallet.accounts().len());
}
