# Examples

Practical examples for common NeoRust SDK operations.

## Token Transfer

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let sender = Account::from_private_key("your_private_key")?;
    let signer = AccountSigner::new(sender);
    
    let gas_token = GasToken::new(&client);
    let recipient = "NXXXXxxxXXXxxxXXXxxxXXXxxxXXXxxx".to_script_hash()?;
    let amount = 1000000000; // 10 GAS
    
    let tx_hash = gas_token
        .transfer(&signer, &recipient, amount, None)
        .await?;
    
    println!("Transfer transaction: {}", tx_hash);
    Ok(())
}
```

## Smart Contract Invocation

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678".parse::<ScriptHash>()?;
    
    let result = client
        .invoke_function(
            &contract_hash,
            "getValue",
            vec![ContractParameter::String("key1".to_string())],
            vec![],
        )
        .await?;
    
    println!("Contract result: {:?}", result);
    Ok(())
}
```

## More Examples

For more comprehensive examples, visit our [Examples Page](/examples). 