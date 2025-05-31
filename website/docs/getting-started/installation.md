# Installation Guide

<div className="hero hero--primary">
  <div className="container">
    <h1 className="hero__title">🚀 Installation Guide</h1>
    <p className="hero__subtitle">
      Get NeoRust up and running in minutes
    </p>
    <p>
      Choose your preferred installation method and start building on Neo N3 today.
    </p>
  </div>
</div>

## 📋 System Requirements

### **Minimum Requirements**
- **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- **Memory**: 4GB RAM
- **Storage**: 2GB free space
- **Network**: Stable internet connection

### **Recommended Requirements**
- **Operating System**: Latest version of Windows 11, macOS 13+, or Linux
- **Memory**: 8GB+ RAM
- **Storage**: 10GB+ free space (for blockchain data)
- **Network**: High-speed internet connection

### **Development Requirements**
- **Node.js**: 18.0+ (for GUI development)
- **Rust**: 1.70+ (for SDK and CLI development)
- **Git**: Latest version
- **Code Editor**: VS Code, IntelliJ, or similar

## 🖥️ Desktop GUI Installation

### **Option 1: Download Pre-built Application**

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🪟 Windows</h3>
      </div>
      <div className="card__body">
        <p>Download the MSI installer for easy installation.</p>
        <pre><code>NeoRust-Desktop-v0.1.9-x64.msi</code></pre>
      </div>
      <div className="card__footer">
        <a href="https://github.com/R3E-Network/NeoRust/releases" className="button button--primary">
          Download for Windows
        </a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🍎 macOS</h3>
      </div>
      <div className="card__body">
        <p>Download the DMG file for macOS installation.</p>
        <pre><code>NeoRust-Desktop-v0.1.9.dmg</code></pre>
      </div>
      <div className="card__footer">
        <a href="https://github.com/R3E-Network/NeoRust/releases" className="button button--primary">
          Download for macOS
        </a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🐧 Linux</h3>
      </div>
      <div className="card__body">
        <p>Download the AppImage for universal Linux support.</p>
        <pre><code>NeoRust-Desktop-v0.1.9.AppImage</code></pre>
      </div>
      <div className="card__footer">
        <a href="https://github.com/R3E-Network/NeoRust/releases" className="button button--primary">
          Download for Linux
        </a>
      </div>
    </div>
  </div>
</div>

### **Option 2: Build from Source**

```bash
# Clone the repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust/neo-gui

# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

## 💻 CLI Installation

### **Option 1: Download Pre-built Binary**

```bash
# Linux/macOS
curl -L https://github.com/R3E-Network/NeoRust/releases/latest/download/neo-cli-linux -o neo-cli
chmod +x neo-cli
sudo mv neo-cli /usr/local/bin/

# Verify installation
neo-cli --version
```

```powershell
# Windows PowerShell
Invoke-WebRequest -Uri "https://github.com/R3E-Network/NeoRust/releases/latest/download/neo-cli-windows.exe" -OutFile "neo-cli.exe"

# Add to PATH or run directly
.\neo-cli.exe --version
```

### **Option 2: Install via Cargo**

```bash
# Install from crates.io
cargo install neo-cli

# Or install from git
cargo install --git https://github.com/R3E-Network/NeoRust.git neo-cli

# Verify installation
neo-cli --version
```

### **Option 3: Build from Source**

```bash
# Clone the repository
git clone https://github.com/R3E-Network/NeoRust.git
cd NeoRust/neo-cli

# Build the CLI
cargo build --release

# The binary will be at ./target/release/neo-cli
./target/release/neo-cli --version
```

## 📚 Rust SDK Installation

### **Add to Cargo.toml**

```toml
[dependencies]
neo3 = "0.1.9"

# Optional features
neo3 = { version = "0.1.9", features = ["futures", "ledger", "aws"] }
```

### **Available Features**

| Feature | Description | Use Case |
|---------|-------------|----------|
| `futures` | Async/await support | High-performance applications |
| `ledger` | Hardware wallet support | Enhanced security |
| `aws` | AWS integration | Cloud deployments |
| `default` | Core functionality | Basic usage |

### **Basic Usage Example**

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Neo N3 TestNet
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    // Get blockchain information
    let block_count = client.get_block_count().await?;
    println!("Current block height: {}", block_count);
    
    Ok(())
}
```

