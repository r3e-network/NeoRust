# Quick Start

Get up and running with NeoRust SDK in just a few minutes. This guide will walk you through creating your first Neo N3 application.

## Prerequisites

- Rust 1.70+ installed
- NeoRust SDK added to your project

If you haven't installed NeoRust yet, see the [Installation Guide](./installation.md).

## 1. Your First Connection

Let's start by connecting to the Neo N3 network and getting some basic information:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Neo N3 TestNet
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Get network information
    let version = client.get_version().await?;
    let block_count = client.get_block_count().await?;
    
    println!("Connected to: {}", version.useragent);
    println!("Network: {}", version.network);
    println!("Current block height: {}", block_count);
    
    Ok(())
}
```

Run this with `cargo run` and you should see output like:
```
Connected to: Neo:3.6.0
Network: 894710606
Current block height: 2845234
```

## 2. Create Your First Account

Neo accounts are the foundation of the ecosystem. Let's create one:

```rust
use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new account
    let account = Account::create()?;
    
    // Display account information
    println!("🎉 New account created!");
    println!("Address: {}", account.get_address());
    println!("Script Hash: {}", account.get_script_hash());
    
    // Get the public key
    let public_key = account.get_public_key();
    println!("Public Key: {}", hex::encode(public_key.encode_point(true)));
    
    Ok(())
}
```

:::tip
**Keep your private keys safe!** In production, never log or expose private keys. Store them securely using proper key management practices.
:::

## 3. Check Account Balance

Let's check the balance of any Neo address:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Use any Neo address (this one has testnet tokens)
    let address = "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv";
    let script_hash = address.to_script_hash()?;
    
    // Check GAS balance
    let gas_token = GasToken::new(&client);
    let gas_balance = gas_token.balance_of(&script_hash).await?;
    let gas_decimals = gas_token.decimals().await?;
    
    // Check NEO balance
    let neo_token = NeoToken::new(&client);
    let neo_balance = neo_token.balance_of(&script_hash).await?;
    
    println!("Address: {}", address);
    println!("GAS Balance: {} (decimals: {})", gas_balance, gas_decimals);
    println!("NEO Balance: {}", neo_balance);
    
    Ok(())
}
```

## 4. Working with Wallets

For managing multiple accounts, use wallets:

```rust
use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new wallet
    let mut wallet = Wallet::new();
    wallet.set_name("MyFirstWallet".to_string());
    
    // Add multiple accounts
    for i in 1..=3 {
        let account = Account::create()?;
        wallet.add_account(account);
        println!("Added account {}: {}", i, wallet.get_accounts().last().unwrap().get_address());
    }
    
    // Set the first account as default
    if let Some(first_account) = wallet.get_accounts().first() {
        wallet.set_default_account(first_account.get_script_hash());
    }
    
    println!("\n💼 Wallet '{}' created with {} accounts", 
             wallet.get_name(), 
             wallet.get_accounts().len());
    
    if let Some(default_account) = wallet.get_default_account() {
        println!("Default account: {}", default_account.get_address());
    }
    
    Ok(())
}
```

## 5. Invoke a Smart Contract

Let's call a simple smart contract method:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Get GAS token info (GAS is a native contract)
    let gas_token = GasToken::new(&client);
    
    // Call read-only methods
    let symbol = gas_token.symbol().await?;
    let decimals = gas_token.decimals().await?;
    let total_supply = gas_token.total_supply().await?;
    
    println!("Token Information:");
    println!("  Symbol: {}", symbol);
    println!("  Decimals: {}", decimals);
    println!("  Total Supply: {}", total_supply);
    
    Ok(())
}
```

## 6. Complete Example: Transfer Tokens

Here's a complete example that demonstrates transferring GAS tokens:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Load an account with some GAS (you'll need to fund this on testnet)
    // For demo purposes - in practice, load from secure storage
    let sender = Account::from_private_key("your_private_key_here")?;
    let signer = AccountSigner::new(sender);
    
    // Recipient address
    let recipient = "NRecipientAddressHere".to_script_hash()?;
    
    // Transfer 1 GAS (1 GAS = 100,000,000 smallest units)
    let gas_token = GasToken::new(&client);
    let amount = 100_000_000; // 1 GAS
    
    println!("Sending {} GAS to {}...", amount as f64 / 100_000_000.0, recipient);
    
    // Execute the transfer
    let tx_hash = gas_token
        .transfer(&signer, &recipient, amount, None)
        .await?;
    
    println!("✅ Transfer successful!");
    println!("Transaction Hash: {}", tx_hash);
    println!("View on explorer: https://testnet.neotube.io/transaction/{}", tx_hash);
    
    Ok(())
}
```

:::warning
**Testnet Tokens Required**
To run the transfer example, you'll need testnet GAS tokens. Get them from the [Neo Testnet Faucet](https://neowish.ngd.network/).
:::

## Common Patterns

### Error Handling

Always handle errors properly in production code:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() {
    let result = connect_and_query().await;
    
    match result {
        Ok(block_count) => println!("Current block: {}", block_count),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn connect_and_query() -> Result<u32, Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    let block_count = client.get_block_count().await?;
    Ok(block_count)
}
```

### Reusing Connections

For better performance, reuse client connections:

```rust
use neo3::prelude::*;

pub struct NeoService {
    client: RpcClient<HttpProvider>,
}

impl NeoService {
    pub fn new(endpoint: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = HttpProvider::new(endpoint)?;
        let client = RpcClient::new(provider);
        Ok(Self { client })
    }
    
    pub async fn get_block_count(&self) -> Result<u32, Box<dyn std::error::Error>> {
        Ok(self.client.get_block_count().await?)
    }
    
    pub async fn get_gas_balance(&self, address: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let script_hash = address.to_script_hash()?;
        let gas_token = GasToken::new(&self.client);
        Ok(gas_token.balance_of(&script_hash).await?)
    }
}
```

## Next Steps

🎉 **Congratulations!** You've successfully created your first Neo N3 applications with NeoRust.

### What's Next?

1. **📚 [Explore Documentation](../intro.md)** - Learn about advanced features
2. **🔍 [View Examples](/examples)** - See more practical code examples  
3. **🛠️ [Try the CLI Tool](/cli)** - Use command-line tools for development
4. **🖥️ [Download GUI App](/gui)** - Try the desktop wallet application
5. **🤝 [Join Community](https://github.com/R3E-Network/NeoRust/discussions)** - Get help and share projects

### Useful Resources

- **[Neo N3 Docs](https://docs.neo.org/)** - Official Neo documentation
- **[Testnet Faucet](https://neowish.ngd.network/)** - Get testnet tokens
- **[Block Explorer](https://testnet.neotube.io/)** - View transactions and blocks
- **[API Reference](https://docs.rs/neo3)** - Complete API documentation

Happy building! 🚀 