---
sidebar_position: 5
---

# 🌉 Neo X & EVM Integration

NeoRust natively supports **Neo X** (the EVM-compatible sidechain) and bridges the gap between NeoVM and EVM environments via `ethers-rs`.

## Ecosystem Client

The `EcosystemClient` provides a unified, cross-chain interface to easily manage operations across Neo N3 and Neo X.

```rust
use neo3::sdk::unified::EcosystemClient;
use neo3::neo_x::NeoXWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a randomized secure EVM wallet
    let wallet = NeoXWallet::create_random();

    // Initialize an Anti-MEV protected EcosystemClient for Neo X
    // Automatically guards against front-running via a protected mempool
    let client = EcosystemClient::new_neox_anti_mev(wallet);

    // Query balance using standard ethers-rs providers underneath
    let balance = client.get_balance().await?;
    println!("EVM Balance: {} Wei", balance);

    // Easily bridge funds back to an N3 address
    let tx_hash = client.bridge_to_other_chain("NXX...YourN3Address", "1000000000").await?;
    println!("Bridge Tx: {}", tx_hash);
    
    Ok(())
}
```

## Anti-MEV Configuration

When building applications on the Neo X EVM, front-running and sandwich attacks are a risk (especially in DeFi). The `NeoXProvider` allows initialization with an Anti-MEV endpoint which obscures transaction ordering until inclusion.

```rust
use neo3::neo_x::NeoXProvider;
use neo3::neo_clients::{HttpProvider, RpcClient};

// Initialize an Anti-MEV protected NeoXProvider directly
let provider = NeoXProvider::new_anti_mev(None);
```

## Ethers-rs Compatibility

Because the `neo_x` module leverages the standard `ethers-rs` ecosystem underneath, you can retrieve the underlying HTTP provider and use it alongside other EVM development tools directly:

```rust
let neo_x_provider: NeoXProvider<'_, HttpProvider> = NeoXProvider::new("https://rpc.neo-x.org", None);
    
// Extract the raw ethers provider if you need native EVM interactions
let raw_evm = neo_x_provider.evm_provider().unwrap();
let chain_id = raw_evm.get_chainid().await.unwrap();
```