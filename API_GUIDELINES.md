# NeoRust API Design Guidelines

This document outlines the API design principles and guidelines for the NeoRust SDK to ensure consistency, usability, and professionalism.

## 🎯 Core Principles

### 1. **Clarity Over Brevity**
```rust
// ❌ Bad
pub fn tx(h: &str) -> Result<Tx, E> { ... }

// ✅ Good
pub fn get_transaction(hash: &str) -> Result<Transaction, Neo3Error> { ... }
```

### 2. **Type Safety First**
```rust
// ❌ Bad - using strings for everything
pub fn transfer(from: &str, to: &str, amount: &str) -> Result<String, Error>

// ✅ Good - using proper types
pub fn transfer(
    from: &Address,
    to: &Address,
    amount: Amount,
) -> Result<TransactionId, Neo3Error>
```

### 3. **Builder Pattern for Complex Objects**
```rust
// ✅ Good - builder pattern for transactions
let tx = TransactionBuilder::new()
    .script(script)
    .add_signer(signer)
    .valid_until_block(height + 100)
    .build()?;
```

### 4. **Consistent Error Handling**
```rust
// All errors implement std::error::Error and provide context
#[derive(Debug, thiserror::Error)]
pub enum Neo3Error {
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: Amount, available: Amount },
}
```

## 📚 Module Organization

### Public API Structure
```rust
// crates/neo3/src/lib.rs
pub mod prelude {
    // Common imports for users
    pub use neo3_types::{Address, ScriptHash, Amount};
    pub use neo3_rpc::{RpcClient, HttpProvider};
    pub use neo3_builder::TransactionBuilder;
    pub use crate::error::Neo3Error;
}

// Re-export from subcrates
pub use neo3_types as types;
pub use neo3_crypto as crypto;
pub use neo3_rpc as rpc;
pub use neo3_builder as builder;
pub use neo3_wallets as wallets;
pub use neo3_contracts as contracts;

// High-level convenience APIs
pub mod neo {
    pub async fn connect(url: &str) -> Result<Client, Neo3Error> { ... }
}
```

## 🔧 API Patterns

### 1. **Async-First Design**
```rust
// All I/O operations are async
impl RpcClient {
    pub async fn get_block_count(&self) -> Result<u64, Neo3Error> { ... }
    pub async fn get_transaction(&self, hash: &TxId) -> Result<Transaction, Neo3Error> { ... }
}
```

### 2. **Trait-Based Abstractions**
```rust
// Define traits for extensibility
#[async_trait]
pub trait Provider: Send + Sync {
    async fn send_request<T, R>(&self, method: &str, params: T) -> Result<R, Neo3Error>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned;
}

// Multiple implementations
pub struct HttpProvider { ... }
pub struct WebSocketProvider { ... }
pub struct IpcProvider { ... }
```

### 3. **Method Chaining**
```rust
// Enable fluent interfaces
impl WalletBuilder {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    pub fn with_password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }
    
    pub fn build(self) -> Result<Wallet, Neo3Error> { ... }
}
```

### 4. **Option/Result Patterns**
```rust
// Use Option for optional values
impl Account {
    pub fn label(&self) -> Option<&str> { ... }
}

// Use Result for fallible operations
impl Wallet {
    pub fn decrypt(&self, password: &str) -> Result<DecryptedWallet, Neo3Error> { ... }
}
```

## 📝 Documentation Standards

