# Neo X

## Overview

Neo X is an EVM-compatible chain maintained by Neo, enabling developers to leverage Ethereum compatibility while benefiting from Neo's infrastructure and security. The Neo X module in NeoRust provides interfaces for interacting with this EVM-compatible environment using Alloy.

## Key Features

- **EVM Compatibility Layer**: Interact with Neo X as an Ethereum-compatible chain via Alloy
- **Unified Ecosystem Client**: Seamlessly write code that operates across Neo N3 and Neo X
- **Protected RPC Routing**: Optionally route requests through a third-party Anti-MEV endpoint; the service may mitigate MEV exposure but does not guarantee prevention
- **Bridge Functionality**: Transfer tokens seamlessly between Neo N3 and Neo X natively
- **Transaction Support**: Create, sign, and send EVM transactions on Neo X

## Components

### Unified Ecosystem Client

The `EcosystemClient` is the recommended way to interact with Neo X. It provides a standard interface to both N3 and Neo X, reducing the need for duplicate logic when your application touches both ecosystems.

```rust,no_run
use neo3::sdk::unified::EcosystemClient;
use neo3::neo_x::NeoXWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new randomized EVM Wallet (or load from PK)
    let evm_wallet = NeoXWallet::create_random();

    // Route requests through the configured third-party protected RPC endpoint
    let client = EcosystemClient::new_neox_anti_mev(evm_wallet);

    // Get Balance directly via the Unified Interface
    let balance = client.get_balance().await?;
    println!("Neo X Balance: {} Wei", balance);

    Ok(())
}
```

### Neo X Provider & Wallet

If you need deeper access, you can work directly with the `NeoXProvider` and `NeoXWallet`. The `NeoXProvider` exposes an Alloy `RootProvider` for executing low-level EVM requests.

```rust,no_run
use neo3::neo_clients::{HttpProvider, RpcClient};
use neo3::neo_x::{NeoXProvider, NeoXWallet, NeoXClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let neo_x_provider: NeoXProvider<'_, HttpProvider> = NeoXProvider::new("https://rpc.neo-x.org", None);
    
    // Extract the Alloy provider if you need native EVM interactions
    let _raw_evm = neo_x_provider.evm_provider().unwrap();
    let chain_id = neo_x_provider.chain_id().await?;

    let wallet = NeoXWallet::create_random();
    let client = NeoXClient::new(wallet, neo_x_provider);
    
    Ok(())
}
```

### Neo X Bridge

The bridge facilitates token transfers between Neo N3 and Neo X. The unified client abstracts this complex process so you can trigger a cross-chain transfer directly:

```rust,no_run
use neo3::sdk::unified::EcosystemClient;
use neo3::neo_x::NeoXWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = NeoXWallet::create_random();
    let client = EcosystemClient::new_neox_anti_mev(wallet);

    // Bridge funds from Neo X back to an N3 address
    // Parameters: destination_address (N3), amount (Wei)
    let tx_hash = client.bridge_to_other_chain("NXX...YourN3Address", "1000000000").await?;
    println!("Bridge Tx: {}", tx_hash);

    Ok(())
}
```

## Integration with Ethereum Tools

Neo X's EVM compatibility enables integration with popular Ethereum development tools:

- **Alloy**: NeoRust uses Alloy for EVM providers, signing, transaction types, and RPC interoperability.
- **Metamask**: Connect Metamask to Neo X by adding it as a custom network.
- **Hardhat/Foundry**: Deploy Solidity contracts to Neo X.
- **Web3.js/ethers.js**: Interact with Neo X using JavaScript libraries.

## Considerations

- **Gas Costs**: Neo X uses a gas model similar to Ethereum. Transactions cost native GAS token.
- **Cross-Chain Operations**: Bridge operations require network confirmations and may take a few minutes to finalize.
- **Security**: The optional Anti-MEV mode routes through a third-party protected RPC endpoint. Its effectiveness depends on that service and does not guarantee prevention of front-running or sandwich attacks.

## Related Documentation

- [Neo X EVM Code Examples](../../examples/neo_x/examples/)
- [Bridge Operations](bridge.md)
- [EVM Contracts](evm-contracts.md)
