# Token Operations

Work with NEP-17 and NEP-11 tokens on the Neo network.

## NEP-17 Tokens (Fungible)

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // GAS token operations
    let gas_token = GasToken::new(&client);
    let symbol = gas_token.symbol().await?;
    let decimals = gas_token.decimals().await?;
    
    println!("Token: {} (decimals: {})", symbol, decimals);
    
    // Check balance
    let address = "NAddress".to_script_hash()?;
    let balance = gas_token.balance_of(&address).await?;
    println!("Balance: {} {}", balance, symbol);
    
    Ok(())
}
```

## Custom NEP-17 Tokens

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let token_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    let token = Nep17Token::new(&client, token_hash);
    
    let symbol = token.symbol().await?;
    let total_supply = token.total_supply().await?;
    
    println!("Token: {} (supply: {})", symbol, total_supply);
    Ok(())
}
```

## NEP-11 NFTs

Support for Non-Fungible Tokens using the NEP-11 standard.

## Token Standards

- **NEP-17**: Fungible tokens (like ERC-20)
- **NEP-11**: Non-fungible tokens (like ERC-721)
- Native tokens: NEO and GAS 