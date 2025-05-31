# Command Line Interface - Powerful Neo N3 Tools

Welcome to the **NeoRust CLI** - a professional, feature-rich command-line interface for Neo N3 blockchain operations with beautiful output and comprehensive functionality.

![NeoRust CLI](../static/img/cli-hero.png)

## 🌟 Why Choose NeoRust CLI

The NeoRust CLI is designed for developers, power users, and automation scenarios. It provides the full power of the Neo N3 blockchain through an elegant command-line interface.

### 🎨 **Beautiful Output**
- **Colored Interface**: Rich, colored output for better readability
- **Progress Indicators**: Real-time progress bars and spinners
- **Interactive Prompts**: User-friendly input prompts and confirmations
- **Formatted Tables**: Beautiful table output for data display
- **Status Icons**: Visual indicators for success, error, and warning states

### 🔧 **Comprehensive Features**
- **Wallet Management**: Complete wallet operations
- **NFT Operations**: Full NFT lifecycle management
- **Network Tools**: Blockchain monitoring and interaction
- **Developer Utilities**: Encoding, hashing, and debugging tools
- **Automation Ready**: Perfect for scripts and CI/CD pipelines

### ⚡ **High Performance**
- **Fast Execution**: Optimized Rust implementation
- **Parallel Operations**: Concurrent processing where possible
- **Memory Efficient**: Minimal resource usage
- **Reliable**: Comprehensive error handling and recovery

## 🚀 Quick Start

### Installation

#### Option 1: Download Pre-built Binary
```bash
# Download the latest release for your platform
curl -L https://github.com/R3E-Network/NeoRust/releases/latest/download/neo-cli-linux -o neo-cli
chmod +x neo-cli

# Or for other platforms:
# Windows: neo-cli-windows.exe
# macOS: neo-cli-macos
```

#### Option 2: Build from Source
```bash
# Clone the repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust/neo-cli

# Build the CLI
cargo build --release

# The binary will be at ./target/release/neo-cli
```

#### Option 3: Install via Cargo
```bash
# Install directly from crates.io
cargo install neo-cli

# Or from git
cargo install --git https://github.com/R3E-Network/NeoRust.git neo-cli
```

### First Steps

```bash
# Check installation
neo-cli --version

# Get help
neo-cli --help

# Create your first wallet
neo-cli wallet create --name "MyWallet"

# Check network status
neo-cli network status
```

## 📋 Command Overview

### 🏠 **Main Commands**

| Command | Description | Example |
|---------|-------------|---------|
| `wallet` | Wallet management operations | `neo-cli wallet create --name "MyWallet"` |
| `nft` | NFT operations and management | `neo-cli nft mint --contract "0x..." --to "NX8..."` |
| `network` | Network connectivity and info | `neo-cli network connect --network testnet` |
| `tools` | Developer utilities | `neo-cli tools encode --input "hello" --format base64` |

### 💼 **Wallet Commands**

```bash
# Wallet Management
neo-cli wallet create --name "MyWallet" --path "./wallet.json"
neo-cli wallet open --path "./wallet.json"
neo-cli wallet close
neo-cli wallet list

# Address Management
neo-cli wallet create-address --label "Main Account"
neo-cli wallet import-address --wif "KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr"
neo-cli wallet export-address --address "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc"
neo-cli wallet list-addresses

# Balance and Transactions
neo-cli wallet balance --detailed
neo-cli wallet history --limit 10
neo-cli wallet send --to "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc" --amount 1.5 --asset NEO

# Backup and Recovery
neo-cli wallet backup --output "./backup.json"
neo-cli wallet restore --input "./backup.json"
```

### 🎨 **NFT Commands**

```bash
# NFT Collection Management
neo-cli nft deploy --name "MyCollection" --symbol "MC" --max-supply 1000
neo-cli nft info --contract "0x1234567890abcdef1234567890abcdef12345678"
neo-cli nft collection --contract "0x..." --owner "NX8..."

# NFT Operations
neo-cli nft mint --contract "0x..." --to "NX8..." --token-id "001" --metadata "ipfs://..."
neo-cli nft transfer --contract "0x..." --token-id "001" --from "NX8..." --to "NY9..."
neo-cli nft burn --contract "0x..." --token-id "001"

# NFT Information
neo-cli nft balance --contract "0x..." --owner "NX8..."
neo-cli nft metadata --contract "0x..." --token-id "001"
neo-cli nft properties --contract "0x..." --token-id "001"
```

