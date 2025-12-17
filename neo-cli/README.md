# Neo CLI

A command-line interface for interacting with the Neo N3 blockchain, built on the NeoRust SDK (`neo3`).

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Neo CLI wraps common SDK workflows into a single tool: network connectivity checks, contract operations, DeFi helpers, NeoFS commands, and project generation templates.

> Note: some subcommands are still placeholders and may return “NotImplemented”. Use `neo-cli --help` for the authoritative list of available commands.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust

# Build the CLI tool
cargo build --release -p neo-cli

# Run the CLI
./target/release/neo-cli --help
```

### Using Cargo

```bash
# If/when published to crates.io:
cargo install neo-cli
```

## Quick Start

```bash
# Show top-level help
neo-cli --help

# Initialize configuration (optional; creates a config file in your OS config dir)
neo-cli init

# Create a wallet file (prompts for password unless provided)
neo-cli wallet create --path my-wallet.json

# Connect to a network (interactive if omitted)
neo-cli network connect --network testnet

# Inspect network state
neo-cli network status
neo-cli network block

# Token/DeFi helpers
neo-cli de-fi token NEO
neo-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz

# Check NeoFS connection status
neo-cli fs status

# Generate a new project from templates
neo-cli generate --list
neo-cli generate --template nep17-token my-token
```

## Command Reference

Run `neo-cli --help` (and `neo-cli <command> --help`) for the full set of flags and subcommands.

### Network

- `neo-cli network connect`: connect to an RPC endpoint / named network
- `neo-cli network status`: show basic network information
- `neo-cli network peers`: list peers (requires a working connection)
- `neo-cli network block`: fetch latest (or specified) block

### Wallet

- `neo-cli wallet create`, `open`, `backup`, `restore`, `hd-wallet`
- `neo-cli wallet send` and `neo-cli wallet balance` exist, but some on-chain operations are still stubbed; for now, `neo-cli de-fi token` and `neo-cli de-fi balance <TOKEN> <ADDRESS>` are the most reliable token helpers.

### Contracts

- `neo-cli contract deploy`, `update`, `invoke`, `list-native-contracts`

### DeFi

- `neo-cli de-fi token` and `neo-cli de-fi balance` for token metadata/balance queries
- `neo-cli de-fi transfer` exists, but signing workflows are still evolving (see subcommand `--help`)
- Protocol helpers: `neo-cli de-fi flamingo ...`, `neo-cli de-fi neo-burger ...`, `neo-cli de-fi neo-compound ...`, `neo-cli de-fi grand-share ...`

### NeoFS

- `neo-cli fs ...`: endpoints, container, object, status
- `neo-cli neo-fs ...`: advanced NeoFS commands (acl/config/status)

```bash
# List all available NeoFS endpoints for mainnet
neo-cli fs endpoints list

# Test connection to a specific endpoint
neo-cli fs endpoints test --endpoint grpc.mainnet.fs.neo.org:8082

# Get detailed information about an endpoint
neo-cli fs endpoints info --endpoint grpc.mainnet.fs.neo.org:8082

# Create a container
neo-cli fs container create --config container-config.json

# Upload a file
neo-cli fs object put --container CID --file path/to/file

# Download a file
neo-cli fs object get --container CID --id OID --output path/to/save
```

## Configuration

Neo CLI uses a configuration file to store settings like network preferences, RPC endpoints, and more. You can initialize the configuration with:

```bash
neo-cli init [--path /custom/path/config.json]
```

The default location for the configuration file is in your system's config directory under `neo-cli/config.json`.

## Testing

The Neo CLI includes comprehensive automated tests to ensure functionality and help with development.

### Running Tests

To run CLI tests:

```bash
cargo test -p neo-cli
```

### Test Structure

- **Unit Tests**: Test individual functions and components
- **Integration Tests**: Test the CLI commands from a user perspective
  - `defi_tests.rs`: Tests for DeFi and well-known contract commands
  - `fs_tests.rs`: Tests for NeoFS storage operations
  - `blockchain_tests.rs`: Tests for blockchain query commands
  - `wallet_tests.rs`: Tests for wallet management commands

### Writing New Tests

When adding new features to the CLI, follow this pattern for testing:

1. Create unit tests for new functions in the source files
2. Add integration tests in the appropriate test module
3. Run the tests to verify functionality
4. Ensure both success cases and error handling are tested

## Development

### Building from Source

```bash
cargo build [--release]
```

## License

MIT License

## Credits

Developed by the R3E Network team

---
Copyright © 2020-2025 R3E Network. All rights reserved.
