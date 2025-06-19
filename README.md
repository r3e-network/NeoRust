# NeoRust - Professional Neo N3 SDK for Rust

<div align="center">
  <p>
    <img src="assets/images/neo-logo.png" alt="Neo Logo" width="125" align="middle"/>&nbsp;&nbsp;&nbsp;&nbsp;
    <img src="assets/images/neo-x-logo.png" alt="Neo X Logo" width="80" align="middle"/>&nbsp;&nbsp;&nbsp;&nbsp;
    <img src="assets/images/r3e-logo.png" alt="R3E Logo" width="300" align="middle"/>
  </p>
  
  <h3>🚀 The Complete Neo N3 Development Suite</h3>
  <p><strong>Production-Ready SDK • Enterprise GUI • Powerful CLI</strong></p>
  
  [![CI](https://github.com/R3E-Network/NeoRust/actions/workflows/ci.yml/badge.svg)](https://github.com/R3E-Network/NeoRust/actions/workflows/ci.yml)
  [![Security](https://github.com/R3E-Network/NeoRust/actions/workflows/security.yml/badge.svg)](https://github.com/R3E-Network/NeoRust/actions/workflows/security.yml)
  [![Documentation](https://github.com/R3E-Network/NeoRust/actions/workflows/docs.yml/badge.svg)](https://github.com/R3E-Network/NeoRust/actions/workflows/docs.yml)
  [![Crates.io](https://img.shields.io/crates/v/neo3.svg)](https://crates.io/crates/neo3)
  [![Documentation](https://docs.rs/neo3/badge.svg)](https://docs.rs/neo3)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
</div>

---

**NeoRust** is the most comprehensive and production-ready Rust SDK for the Neo N3 blockchain ecosystem. Built with enterprise requirements in mind, it provides everything needed to build, deploy, and manage Neo applications at scale.

## ✨ Key Features

- **🔧 Complete Neo N3 Protocol** - Full implementation of Neo N3 blockchain protocol
- **🎨 Professional GUI** - Enterprise-grade desktop application built with Tauri
- **💻 Powerful CLI** - Feature-rich command-line interface for automation
- **🚀 Production Ready** - Battle-tested with comprehensive test coverage
- **🔐 Security First** - Industry-standard cryptography and secure key management
- **📚 Well Documented** - Extensive documentation with real-world examples
- **🌐 Multi-Network** - Support for MainNet, TestNet, and private networks
- **⚡ High Performance** - Async/await design with connection pooling

## 🚀 Quick Start

### Installation

```bash
# Install from crates.io
cargo install neo3

# Or clone and build from source
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust
cargo build --release
```

### Basic Usage

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Neo N3 network
    let provider = HttpProvider::new("https://mainnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Query blockchain
    let block_count = client.get_block_count().await?;
    println!("Current block height: {}", block_count);
    
    // Create wallet
    let wallet = Wallet::new();
    let account = Account::create()?;
    
    // Build transaction
    let tx = TransactionBuilder::new()
        .add_transfer(from, to, asset, amount)
        .build()?;
    
    Ok(())
}
```

## 📦 Project Structure

```
NeoRust/
├── crates/              # Core SDK crates (planned workspace)
│   ├── neo3/           # Main SDK crate
│   ├── neo3-types/     # Core types and primitives
│   ├── neo3-crypto/    # Cryptographic operations
│   ├── neo3-rpc/       # RPC client implementation
│   ├── neo3-builder/   # Transaction & script builders
│   ├── neo3-wallets/   # Wallet management
│   └── neo3-contracts/ # Smart contract tools
├── apps/               # Applications
│   ├── neo-cli/       # Command-line interface
│   └── neo-gui/       # Desktop GUI application
├── examples/          # Comprehensive examples
├── docs/             # Documentation
└── tests/            # Integration tests
```

## 🛠️ Components

### Core SDK (`neo3`)

The foundation library providing:

- **RPC Client** - High-performance JSON-RPC client with retry logic
- **Transaction Builder** - Type-safe transaction construction
- **Cryptography** - Secp256r1 signatures, SHA256, Base58
- **Wallet Management** - NEP-6 standard wallet support
- **Smart Contracts** - Contract deployment and invocation
- **Type System** - Strongly-typed blockchain primitives

### Desktop GUI (`neo-gui`)

Professional desktop application featuring:

- **Wallet Management** - Create, import, and manage wallets
- **Portfolio Dashboard** - Real-time asset tracking
- **Transaction History** - Comprehensive transaction explorer
- **NFT Marketplace** - Browse and manage NFT collections
- **Developer Tools** - Contract deployment and testing
- **Network Monitor** - Real-time blockchain statistics

### Command Line Interface (`neo-cli`)

Powerful CLI for automation:

```bash
# Wallet operations
neo-cli wallet create --name production-wallet
neo-cli wallet import --wif <private-key>

# Network operations
neo-cli network status
neo-cli network connect --endpoint https://mainnet1.neo.org

# Transaction operations
neo-cli transfer --from <address> --to <address> --amount 10 --asset NEO

# Contract operations
neo-cli contract deploy --nef contract.nef --manifest contract.json
neo-cli contract invoke --hash <contract-hash> --method transfer
```

## 📚 Documentation

### Getting Started
- [Installation Guide](docs/getting-started/installation.md)
- [Quick Start Tutorial](docs/getting-started/quick-start.md)
- [API Reference](https://docs.rs/neo3)

### Guides
- [Wallet Management](docs/guides/wallet-guide.md)
- [Smart Contract Interaction](docs/guides/contract-guide.md)
- [Transaction Building](docs/guides/transaction-guide.md)
- [Security Best Practices](docs/guides/security.md)

### Examples
- [Basic Operations](examples/basic/)
- [Advanced Scenarios](examples/advanced/)
- [DeFi Integration](examples/defi/)
- [NFT Operations](examples/nft/)

## 🔧 Development

### Prerequisites

- Rust 1.70+ (2021 edition)
- Node.js 20+ (for GUI development)
- Git

### Building from Source

```bash
# Clone repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust

# Build everything
cargo build --workspace --release

# Run tests
cargo test --workspace

# Build documentation
cargo doc --workspace --no-deps --open
```

### Running the GUI

```bash
cd neo-gui
npm install
npm run tauri dev
```

### Development Workflow

1. **Code Style** - Run `cargo fmt` before committing
2. **Linting** - Ensure `cargo clippy` passes
3. **Testing** - Add tests for new functionality
4. **Documentation** - Update docs for API changes
5. **Security** - Run `cargo audit` regularly

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific component tests
cargo test -p neo3-rpc
cargo test -p neo3-crypto

# Run integration tests
cargo test --test '*' --features integration

# Run with coverage
cargo tarpaulin --workspace --out Html
```

## 🔐 Security

NeoRust follows security best practices:

- **No hardcoded secrets** - All sensitive data encrypted
- **Secure key storage** - Hardware wallet support
- **Input validation** - Comprehensive input sanitization
- **Dependency auditing** - Regular security updates
- **Secure defaults** - TLS verification enabled

For security issues, please email security@r3e.network

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Areas for Contribution

- 🐛 Bug fixes and issue resolution
- ✨ New features and enhancements
- 📚 Documentation improvements
- 🧪 Test coverage expansion
- 🌐 Internationalization

## 📊 Production Status

### ✅ Production Ready
- **Core SDK** - Complete Neo N3 protocol implementation
- **Wallet Operations** - Full NEP-6 wallet management
- **RPC Client** - Robust network communication
- **Transaction Building** - All transaction types supported
- **Cryptography** - Industry-standard implementations

### 🚧 In Development
- **Multi-crate workspace** - Modular architecture refactoring
- **Hardware wallet integration** - Ledger support expansion
- **Advanced DeFi features** - DEX and lending protocols
- **Cross-chain bridges** - Neo X integration

## 🌟 Sponsors & Partners

<div align="center">
  <p>
    <a href="https://neo.org"><img src="assets/images/neo-logo.png" height="50" alt="Neo" /></a>
    &nbsp;&nbsp;&nbsp;&nbsp;
    <a href="https://r3e.network"><img src="assets/images/r3e-logo.png" height="50" alt="R3E Network" /></a>
  </p>
</div>

## 📄 License

This project is dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

## 🔗 Links

- **Documentation**: [https://docs.rs/neo3](https://docs.rs/neo3)
- **Repository**: [https://github.com/R3E-Network/NeoRust](https://github.com/R3E-Network/NeoRust)
- **Issue Tracker**: [GitHub Issues](https://github.com/R3E-Network/NeoRust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/R3E-Network/NeoRust/discussions)
- **Neo Official**: [https://neo.org](https://neo.org)

---

<div align="center">
  <p>Built with ❤️ by the NeoRust Team</p>
  <p>Empowering the Neo N3 ecosystem with professional Rust tooling</p>
</div>