### 1. **Module-Level Documentation**
```rust
//! # Neo3 RPC Client
//! 
//! This module provides a high-performance RPC client for interacting with Neo N3 nodes.
//! 
//! ## Examples
//! 
//! ```rust
//! use neo3_rpc::{RpcClient, HttpProvider};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let provider = HttpProvider::new("https://mainnet1.neo.org")?;
//!     let client = RpcClient::new(provider);
//!     
//!     let block_count = client.get_block_count().await?;
//!     println!("Current block height: {}", block_count);
//!     
//!     Ok(())
//! }
//! ```
```

### 2. **Function Documentation**
```rust
/// Sends a transaction to the network.
/// 
/// # Arguments
/// 
/// * `transaction` - The signed transaction to send
/// 
/// # Returns
/// 
/// Returns the transaction hash if successful.
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - The transaction is invalid
/// - The network is unreachable
/// - The transaction is rejected by the network
/// 
/// # Examples
/// 
/// ```rust
/// # use neo3_rpc::RpcClient;
/// # async fn example(client: &RpcClient) -> Result<(), Box<dyn std::error::Error>> {
/// let tx = create_transaction()?;
/// let tx_hash = client.send_transaction(&tx).await?;
/// println!("Transaction sent: {}", tx_hash);
/// # Ok(())
/// # }
/// ```
pub async fn send_transaction(&self, transaction: &Transaction) -> Result<TxId, Neo3Error> {
    // Implementation
}
```

### 3. **Type Documentation**
```rust
/// Represents a Neo N3 address.
/// 
/// An address is a base58-encoded string that represents a script hash
/// with a version byte and checksum.
/// 
/// # Examples
/// 
/// ```rust
/// use neo3_types::Address;
/// use std::str::FromStr;
/// 
/// let address = Address::from_str("NKuyBkoGdZZSLyPbJEetheRhMjeznFZszf")?;
/// let script_hash = address.to_script_hash();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(String);
```

## 🎨 Naming Conventions

### Types
- **Structs**: `PascalCase` - `TransactionBuilder`, `RpcClient`
- **Enums**: `PascalCase` - `NetworkType`, `WitnessScope`
- **Traits**: `PascalCase` - `Provider`, `Signer`
- **Type Aliases**: `PascalCase` - `TxId`, `BlockHash`

### Functions and Methods
- **Functions**: `snake_case` - `get_block_count`, `send_transaction`
- **Constructors**: `new` or descriptive - `new`, `from_wif`, `with_provider`
- **Getters**: No `get_` prefix - `balance()` not `get_balance()`
- **Setters**: `set_` prefix - `set_network()`
- **Predicates**: `is_` or `has_` prefix - `is_valid()`, `has_balance()`

### Constants and Statics
- **Constants**: `SCREAMING_SNAKE_CASE` - `MAX_TRANSACTION_SIZE`
- **Static Variables**: `SCREAMING_SNAKE_CASE` - `DEFAULT_TIMEOUT`

### Modules
- **Module Names**: `snake_case` - `transaction_builder`, `rpc_client`
- **File Names**: `snake_case` - `script_hash.rs`, `address.rs`

## 🔐 Security Considerations

### 1. **No Secrets in APIs**
```rust
// ❌ Bad - exposing private key
pub struct Account {
    pub private_key: String,
}

// ✅ Good - keeping private key private
pub struct Account {
    private_key: SecretString,
}

impl Account {
    pub fn sign(&self, message: &[u8]) -> Signature { ... }
}
```

### 2. **Input Validation**
```rust
impl Address {
    pub fn from_str(s: &str) -> Result<Self, Neo3Error> {
        // Validate address format
        if !is_valid_address(s) {
            return Err(Neo3Error::InvalidAddress(s.to_string()));
        }
        Ok(Self(s.to_string()))
    }
}
```

### 3. **Secure Defaults**
```rust
impl ClientConfig {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            verify_tls: true, // Secure by default
        }
    }
}
```

## 🚀 Performance Guidelines

### 1. **Zero-Copy Where Possible**
```rust
// Use references instead of cloning
impl Transaction {
    pub fn hash(&self) -> &TxId {
        &self.hash
    }
}
```

### 2. **Lazy Evaluation**
```rust
// Compute expensive values only when needed
impl Block {
    pub fn merkle_root(&self) -> &Hash {
        self.merkle_root.get_or_init(|| {
            calculate_merkle_root(&self.transactions)
        })
    }
}
```

### 3. **Efficient Serialization**
```rust
// Implement custom serialization for performance
impl Serialize for ScriptHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Efficient hex serialization
        serializer.serialize_str(&self.to_hex())
    }
}
```

## ✅ API Checklist

Before releasing an API:

- [ ] All public items are documented
- [ ] Examples are provided for complex APIs
- [ ] Error cases are documented
- [ ] Breaking changes are noted
- [ ] Performance characteristics are documented
- [ ] Security considerations are addressed
- [ ] API follows naming conventions
- [ ] Tests cover all public APIs
- [ ] Benchmarks exist for performance-critical APIs
- [ ] API is reviewed by team