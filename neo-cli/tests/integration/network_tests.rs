use super::utils::{assert_output_contains, assert_success, CliTest};

#[test]
fn test_network_status() {
	let cli = CliTest::new();

	// Get network status
	let output = cli.run(&["network", "status"]);

	assert_success(&output);
	assert_output_contains(&output, "Network");
	assert_output_contains(&output, "Status");
}

#[test]
fn test_network_nodes() {
	let cli = CliTest::new();

	// List connected nodes
	let output = cli.run(&["network", "peers"]);

	assert_success(&output);
	assert_output_contains(&output, "Peers");
}

#[test]
fn test_network_switch() {
	let cli = CliTest::new();

	// Connect to TestNet
	let testnet_output = cli.run(&["network", "connect", "--network", "testnet"]);
	assert_success(&testnet_output);
	assert_output_contains(&testnet_output, "testnet");

	// Check network status to verify
	let status_output = cli.run(&["network", "status"]);
	assert_success(&status_output);
	assert_output_contains(&status_output, "testnet");

	// Connect to MainNet
	let mainnet_output = cli.run(&["network", "connect", "--network", "mainnet"]);
	assert_success(&mainnet_output);
	assert_output_contains(&mainnet_output, "mainnet");
}

#[test]
fn test_network_add_node() {
	let cli = CliTest::new();

	// Add a node
	let output = cli.run(&[
		"network",
		"add",
		"--url",
		"http://seed1.ngd.network:10332",
		"--name",
		"test-node",
	]);

	assert_success(&output);
	assert_output_contains(&output, "Node added");

	// Verify node is in the list
	let nodes_output = cli.run(&["network", "list"]);
	assert_success(&nodes_output);
	assert_output_contains(&nodes_output, "test-node");
}

#[test]
fn test_network_set_default() {
	let cli = CliTest::new();

	// First add a node
	cli.run(&[
		"network",
		"add",
		"--url",
		"http://seed2.ngd.network:10332",
		"--name",
		"default-node",
	]);

	// Connect to the node instead (no set-default command)
	let output = cli.run(&["network", "connect", "--network", "default-node"]);

	assert_success(&output);
	assert_output_contains(&output, "default-node");
}

#[test]
fn test_network_ping() {
	let cli = CliTest::new();

	// Ping a node
	let output = cli.run(&["network", "ping", "--network", "mainnet"]);

	assert_success(&output);
	// Either ping succeeds or times out but command should complete
	assert_output_contains(&output, "Ping");
}
