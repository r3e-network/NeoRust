# Basic Examples

These examples cover fundamental Neo N3 operations. Start here if you're new to Neo N3 or NeoRust.

## 📚 Examples

### 01. Network Connection (`01_connect_to_network.rs`)
Learn how to connect to Neo N3 nodes and check network status.

**What you'll learn:**
- Connecting to TestNet and MainNet
- Checking node health and block height
- Handling network errors gracefully
- Using multiple RPC endpoints

**Run:**
```bash
cargo run --example 01_connect_to_network
```

### 02. Wallet Creation (`02_create_wallet.rs`)
Create and manage Neo N3 wallets securely.

**What you'll learn:**
- Creating new wallets with secure random keys
- Generating addresses from public keys
- Understanding NEP-6 wallet format
- Best practices for key storage

**Run:**
```bash
cargo run --example 02_create_wallet
```

### 03. Balance Queries (`03_check_balance.rs`)
Query account balances and asset information.

**What you'll learn:**
- Checking NEO and GAS balances
- Querying NEP-17 token balances
- Understanding asset decimals and formatting
- Handling non-existent accounts

**Run:**
```bash
cargo run --example 03_check_balance
```

### 04. Simple Transfers (`04_send_transaction.rs`)
Send NEO and GAS between addresses.

**What you'll learn:**
- Building basic transfer transactions
- Calculating network fees
- Signing transactions securely
- Broadcasting to the network

**Run:**
```bash
cargo run --example 04_send_transaction
```

## 🎯 Learning Path

1. **Start with 01_connect_to_network** - Understand network basics
2. **Move to 02_create_wallet** - Learn wallet fundamentals
3. **Try 03_check_balance** - Query blockchain data
4. **Complete with 04_send_transaction** - Make your first transaction

## 🔧 Prerequisites

- Rust 1.70+
- Internet connection
- Basic understanding of blockchain concepts

## 🌐 Network Settings

All basic examples use **TestNet** by default for safety:
- No real assets at risk
- Free test tokens available
- Same functionality as MainNet

## 💡 Tips

- Always test on TestNet first
- Never commit private keys to code
- Use proper error handling
- Validate all inputs

## 🚀 Next Steps

After completing basic examples, move to:
- [Intermediate Examples](../intermediate/) - Contract interaction and advanced operations
- [Advanced Examples](../advanced/) - Production patterns and DeFi integration