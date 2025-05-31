# Quick Start Guide

<div className="hero hero--primary">
  <div className="container">
    <h1 className="hero__title">⚡ Quick Start</h1>
    <p className="hero__subtitle">
      Get up and running with NeoRust in 5 minutes
    </p>
    <p>
      Choose your preferred interface and start building on Neo N3 immediately.
    </p>
  </div>
</div>

## 🎯 Choose Your Path

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🖥️ Desktop GUI</h3>
      </div>
      <div className="card__body">
        <p><strong>Best for:</strong> Visual users, wallet management, NFT trading</p>
        <p><strong>Time:</strong> 2 minutes</p>
      </div>
      <div className="card__footer">
        <a href="#desktop-gui-quick-start" className="button button--primary">Start with GUI →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>💻 Command Line</h3>
      </div>
      <div className="card__body">
        <p><strong>Best for:</strong> Developers, automation, scripting</p>
        <p><strong>Time:</strong> 3 minutes</p>
      </div>
      <div className="card__footer">
        <a href="#cli-quick-start" className="button button--primary">Start with CLI →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>📚 Rust SDK</h3>
      </div>
      <div className="card__body">
        <p><strong>Best for:</strong> Application integration, custom solutions</p>
        <p><strong>Time:</strong> 5 minutes</p>
      </div>
      <div className="card__footer">
        <a href="#sdk-quick-start" className="button button--primary">Start with SDK →</a>
      </div>
    </div>
  </div>
</div>

---

## 🖥️ Desktop GUI Quick Start

### **Step 1: Download and Install**

```bash
# Clone the repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust/neo-gui

# Install dependencies
npm install

# Start the application
npm run dev
```

The GUI will open at `http://localhost:1420`

### **Step 2: Create Your First Wallet**

1. **Welcome Screen**: Click "Create New Wallet"
2. **Wallet Setup**: 
   - Enter wallet name: `MyFirstWallet`
   - Set a strong password
   - Click "Create Wallet"
3. **Recovery Phrase**: 
   - **⚠️ IMPORTANT**: Write down your 12-word recovery phrase
   - Store it securely offline
   - Click "I've Saved My Recovery Phrase"

### **Step 3: Connect to TestNet**

1. **Network Selection**: Click the network dropdown (top right)
2. **Select TestNet**: Choose "Neo N3 TestNet"
3. **Wait for Sync**: The app will connect and synchronize

### **Step 4: Get Test Tokens**

1. **Copy Address**: Click your address to copy it
2. **Visit Faucet**: Go to [Neo TestNet Faucet](https://neowish.ngd.network/)
3. **Request Tokens**: Paste your address and request test NEO/GAS

### **Step 5: Explore Features**

<div className="row">
  <div className="col col--6">
    <h4>🏠 Dashboard</h4>
    <ul>
      <li>View your portfolio balance</li>
      <li>See recent transactions</li>
      <li>Monitor network status</li>
    </ul>
  </div>
  <div className="col col--6">
    <h4>🎨 NFT Section</h4>
    <ul>
      <li>Browse NFT collections</li>
      <li>Mint your first NFT</li>
      <li>Transfer NFTs</li>
    </ul>
  </div>
</div>

**🎉 Congratulations!** You're now ready to use the NeoRust Desktop GUI.

---

## 💻 CLI Quick Start

### **Step 1: Install the CLI**

```bash
# Option 1: Download binary
curl -L https://github.com/R3E-Network/NeoRust/releases/latest/download/neo-cli-linux -o neo-cli
chmod +x neo-cli
sudo mv neo-cli /usr/local/bin/

# Option 2: Install via Cargo
cargo install neo-cli

# Verify installation
neo-cli --version
```

### **Step 2: Create Your First Wallet**

```bash
# Create a new wallet
neo-cli wallet create --name "MyFirstWallet"

# You'll see output like:
# ✅ Wallet created successfully!
# 📁 Location: ~/.neorust/wallets/MyFirstWallet.json
# 🔑 Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
# ⚠️  Please backup your wallet file securely!
```

### **Step 3: Connect to TestNet**

```bash
# Connect to TestNet
neo-cli network connect --network testnet

# Check network status
neo-cli network status

# You'll see:
# 🌐 Network: Neo N3 TestNet
# 📊 Block Height: 2,345,678
# ✅ Status: Connected
```

### **Step 4: Check Your Balance**

```bash
# View wallet balance
neo-cli wallet balance

# Output:
# 💼 Wallet Balance
# ├─ 🟢 NEO: 0.00000000
# ├─ ⛽ GAS: 0.00000000
# └─ Total Value: $0.00
```

### **Step 5: Get Test Tokens**

```bash
# Get your address
neo-cli wallet list-addresses

# Visit the faucet with your address
# Then check balance again
neo-cli wallet balance
```

### **Step 6: Send Your First Transaction**

```bash
# Send NEO to another address
neo-cli wallet send \
  --to "NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c" \
  --amount 1.0 \
  --asset NEO

# You'll see:
# 🚀 Transaction sent successfully!
# 📝 TX Hash: 0x1234567890abcdef...
# ⏱️  Estimated confirmation: 15 seconds
```

**🎉 Congratulations!** You're now ready to use the NeoRust CLI.

---

## 📚 SDK Quick Start

### **Step 1: Create a New Rust Project**

```bash
# Create new project
cargo new my-neo-app
cd my-neo-app

# Add NeoRust dependency
cargo add neo3 --features futures
```

### **Step 2: Basic Connection Example**

Edit `src/main.rs`:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Connecting to Neo N3 TestNet...");
    
    // Connect to TestNet
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    // Get blockchain info
    let version = client.get_version().await?;
    let block_count = client.get_block_count().await?;
    
    println!("✅ Connected to: {}", version.useragent);
    println!("📊 Current block height: {}", block_count);
    
    Ok(())
}
```

### **Step 3: Run Your First App**

```bash
# Run the application
cargo run