### 🌐 **Network Commands**

```bash
# Network Management
neo-cli network connect --network "Neo N3 Testnet"
neo-cli network disconnect
neo-cli network status
neo-cli network list

# Network Information
neo-cli network info --detailed
neo-cli network peers
neo-cli network block --height 1000000
neo-cli network transaction --hash "0x..."

# Network Configuration
neo-cli network add --name "Custom" --rpc "https://custom-node.com:443"
neo-cli network remove --name "Custom"
neo-cli network test --network "testnet"
```

### 🔧 **Developer Tools**

```bash
# Encoding/Decoding
neo-cli tools encode --input "hello world" --format base64
neo-cli tools decode --input "aGVsbG8gd29ybGQ=" --format base64
neo-cli tools encode --input "neo" --format hex

# Hashing
neo-cli tools hash --input "hello" --algorithm sha256
neo-cli tools hash --input "neo" --algorithm ripemd160
neo-cli tools hash --file "./contract.nef" --algorithm sha256

# Random Generation
neo-cli tools random --length 32 --format hex
neo-cli tools random --length 16 --format base64

# Address Utilities
neo-cli tools validate-address --address "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc"
neo-cli tools generate-address --count 5

# Transaction Tools
neo-cli tools calculate-fee --transaction "./tx.json"
neo-cli tools sign-transaction --transaction "./tx.json" --key "KxDgvE..."
```

## 🎨 Beautiful Output Examples

### **Wallet Balance Display**
```
┌─────────────────────────────────────────────────────────────┐
│                    💼 Wallet Balance                        │
├─────────────────────────────────────────────────────────────┤
│ Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc                │
│                                                             │
│ Assets:                                                     │
│ ├─ 🟢 NEO      │ 100.00000000 │ $2,500.00                  │
│ ├─ ⛽ GAS      │  50.12345678 │ $1,250.62                  │
│ └─ 🎨 FLUND    │   1.00000000 │   $25.00                   │
│                                                             │
│ Total Value: $3,775.62                                      │
└─────────────────────────────────────────────────────────────┘
```

### **Network Status Display**
```
🌐 Network Status: Neo N3 MainNet

┌─────────────────┬─────────────────────────────────────────┐
│ Property        │ Value                                   │
├─────────────────┼─────────────────────────────────────────┤
│ 📊 Block Height │ 12,345,678                             │
│ ⏱️  Block Time   │ 15.2s                                   │
│ 🔗 Connections  │ 42 peers                                │
│ 💾 Memory Pool  │ 156 transactions                        │
│ ⛽ Network Fee  │ 0.00123456 GAS                          │
│ 🔄 Status       │ ✅ Synchronized                         │
└─────────────────┴─────────────────────────────────────────┘
```

### **NFT Collection Display**
```
🎨 NFT Collection: CryptoPunks Neo

┌─────────────────────────────────────────────────────────────┐
│ Contract: 0x1234567890abcdef1234567890abcdef12345678        │
│ Symbol: CPN                                                 │
│ Total Supply: 10,000                                        │
│ Owner: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc                  │
├─────────────────────────────────────────────────────────────┤
│ Recent Mints:                                               │
│ ├─ #9999 → NX8... (2 minutes ago)                          │
│ ├─ #9998 → NY9... (5 minutes ago)                          │
│ └─ #9997 → NZ1... (8 minutes ago)                          │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Advanced Usage

### **Configuration File**
Create a configuration file at `~/.neorust/config.toml`:

```toml
[default]
network = "Neo N3 TestNet"
wallet_path = "~/.neorust/wallets/"
auto_backup = true

[networks.mainnet]
name = "Neo N3 MainNet"
rpc_url = "https://mainnet1.neo.coz.io:443"
magic = 860833102

[networks.testnet]
name = "Neo N3 TestNet"
rpc_url = "https://testnet1.neo.coz.io:443"
magic = 894710606

