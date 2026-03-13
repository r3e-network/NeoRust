//! # Neo X
//!
//! Support for Neo X, an EVM-compatible chain maintained by Neo.
//!
//! ## Overview
//!
//! The neo_x module provides interfaces for interacting with Neo X, an EVM-compatible
//! chain maintained by Neo. It includes:
//!
//! - EVM compatibility layer for interacting with Neo X as an Ethereum-compatible chain
//! - Bridge functionality for transferring tokens between Neo N3 and Neo X
//! - EVM Wallet and client wrappers powered by `ethers-rs`
//! - Anti-MEV RPC support
//! - Transaction creation and signing for Neo X
//! - Provider interfaces for connecting to Neo X nodes
//!
//! This module enables seamless integration between Neo N3 and EVM-compatible ecosystems,
//! allowing developers to leverage both blockchain environments natively.
//!
//! ## Examples
//!
//! ### Connecting to Neo X and getting chain information
//!
//! ```no_run
//! use neo3::neo_clients::{HttpProvider, RpcClient};
//! use neo3::neo_x::NeoXProvider;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Neo N3 (optional, for cross-chain ops)
//!     let neo_provider = HttpProvider::new("https://mainnet1.neo.org:443")?;
//!     let neo_client = RpcClient::new(neo_provider);
//!
//!     // Initialize the Neo X EVM provider (Standard or Anti-MEV)
//!     let neo_x_provider = NeoXProvider::new_anti_mev(Some(&neo_client));
//!
//!     // Get the chain ID for Neo X natively via ethers-rs or fallback to N3 node
//!     let chain_id = neo_x_provider.chain_id().await?;
//!     println!("Neo X Chain ID: {}", chain_id);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Using NeoXWallet to check EVM balance
//!
//! ```no_run
//! use neo3::neo_clients::{HttpProvider, RpcClient};
//! use neo3::neo_x::{NeoXProvider, NeoXWallet, NeoXClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let neo_x_provider: NeoXProvider<'_, HttpProvider> = NeoXProvider::new("https://rpc.neo-x.org", None);
//!     
//!     // Create a random wallet or load from private key
//!     let wallet = NeoXWallet::create_random();
//!     println!("Generated EVM Address: {:?}", wallet.address());
//!
//!     let client = NeoXClient::new(wallet, neo_x_provider);
//!     let balance = client.get_balance().await.unwrap();
//!     println!("Balance: {}", balance);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Using the bridge to transfer tokens between Neo N3 and Neo X
//!
//! ```no_run
//! use neo3::neo_clients::{HttpProvider, RpcClient};
//! use neo3::neo_protocol::{Account, AccountTrait};
//! use neo3::neo_types::ScriptHash;
//! use neo3::neo_x::NeoXBridgeContract;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Neo N3
//!     let neo_provider = HttpProvider::new("https://mainnet1.neo.org:443")?;
//!     let neo_client = RpcClient::new(neo_provider);
//!
//!     // Initialize the bridge contract
//!     let bridge = NeoXBridgeContract::new(Some(&neo_client))?;
//!
//!     // Get the GAS token script hash
//!     let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
//!
//!     // Query bridge fee for the token
//!     let fee = bridge.get_fee(&gas_token).await?;
//!     println!("Bridge fee: {}", fee);
//!
//!     Ok(())
//! }
//! ```

/// Bridge operations between Neo N3 and Neo X
pub mod bridge;
/// EVM compatibility helpers for Neo X
pub mod evm;

pub use bridge::*;
pub use evm::*;
