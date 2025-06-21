use super::utils::{assert_output_contains, assert_success, CliTest};

const TEST_WALLET_PASSWORD: &str = "password123";

#[test]
fn test_wallet_create() {
	let cli = CliTest::new();

	// Create wallet
	let wallet_path = cli.temp_dir.path().join("test-wallet.json").to_string_lossy().to_string();
	let output =
		cli.run(&["wallet", "create", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);

	assert_success(&output);
	assert_output_contains(&output, "Creating");

	// Verify file exists
	assert!(std::path::Path::new(&wallet_path).exists());
}

#[test]
fn test_wallet_open_and_close() {
	let cli = CliTest::new();

	// Create wallet first
	let wallet_path = cli.temp_dir.path().join("test-wallet.json").to_string_lossy().to_string();
	let create_output =
		cli.run(&["wallet", "create", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);
	assert_success(&create_output);

	// Test open wallet
	let open_output =
		cli.run(&["wallet", "open", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);
	assert_success(&open_output);
	assert_output_contains(&open_output, "Wallet opened successfully");

	// Test close wallet
	let close_output = cli.run(&["wallet", "close"]);
	assert_success(&close_output);
	assert_output_contains(&close_output, "Wallet closed");
}

#[test]
#[ignore] // Address creation not implemented yet
fn test_wallet_create_address() {
	let cli = CliTest::new();

	// Create and open wallet
	let wallet_path = cli.temp_dir.path().join("test-wallet.json").to_string_lossy().to_string();
	cli.run(&["wallet", "create", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);
	cli.run(&["wallet", "open", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);

	// Create a new address
	let output = cli.run(&["wallet", "create-address", "--count", "1"]);

	assert_success(&output);
	assert_output_contains(&output, "New address created");
}

#[test]
fn test_wallet_list_address() {
	let cli = CliTest::new();

	// Create and open wallet
	let wallet_path = cli.temp_dir.path().join("test-wallet.json").to_string_lossy().to_string();
	cli.run(&["wallet", "create", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);
	cli.run(&["wallet", "open", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);

	// List addresses
	let output = cli.run(&["wallet", "list"]);

	assert_success(&output);
	// Should contain address details (even if just showing there are no addresses)
	assert_output_contains(&output, "Address");
}

#[test]
#[ignore] // Address creation not implemented yet
fn test_wallet_balance() {
	let cli = CliTest::new();

	// Create and open wallet
	let wallet_path = cli.temp_dir.path().join("test-wallet.json").to_string_lossy().to_string();
	cli.run(&["wallet", "create", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);
	cli.run(&["wallet", "open", "--path", &wallet_path, "--password", TEST_WALLET_PASSWORD]);

	// Create an address
	cli.run(&["wallet", "create-address", "--count", "1"]);

	// Check balance (will be zero, but should run successfully)
	let output = cli.run(&["wallet", "balance"]);

	assert_success(&output);
}