[ui]
color = true
progress_bars = true
table_style = "rounded"
```

### **Environment Variables**
```bash
# Set default network
export NEO_NETWORK="testnet"

# Set default wallet path
export NEO_WALLET_PATH="./my-wallet.json"

# Enable verbose logging
export NEO_LOG_LEVEL="debug"

# Disable colored output
export NO_COLOR=1
```

### **Scripting and Automation**

#### **Batch Operations**
```bash
#!/bin/bash
# Batch wallet creation script

for i in {1..10}; do
    neo-cli wallet create --name "Wallet_$i" --path "./wallets/wallet_$i.json"
    neo-cli wallet create-address --label "Account_1"
    echo "Created wallet $i"
done
```

#### **CI/CD Integration**
```yaml
# GitHub Actions example
name: Deploy Smart Contract
on: [push]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install NeoRust CLI
        run: |
          curl -L https://github.com/R3E-Network/NeoRust/releases/latest/download/neo-cli-linux -o neo-cli
          chmod +x neo-cli
      - name: Deploy Contract
        run: |
          ./neo-cli network connect --network testnet
          ./neo-cli contract deploy --file ./contract.nef --manifest ./contract.manifest.json
```

#### **Monitoring Script**
```bash
#!/bin/bash
# Network monitoring script

while true; do
    echo "$(date): Checking network status..."
    
    if neo-cli network status --quiet; then
        echo "✅ Network is healthy"
    else
        echo "❌ Network issues detected"
        # Send alert notification
    fi
    
    sleep 60
done
```

## 🎯 Use Cases

### **For Developers**
- **Smart Contract Testing**: Deploy and test contracts on TestNet
- **Transaction Analysis**: Analyze blockchain transactions
- **Automation Scripts**: Automate repetitive blockchain operations
- **CI/CD Integration**: Integrate blockchain operations into deployment pipelines

### **For Power Users**
- **Bulk Operations**: Perform batch wallet or NFT operations
- **Portfolio Management**: Monitor and manage large portfolios
- **Network Monitoring**: Track blockchain health and performance
- **Advanced Trading**: Execute complex trading strategies

### **For Enterprises**
- **Treasury Management**: Manage corporate digital assets
- **Compliance Reporting**: Generate detailed transaction reports
- **Multi-Signature Operations**: Coordinate multi-sig transactions
- **Infrastructure Monitoring**: Monitor blockchain infrastructure

## 📊 Performance and Reliability

### **Benchmarks**
- **Wallet Creation**: < 100ms
- **Transaction Signing**: < 50ms
- **Network Queries**: < 200ms
- **Bulk Operations**: 1000+ ops/minute

### **Error Handling**
- **Graceful Degradation**: Continues operation despite network issues
- **Retry Logic**: Automatic retry for transient failures
- **Detailed Errors**: Clear error messages with suggested solutions
- **Recovery Options**: Multiple recovery strategies for failed operations

### **Logging and Debugging**
```bash
# Enable debug logging
neo-cli --log-level debug wallet balance

# Save logs to file
neo-cli --log-file ./neo-cli.log network status

# Verbose output
neo-cli -v wallet create --name "TestWallet"
```

## 🔒 Security Features

### **Secure Key Management**
- **Memory Protection**: Private keys cleared from memory after use
- **Encrypted Storage**: All sensitive data encrypted at rest
- **Hardware Wallet Support**: Integration with Ledger devices
- **Secure Random**: Cryptographically secure random number generation

### **Network Security**
- **TLS Verification**: All network connections verified
- **Certificate Pinning**: Protection against man-in-the-middle attacks
- **Request Validation**: All RPC requests validated
- **Rate Limiting**: Protection against abuse

## 📚 Next Steps

- **[Command Reference](./commands)**: Complete command documentation
- **[Configuration Guide](./configuration)**: Advanced configuration options
- **[Scripting Guide](./scripting)**: Automation and scripting examples
- **[Troubleshooting](./troubleshooting)**: Common issues and solutions
- **[API Integration](./api-integration)**: Using CLI in applications

---

**Ready to harness the power of Neo N3 from the command line?**

[Download NeoRust CLI →](https://github.com/R3E-Network/NeoRust/releases) 