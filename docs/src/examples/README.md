# NeoRust Examples

This section contains various examples demonstrating how to use the NeoRust SDK for different Neo blockchain operations.

## Available Example Categories

### Basic Operations
Examples demonstrating fundamental Neo N3 operations:
- **Network Connection**: Basic connectivity to Neo N3 TestNet/MainNet
- **Account Management**: Creating and managing Neo accounts
- **Balance Checking**: Querying NEO and GAS balances
- **Transaction Creation**: Building and sending basic transactions

### Wallet Management
Examples for wallet operations:
- **Wallet Creation**: Creating new Neo wallets with NEP-6 format
- **Account Import/Export**: Working with WIF and mnemonic phrases
- **Message Signing**: Cryptographic message signing and verification
- **Multi-signature Wallets**: Creating and managing multi-sig accounts

### Smart Contracts
Examples for smart contract interaction:
- **Contract Invocation**: Calling methods on deployed contracts
- **NEP-17 Tokens**: Working with fungible tokens
- **NEP-11 NFTs**: Non-fungible token operations
- **Famous Contracts**: Interacting with popular Neo ecosystem contracts

### Advanced Features
- **Neo X Integration**: Cross-chain operations with Neo X EVM chain
- **NeoFS Operations**: Decentralized file storage operations
- **Oracle Integration**: Using Neo's Oracle service
- **Cryptographic Operations**: Advanced key management and signing

## Running the Examples

Each example can be run from the root of the repository using:

```bash
cargo run --example <example_name>
```

For example:

```bash
cargo run --example wallet_creation
```

Or navigate to the specific example directory and run:

```bash
cargo run
```

## Example Code Structure

Most examples follow this structure:

1. **Setup**: Establishing connection to Neo nodes
2. **Account preparation**: Loading or creating accounts
3. **Main operation**: Performing the specific blockchain operation
4. **Verification**: Checking the results of the operation

## Adding Your Own Examples

If you've created a useful example and would like to contribute it, please follow these steps:

1. Create a new directory under `/examples` with a descriptive name
2. Add your Rust code and a proper `Cargo.toml` file
3. Document your example with clear comments
4. Create a pull request to the NeoRust repository

## Full Example List

Here's a list of all available examples in the repository:

- [basic_connection](https://github.com/username/NeoRust/tree/main/examples/basic_connection)
- [wallet_management](https://github.com/username/NeoRust/tree/main/examples/wallet_management)
- [message_signing](https://github.com/username/NeoRust/tree/main/examples/message_signing)
- [neo_x](https://github.com/username/NeoRust/tree/main/examples/neo_x)
- [simple_transfer](https://github.com/username/NeoRust/tree/main/examples/simple_transfer)
- [contract_invocation](https://github.com/username/NeoRust/tree/main/examples/contract_invocation)