# You'll see:
# 🚀 Connecting to Neo N3 TestNet...
# ✅ Connected to: NEO-GO/0.100.1
# 📊 Current block height: 2,345,678
```

### **Step 4: Create a Wallet**

Add this to your `main.rs`:

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Previous connection code...
    
    // Create a new wallet
    let mut wallet = Wallet::new();
    wallet.set_name("MySDKWallet".to_string());
    
    // Create an account
    let account = Account::create()?;
    let address = account.get_address();
    wallet.add_account(account);
    
    println!("💼 Created wallet with address: {}", address);
    
    // Save wallet to file
    wallet.save_to_file("./my-wallet.json")?;
    println!("💾 Wallet saved to: ./my-wallet.json");
    
    Ok(())
}
```

### **Step 5: Query Blockchain Data**

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    // Get latest block
    let block_count = client.get_block_count().await?;
    let latest_block = client.get_block_by_index(block_count - 1, 1).await?;
    
    println!("🔗 Latest block: {}", latest_block.hash);
    println!("📝 Transactions: {}", latest_block.tx.len());
    
    // Get NEO token info
    let neo_hash = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5".parse()?;
    let neo_token = Nep17Contract::new(neo_hash, client.clone());
    
    let symbol = neo_token.symbol().await?;
    let total_supply = neo_token.total_supply().await?;
    
    println!("🪙 Token: {} (Total Supply: {})", symbol, total_supply);
    
    Ok(())
}
```

### **Step 6: Advanced Features**

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    // Load wallet
    let wallet = Wallet::from_file("./my-wallet.json")?;
    let account = wallet.get_default_account()?;
    
    // Check balance
    let neo_hash = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5".parse()?;
    let neo_token = Nep17Contract::new(neo_hash, client);
    
    let balance = neo_token.balance_of(account.get_script_hash()).await?;
    println!("💰 NEO Balance: {}", balance);
    
    // Send transaction (if you have balance)
    if balance > 0 {
        let recipient = "NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c".parse()?;
        let tx_hash = neo_token.transfer(
            account.clone(),
            recipient,
            100000000, // 1 NEO (8 decimals)
            None,
        ).await?;
        
        println!("🚀 Transaction sent: {}", tx_hash);
    }
    
    Ok(())
}
```

**🎉 Congratulations!** You're now ready to build with the NeoRust SDK.

---

## 🎯 What's Next?

### **Explore More Features**

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🎨 NFT Operations</h3>
      </div>
      <div className="card__body">
        <p>Learn how to mint, transfer, and manage NFTs across all interfaces.</p>
      </div>
      <div className="card__footer">
        <a href="../gui/nft-operations" className="button button--primary">Learn NFTs →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🔧 Smart Contracts</h3>
      </div>
      <div className="card__body">
        <p>Deploy and interact with smart contracts using NeoRust tools.</p>
      </div>
      <div className="card__footer">
        <a href="../sdk/examples" className="button button--primary">Build Contracts →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🏢 Enterprise</h3>
      </div>
      <div className="card__body">
        <p>Scale to production with enterprise-grade features and security.</p>
      </div>
      <div className="card__footer">
        <a href="../advanced/architecture" className="button button--primary">Go Enterprise →</a>
      </div>
    </div>
  </div>
</div>

### **Join the Community**

- **GitHub**: [Star the repository](https://github.com/R3E-Network/NeoRust) ⭐
- **Discussions**: [Join conversations](https://github.com/R3E-Network/NeoRust/discussions) 💬
- **Issues**: [Report bugs or request features](https://github.com/R3E-Network/NeoRust/issues) 🐛

### **Need Help?**

- **Documentation**: [Complete guides](https://neorust.netlify.app) 📚
- **Examples**: [Real-world examples](../examples/basic) 💡
- **Support**: [Community support](https://github.com/R3E-Network/NeoRust/discussions) 🤝

---

**Ready to build the future of Neo N3? Let's go! 🚀** 