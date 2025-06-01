# NeoRust v0.4.1 - Production-Ready Neo N3 SDK

<div align="center">
  <p>
    <img src="assets/images/neo-logo.png" alt="Neo Logo" width="125" align="middle"/>&nbsp;&nbsp;&nbsp;&nbsp;
    <img src="assets/images/neo-x-logo.png" alt="Neo X Logo" width="80" align="middle"/>&nbsp;&nbsp;&nbsp;&nbsp;
    <img src="assets/images/r3e-logo.png" alt="R3E Logo" width="300" align="middle"/>
  </p>
  
  <h3>🚀 Complete Neo N3 Development Suite</h3>
  <p><strong>Rust SDK • Beautiful GUI • Powerful CLI • Production Ready</strong></p>
</div>

[![Rust](https://github.com/R3E-Network/NeoRust/actions/workflows/rust.yml/badge.svg)](https://github.com/R3E-Network/NeoRust/actions/workflows/rust.yml)
[![Release](https://github.com/R3E-Network/NeoRust/actions/workflows/release.yml/badge.svg)](https://github.com/R3E-Network/NeoRust/actions/workflows/release.yml)
[![Netlify Status](https://api.netlify.com/api/v1/badges/bb13b046-abdd-4d5e-962c-31c2a3d06090/deploy-status)](https://app.netlify.com/sites/neorust/deploys)
[![Crates.io](https://img.shields.io/crates/v/neo3.svg)](https://crates.io/crates/neo3)
[![Documentation](https://docs.rs/neo3/badge.svg)](https://docs.rs/neo3)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**NeoRust** is the most comprehensive Rust SDK for the Neo N3 blockchain ecosystem. It provides everything you need to build, deploy, and manage Neo applications - from a powerful Rust library to beautiful desktop applications.

## 🚀 Quick Start

### 📦 **Installation**

#### Download Pre-built Binaries
Visit our [Releases Page](https://github.com/R3E-Network/NeoRust/releases/latest) to download pre-built binaries for your platform:

- **Linux**: `neo-cli-linux-x86_64`, `neo-gui-linux-x86_64`
- **macOS**: `neo-cli-macos-x86_64`, `neo-gui-macos-x86_64` (Intel), `neo-cli-macos-aarch64`, `neo-gui-macos-aarch64` (Apple Silicon)
- **Windows**: `neo-cli-windows-x86_64.exe`, `neo-gui-windows-x86_64.exe`

#### Install from Crates.io
```bash
# Install the CLI globally
cargo install neo-cli

# Add the SDK to your project
cargo add neo3
```

### 📱 Desktop GUI Application

**Download and run the beautiful Neo N3 Wallet:**

```bash
# Clone the repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust/neo-gui

# Install dependencies and start
npm install
npm run dev
```

**Access at**: http://localhost:1420

**Features:**
- 💼 Multi-wallet management with secure storage
- 📊 Real-time portfolio dashboard with interactive charts
- 🎨 NFT collection browser and minting interface
- 🔧 Developer tools for encoding, hashing, and debugging
- 🌐 Network management and blockchain monitoring
- ⚡ Lightning-fast performance with hot reload

### 💻 Command Line Interface

**Install and use the powerful Neo CLI:**

```bash
# Build the CLI application
cd neo-cli
cargo build --release

# Create your first wallet
./target/release/neo-cli wallet create --name "MyWallet"

# Check network status
./target/release/neo-cli network status

# Mint an NFT
./target/release/neo-cli nft mint --contract "0x..." --to "NX8..." --token-id "001"

# Explore all commands
./target/release/neo-cli --help
```

### 📚 Rust SDK Library

**Add to your Cargo.toml:**

```toml
[dependencies]
neo3 = "0.4.1"
```

**Quick example:**

```rust
use neo3::prelude::*;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
   // Connect to Neo N3 TestNet
   let provider = neo_providers::JsonRpcClient::new("https://testnet1.neo.coz.io:443");
   
   // Get blockchain information
   let block_count = provider.get_block_count().await?;
   println!("Current block height: {}", block_count);
   
   // Create a new wallet
   let mut wallet = Wallet::new();
   let account = Account::create()?;
   wallet.add_account(account);
   
   Ok(())
}
```

## 🎯 Core Features

### 🏗️ **Complete Neo N3 Support**
- **Blockchain Integration**: Full Neo N3 protocol compatibility
- **Smart Contracts**: Deploy, invoke, and manage contracts
- **Transaction Building**: Construct and sign all transaction types
- **RPC Client**: High-performance JSON-RPC communication

### 💼 **Wallet & Account Management**
- **NEP-6 Standard**: Industry-standard wallet format
- **Multi-Account**: Manage multiple accounts per wallet
- **Hardware Wallets**: Ledger device support
- **Secure Storage**: Encrypted private key management

### 🎨 **NFT & Token Operations**
- **NEP-17 Tokens**: Full token standard support
- **NFT Management**: Mint, transfer, and manage NFTs
- **Collection Tools**: Create and manage NFT collections
- **Metadata Handling**: IPFS and on-chain metadata support

### 🌐 **Network & Infrastructure**
- **Multi-Network**: MainNet, TestNet, and private networks
- **Node Management**: Connect to multiple Neo nodes
- **Network Monitoring**: Real-time blockchain statistics
- **Health Checks**: Automatic node health monitoring

### 🔧 **Developer Tools**
- **Contract Debugging**: Advanced debugging capabilities
- **Transaction Analysis**: Detailed transaction inspection
- **Gas Optimization**: Fee calculation and optimization
- **Testing Framework**: Comprehensive testing utilities

## 📸 Screenshots

### Desktop GUI Application
![Neo N3 Wallet Dashboard](assets/screenshots/dashboard.png)
*Beautiful dashboard with portfolio overview and real-time charts*

![Wallet Management](assets/screenshots/wallet.png)
*Comprehensive wallet management with multi-account support*

![NFT Marketplace](assets/screenshots/nft.png)
*Elegant NFT collection browser with minting capabilities*

### Command Line Interface
![CLI Commands](assets/screenshots/cli.png)
*Powerful CLI with beautiful colored output and progress indicators*

## 🏗️ Architecture

### 📦 **Modular Design**
```
NeoRust/
├── 📚 neo3/              # Core Rust SDK library
├── 🖥️  neo-gui/          # Desktop GUI application (Tauri + React)
├── 💻 neo-cli/           # Command line interface
├── 📖 docs/              # Comprehensive documentation
├── 🌐 website/           # Project website
└── 🧪 examples/          # Usage examples and tutorials
```

### 🔧 **Technology Stack**
- **Backend**: Rust with async/await support
- **GUI**: Tauri + React + TypeScript + Tailwind CSS
- **CLI**: Clap + Colored output + Interactive prompts
- **Crypto**: Industry-standard cryptographic libraries
- **Network**: High-performance HTTP/WebSocket clients

## 📖 Documentation

### 📚 **Comprehensive Guides**
- **[API Reference](https://docs.rs/neo3)**: Complete API documentation
- **[User Guide](https://neorust.netlify.app/guide)**: Step-by-step tutorials
- **[Developer Docs](https://neorust.netlify.app/dev)**: Advanced development guides
- **[Examples](https://neorust.netlify.app/examples)**: Real-world usage examples
- **[Release Workflow](docs/RELEASE_WORKFLOW.md)**: Automated release process guide

### 🌐 **Online Resources**
- **Website**: [https://neorust.netlify.app/](https://neorust.netlify.app/)
- **Documentation**: [https://r3e-network.github.io/NeoRust/](https://r3e-network.github.io/NeoRust/)
- **Crate Page**: [https://crates.io/crates/neo3](https://crates.io/crates/neo3)
- **GitHub**: [https://github.com/R3E-Network/NeoRust](https://github.com/R3E-Network/NeoRust)

## 🎯 Use Cases

### 🏢 **Enterprise Applications**
- **DeFi Platforms**: Build decentralized finance applications
- **Asset Management**: Tokenize and manage digital assets
- **Supply Chain**: Track products on the blockchain
- **Identity Solutions**: Decentralized identity management

### 👨‍💻 **Developer Tools**
- **dApp Development**: Build decentralized applications
- **Smart Contract Testing**: Comprehensive testing frameworks
- **Blockchain Analytics**: Monitor and analyze blockchain data
- **Integration Services**: Connect existing systems to Neo

### 🎮 **Gaming & NFTs**
- **Game Asset Management**: In-game item tokenization
- **NFT Marketplaces**: Create and trade digital collectibles
- **Metaverse Integration**: Virtual world asset management
- **Creator Tools**: Content creator monetization platforms

## 🚀 Getting Started

### 1. **Choose Your Interface**

#### 🖥️ Desktop GUI (Recommended for Users)
```bash
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust/neo-gui
npm install && npm run dev
```

#### 💻 Command Line (Recommended for Developers)
```bash
cd NeoRust/neo-cli
cargo build --release
./target/release/neo-cli --help
```

#### 📚 Rust Library (Recommended for Integration)
```toml
[dependencies]
neo3 = "0.4.1"
```

### 2. **Create Your First Wallet**

#### GUI Method:
1. Open the Neo N3 Wallet application
2. Click "Create Wallet" 
3. Follow the secure setup wizard
4. Start managing your Neo assets

#### CLI Method:
```bash
neo-cli wallet create --name "MyFirstWallet"
neo-cli wallet create-address --label "Main Account"
neo-cli wallet balance --detailed
```

#### SDK Method:
```rust
use neo3::prelude::*;

let mut wallet = Wallet::new();
wallet.set_name("MyFirstWallet".to_string());
let account = Account::create()?;
wallet.add_account(account);
```

### 3. **Connect to Neo Network**

#### GUI: 
- Network selector in the top navigation
- Real-time connection status
- Automatic health monitoring

#### CLI:
```bash
neo-cli network connect --network "Neo N3 Testnet"
neo-cli network status
```

#### SDK:
```rust
let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
let client = RpcClient::new(provider);
let block_count = client.get_block_count().await?;
```

## 🔧 Advanced Usage

### 🛠️ **Troubleshooting**

#### Common Build Issues
If you encounter build errors, especially related to `yubihsm` or `MockHsm`, see our [Build Configuration Guide](docs/guides/build-configuration.md) for solutions.

**Quick fix for MockHsm release build error:**
```bash
# Use the mock-hsm feature only for development
cargo build --features "mock-hsm"  # Development
cargo build --release              # Production (no mock features)
```

#### Getting Help
- **Documentation**: [Build Configuration Guide](docs/guides/build-configuration.md)
- **Issues**: [GitHub Issues](https://github.com/R3E-Network/NeoRust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/R3E-Network/NeoRust/discussions)

### 🚀 **Production Deployment**