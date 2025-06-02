# Smart Contracts

Deploy and interact with smart contracts on Neo N3.

## Contract Invocation

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    
    // Read-only call
    let result = client
        .invoke_function(
            &contract_hash,
            "getValue",
            vec![ContractParameter::String("key".to_string())],
            vec![],
        )
        .await?;
    
    println!("Result: {:?}", result);
    Ok(())
}
```

## State-Changing Calls

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let account = Account::from_private_key("private_key")?;
    let signer = AccountSigner::new(account);
    
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    
    let tx_hash = client
        .invoke_function_tx(
            &signer,
            &contract_hash,
            "setValue",
            vec![
                ContractParameter::String("key".to_string()),
                ContractParameter::String("value".to_string()),
            ],
            None,
        )
        .await?;
    
    println!("Transaction: {}", tx_hash);
    Ok(())
}
```

## Features

- Contract deployment
- Method invocation
- Event monitoring
- Parameter handling 