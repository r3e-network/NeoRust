# Transaction Management

Build, sign, and broadcast transactions on the Neo network.

## Creating Transactions

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let account = Account::from_private_key("your_private_key")?;
    let signer = AccountSigner::new(account);
    
    let mut tx_builder = TransactionBuilder::new(&client);
    
    // Add operations to the transaction
    let gas_token = GasToken::new(&client);
    let recipient = "NAddress".to_script_hash()?;
    tx_builder.add_transfer(&gas_token.script_hash(), &recipient, 1000000000)?;
    
    let tx = tx_builder.build_and_sign(&signer).await?;
    let tx_hash = client.send_raw_transaction(&tx).await?;
    
    println!("Transaction sent: {}", tx_hash);
    Ok(())
}
```

## Transaction Types

- **Transfer**: Move tokens between accounts
- **Contract Call**: Invoke smart contract methods
- **Multi-operation**: Combine multiple operations

## Monitoring

Track transaction status and confirmations.

## Fees

Understand network fees and priority settings. 