## 🔧 Development Environment Setup

### **VS Code Setup**

1. **Install Extensions**:
   ```json
   {
     "recommendations": [
       "rust-lang.rust-analyzer",
       "bradlc.vscode-tailwindcss",
       "ms-vscode.vscode-typescript-next",
       "tauri-apps.tauri-vscode"
     ]
   }
   ```

2. **Configure Settings**:
   ```json
   {
     "rust-analyzer.cargo.features": ["futures", "ledger"],
     "typescript.preferences.importModuleSpecifier": "relative"
   }
   ```

### **IntelliJ IDEA Setup**

1. **Install Plugins**:
   - Rust Plugin
   - Tauri Plugin
   - TypeScript Support

2. **Configure Rust Toolchain**:
   - Set Rust toolchain to stable
   - Enable Cargo features: `futures`, `ledger`

## 🌐 Network Configuration

### **Default Networks**

NeoRust comes pre-configured with these networks:

| Network | RPC Endpoint | Magic Number |
|---------|--------------|--------------|
| **MainNet** | `https://mainnet1.neo.coz.io:443` | `860833102` |
| **TestNet** | `https://testnet1.neo.coz.io:443` | `894710606` |

### **Custom Network Setup**

```bash
# CLI: Add custom network
neo-cli network add --name "Private" --rpc "https://private-node.com:443" --magic 123456789

# GUI: Use Settings > Network Configuration
# SDK: Configure in code
let provider = HttpProvider::new("https://custom-node.com:443")?;
```

## ✅ Verification

### **Test Your Installation**

#### **Desktop GUI**
1. Launch the application
2. Create a test wallet
3. Connect to TestNet
4. Verify network synchronization

#### **CLI**
```bash
# Test basic functionality
neo-cli --version
neo-cli network status
neo-cli wallet create --name "TestWallet"
```

#### **SDK**
```rust
// Create a test project
cargo new neo-test
cd neo-test

// Add neo3 dependency and run
cargo add neo3
cargo run
```

## 🔍 Troubleshooting

### **Common Issues**

#### **GUI Won't Start**
```bash
# Check Node.js version
node --version  # Should be 18+

# Clear npm cache
npm cache clean --force

# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

#### **CLI Not Found**
```bash
# Check PATH
echo $PATH

# Verify binary location
which neo-cli

# Reinstall if needed
cargo install --force neo-cli
```

#### **SDK Compilation Errors**
```bash
# Update Rust toolchain
rustup update

# Clear cargo cache
cargo clean

# Check feature flags
cargo check --features "futures,ledger"
```

### **Getting Help**

- **GitHub Issues**: [Report problems](https://github.com/R3E-Network/NeoRust/issues)
- **Discussions**: [Community support](https://github.com/R3E-Network/NeoRust/discussions)
- **Documentation**: [Complete guides](https://neorust.netlify.app)

## 🎯 Next Steps

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🚀 Quick Start</h3>
      </div>
      <div className="card__body">
        <p>Get up and running with your first Neo N3 application in 5 minutes.</p>
      </div>
      <div className="card__footer">
        <a href="./quick-start" className="button button--primary">Start Building →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>💼 First Wallet</h3>
      </div>
      <div className="card__body">
        <p>Create your first Neo wallet and start managing digital assets.</p>
      </div>
      <div className="card__footer">
        <a href="./first-wallet" className="button button--primary">Create Wallet →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>📚 Learn More</h3>
      </div>
      <div className="card__body">
        <p>Explore advanced features and learn best practices.</p>
      </div>
      <div className="card__footer">
        <a href="../intro" className="button button--primary">Explore Features →</a>
      </div>
    </div>
  </div>
</div>

---

**Installation complete? Let's start building! 🚀** 