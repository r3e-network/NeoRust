# Wallet Management

Learn how to create and manage wallets with NeoRust SDK.

## Creating a New Wallet

```rust
use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = Wallet::new();
    wallet.set_name("MyWallet".to_string());
    
    let account = Account::create()?;
    wallet.add_account(account);
    
    println!("Wallet created with {} accounts", wallet.get_accounts().len());
    Ok(())
}
```

## Loading from NEP-6

```rust
use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet_json = std::fs::read_to_string("wallet.json")?;
    let nep6_wallet: Nep6Wallet = serde_json::from_str(&wallet_json)?;
    
    let password = "your_password";
    let wallet = Wallet::from_nep6(&nep6_wallet, password)?;
    
    println!("Loaded wallet: {}", wallet.get_name());
    Ok(())
}
```

## Account Management

- Create new accounts
- Import existing accounts
- Manage multiple accounts
- Set default account

## Security

- Password protection
- Secure key storage
- Hardware wallet integration 