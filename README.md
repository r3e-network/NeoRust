# NeoRust

[![Rust CI](https://github.com/R3E-Network/NeoRust/workflows/Rust%20CI/badge.svg)](https://github.com/R3E-Network/NeoRust/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/neo3.svg)](https://crates.io/crates/neo3)

A comprehensive Rust SDK for the Neo N3 blockchain platform, providing a complete toolkit for interacting with Neo N3 networks.

## Features

- 🔐 **Cryptography** - Complete cryptographic functions including key generation, signing, and verification
- 💼 **Wallet Management** - Create, import, and manage Neo wallets with hardware wallet support
- 🔗 **RPC Client** - Full-featured RPC client for Neo N3 node interaction
- 📦 **Smart Contracts** - Deploy, invoke, and interact with Neo N3 smart contracts
- 🪙 **Token Support** - Native NEP-17 token operations and custom token support
- 🌐 **Network Support** - Mainnet, Testnet, and custom network configurations
- 🖥️ **CLI Tools** - Command-line interface for common blockchain operations
- 🖼️ **GUI Application** - Desktop GUI application built with Tauri and React

## Quick Start

Add NeoRust to your `Cargo.toml`:

```toml
[dependencies]
neo3 = "0.4.1"
```

## Basic Usage

```rust
use neo3::prelude::*;

// Create a new wallet
let wallet = Wallet::new().unwrap();

// Connect to Neo testnet
let client = RpcClient::new("https://testnet1.neo.coz.io:443").unwrap();

// Get account balance
let balance = client.get_balance(&wallet.address()).await?;
println!("Balance: {} NEO", balance.neo);
```

## Components

### Core SDK (`neo3`)
The main Rust SDK providing all blockchain functionality.

### CLI Tool (`neo-cli`)
Command-line interface for blockchain operations:
```bash
cargo run --bin neo-cli -- wallet create
```

### GUI Application (`neo-gui`)
Desktop application with modern React UI. **Note:** Requires GTK libraries on Linux.

## Building

### Core SDK and CLI
```bash
cargo build --workspace --exclude neo-gui
```

### GUI Application (requires additional dependencies)

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
cd neo-gui && npm install && cargo build
```

**macOS and Windows:**
```bash
cd neo-gui && npm install && cargo build
```

## Documentation

- [Getting Started Guide](docs/guides/getting-started.md)
- [API Documentation](https://docs.rs/neo3)
- [Examples](examples/)

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to our repository.