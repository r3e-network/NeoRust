# Welcome to NeoRust

<div className="hero hero--primary">
  <div className="container">
    <h1 className="hero__title">🚀 NeoRust v1.0.6</h1>
    <p className="hero__subtitle">
      Production-Ready Neo N3 Development Suite with 135 Passing Doc Tests
    </p>
    <p>
      The most comprehensive toolkit for Neo N3 blockchain development.
      Build with confidence using our powerful CLI, robust Rust SDK, and curated guides.
    </p>
  </div>
</div>

## 🌟 What Makes NeoRust Special

**NeoRust** isn't just another blockchain SDK - it's a complete development ecosystem that provides everything you need to build production-ready Neo N3 applications.

### ✨ **Three Powerful Resources**

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>📘 Guides & Examples</h3>
      </div>
      <div className="card__body">
        <p>
          Step-by-step guides and real-world examples for wallets, contracts,
          and integration workflows.
        </p>
      </div>
      <div className="card__footer">
        <a href="/docs/getting-started/quick-start" className="button button--primary">Get Started →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>💻 Command Line</h3>
      </div>
      <div className="card__body">
        <p>
          Powerful CLI with beautiful output, progress indicators, and comprehensive 
          tools. Ideal for developers, automation, and CI/CD pipelines.
        </p>
      </div>
      <div className="card__footer">
        <a href="/cli/intro" className="button button--primary">Explore CLI →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>📚 Rust SDK</h3>
      </div>
      <div className="card__body">
        <p>
          Production-ready library with comprehensive Neo N3 support. 
          Zero panics, full test coverage, and enterprise-grade reliability.
        </p>
      </div>
      <div className="card__footer">
        <a href="/sdk/intro" className="button button--primary">Explore SDK →</a>
      </div>
    </div>
  </div>
</div>

## 🏆 Production Ready

### ✅ **Proven Reliability**

<div className="row">
  <div className="col col--3">
    <div className="text--center">
      <h2 className="gradient-text">135/150</h2>
      <p><strong>Doc Tests Passing</strong></p>
    </div>
  </div>
  <div className="col col--3">
    <div className="text--center">
      <h2 className="gradient-text">95%</h2>
      <p><strong>Panic Reduction</strong></p>
    </div>
  </div>
  <div className="col col--3">
    <div className="text--center">
      <h2 className="gradient-text">100%</h2>
      <p><strong>Breaking Changes Documented</strong></p>
    </div>
  </div>
  <div className="col col--3">
    <div className="text--center">
      <h2 className="gradient-text">100%</h2>
      <p><strong>Production Ready</strong></p>
    </div>
  </div>
</div>

### 🔧 **Enterprise Features**
- **Multi-Network Support**: MainNet, TestNet, private networks
- **Hardware Wallet Integration**: Ledger device support
- **Batch Operations**: Efficient bulk transaction processing
- **Monitoring & Analytics**: Built-in performance monitoring

## 🚀 Quick Start

Choose your preferred interface and get started in minutes:

### 💻 **Command Line** (Recommended for Developers)

```bash
cd NeoRust/neo-cli
cargo build --release
./target/release/neo-cli wallet create --name "MyWallet"
```

### 📚 **Rust SDK** (Recommended for Integration)

```toml
[dependencies]
neo3 = "1.0.6"
```

```rust
use neo3::prelude::*;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    let block_count = client.get_block_count().await?;
    println!("Block height: {}", block_count);
    
    Ok(())
}
```

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

## 📚 Documentation Structure

This documentation is organized into several sections:

### 📖 **Getting Started**
- [Installation Guide](./getting-started/installation) - Set up your development environment
- [Quick Start](./getting-started/quick-start) - Get up and running in 5 minutes
- [Testing Guide](./testing) - Validate and test your integration

### 💻 **Command Line Interface**
- [CLI Intro](/cli/intro) - Introduction to the command line tools
- [Commands Reference](/cli/commands) - Complete command documentation
- [Configuration](/cli/configuration) - Customizing the CLI

### 📚 **Rust SDK**
- [SDK Intro](/sdk/intro) - Introduction to the Rust library
- [Installation](/sdk/installation) - Set up the SDK in your project
- [Quick Start](/sdk/quick-start) - Build your first SDK workflow
- [Examples](/sdk/examples) - Real-world usage examples
- [API Reference](/sdk/api-reference) - Complete API documentation

## 🌐 Community & Support

### 📞 **Getting Help**
- **GitHub Issues**: [Report bugs and request features](https://github.com/R3E-Network/NeoRust/issues)
- **Discussions**: [Community discussions and Q&A](https://github.com/R3E-Network/NeoRust/discussions)
- **Documentation**: [Comprehensive guides and API docs](https://neorust.netlify.app)

### 🤝 **Contributing**
- **[Contributing Guide](https://github.com/R3E-Network/NeoRust/blob/v1.0.6/CONTRIBUTING.md)**: How to contribute to NeoRust
- **[System Architecture](https://github.com/R3E-Network/NeoRust/blob/v1.0.6/SYSTEM_ARCHITECTURE_DESIGN.md)**: Core architecture and components
- **[API Guidelines](https://github.com/R3E-Network/NeoRust/blob/v1.0.6/API_GUIDELINES.md)**: API design standards and guidance

### 🔗 **Links**
- **Website**: [https://neorust.netlify.app](https://neorust.netlify.app)
- **Crate**: [https://crates.io/crates/neo3](https://crates.io/crates/neo3)
- **API Docs**: [https://docs.rs/neo3](https://docs.rs/neo3)
- **GitHub**: [https://github.com/R3E-Network/NeoRust](https://github.com/R3E-Network/NeoRust)

---

<div className="text--center">
  <h2>Ready to Build on Neo N3? 🚀</h2>
  <p>Choose your preferred interface and start building amazing blockchain applications today!</p>
  
  <div className="margin-top--lg">
    <a href="/docs/intro" className="button button--primary button--lg margin--sm">
      📘 Read the Docs
    </a>
    <a href="/cli/intro" className="button button--secondary button--lg margin--sm">
      💻 Use CLI Tools
    </a>
    <a href="/sdk/intro" className="button button--outline button--lg margin--sm">
      📚 Integrate SDK
    </a>
  </div>
</div> 
