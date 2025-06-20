use super::utils::{assert_output_contains, assert_success, CliTest};

#[test]
fn test_blockchain_info() {
	let cli = CliTest::new();

	// Get blockchain info
	let output = cli.run(&["network", "status"]);

	assert_success(&output);
	// Any Neo node will return information containing these values
	assert_output_contains(&output, "Network");
	assert_output_contains(&output, "Block Height");
}

#[test]
fn test_blockchain_height() {
	let cli = CliTest::new();

	// Get blockchain height
	let output = cli.run(&["network", "block"]);

	assert_success(&output);
	// Should return a numeric height
	assert_output_contains(&output, "Current block height:");
}

#[test]
fn test_blockchain_get_block_by_index() {
	let cli = CliTest::new();

	// Try to get block 0 (genesis block)
	let output = cli.run(&["network", "block", "--index", "0"]);

	assert_success(&output);
	// Genesis block details
	assert_output_contains(&output, "Block Hash");
	assert_output_contains(&output, "Timestamp");
}

#[test]
fn test_blockchain_get_block_by_hash() {
	let cli = CliTest::new();

	// First get genesis block hash
	let info_output = cli.run(&["network", "block", "--index", "0"]);
	assert_success(&info_output);

	// Extract block hash from output
	let stdout = String::from_utf8_lossy(&info_output.stdout);
	let hash_line = stdout
		.lines()
		.find(|line| line.contains("Block Hash"))
		.expect("Should contain Block Hash line");

	// Extract hash part (assuming format "Block Hash: 0x...")
	let parts: Vec<&str> = hash_line.split(": ").collect();
	let hash = parts.get(1).expect("Should have hash part").trim();

	// Get block by hash
	let output = cli.run(&["network", "block", "--hash", hash]);

	assert_success(&output);
	assert_output_contains(&output, "Block Hash");
	assert_output_contains(&output, hash); // Should contain same hash
}

#[test]
fn test_blockchain_get_asset() {
	let cli = CliTest::new();

	// Get NEO asset info (use de-fi token command instead)
	let output = cli.run(&["de-fi", "token", "--symbol", "NEO"]);

	assert_success(&output);
	assert_output_contains(&output, "Asset Information");
	assert_output_contains(&output, "NEO");
}
