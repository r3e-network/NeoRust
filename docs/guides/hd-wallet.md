# HD Wallet Guide (v0.5.3)

NeoRust ships a BIP-39/44 compatible HD wallet in `neo3::sdk::hd_wallet`.

## Generate and Derive

```rust,no_run
use neo3::sdk::hd_wallet::HDWallet;
use bip39::Language;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = HDWallet::generate(12, None)?;
    let _mnemonic = wallet.mnemonic_phrase();
    // SECURITY: Store the mnemonic securely offline. Avoid logging it.

    let account = wallet.derive_account("m/44'/888'/0'/0/0")?;
    println!("Address: {}", account.get_address());

    // Restore from mnemonic
    let _restored = HDWallet::from_phrase(wallet.mnemonic_phrase(), None, Language::English)?;
    Ok(())
}
```

## Best Practices
- Store mnemonics offline and encrypt at rest.
- Derive accounts deterministically and track paths alongside addresses.
- Use different accounts for production, testnet, and development.

## Encrypted Export / Import

`HDWallet` can export its mnemonic and derived paths as an encrypted JSON blob using
scrypt (Neo defaults) + AES-256. This is intended for “encrypt at rest” use cases.

```rust,no_run
use neo3::sdk::hd_wallet::HDWallet;
use bip39::Language;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
let mut wallet = HDWallet::from_phrase(phrase, None, Language::English)?;
wallet.derive_account("m/44'/888'/0'/0/0")?;

let encrypted_json = wallet.export_encrypted("my strong password")?;
let restored = HDWallet::import_encrypted(&encrypted_json, "my strong password")?;

assert_eq!(restored.mnemonic_phrase(), phrase);
# Ok(())
# }
```
