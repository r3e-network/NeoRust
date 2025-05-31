# CLI Commands Reference

<div className="hero hero--primary">
  <div className="container">
    <h1 className="hero__title">📋 CLI Commands Reference</h1>
    <p className="hero__subtitle">
      Complete command documentation for NeoRust CLI
    </p>
    <p>
      Master every command and option available in the powerful NeoRust command-line interface.
    </p>
  </div>
</div>

## 📚 Command Overview

The NeoRust CLI provides comprehensive blockchain operations through an intuitive command structure.

### **Command Structure**
```bash
neo-cli [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS] [ARGUMENTS]
```

### **Global Options**
```bash
-h, --help       Show help information
-V, --version    Show version information
-v, --verbose    Enable verbose output
-q, --quiet      Suppress non-essential output
--config <FILE>  Use custom configuration file
--log-level <LEVEL>  Set logging level (error, warn, info, debug, trace)
--no-color       Disable colored output
```

---

## 💼 Wallet Commands

### **`neo-cli wallet`**

Comprehensive wallet management operations.

#### **`wallet create`**
Create a new wallet with secure key generation.

```bash
neo-cli wallet create [OPTIONS] --name <NAME>
```

**Options:**
```bash
-n, --name <NAME>           Wallet name (required)
-p, --path <PATH>           Custom wallet file path
--password-prompt           Prompt for password securely
--password <PASSWORD>       Set password (not recommended)
--accounts <COUNT>          Number of accounts to create [default: 1]
--derivation-path <PATH>    Custom derivation path
--entropy <ENTROPY>         Custom entropy source
--format <FORMAT>           Output format (json, yaml, table) [default: table]
```

**Examples:**
```bash
# Basic wallet creation
neo-cli wallet create --name "MyWallet"

# Create wallet with multiple accounts
neo-cli wallet create --name "MultiWallet" --accounts 5

# Create wallet with custom path
neo-cli wallet create --name "CustomWallet" --path "./wallets/custom.json"

# Create wallet with password prompt
neo-cli wallet create --name "SecureWallet" --password-prompt
```

**Output:**
```
✅ Wallet created successfully!
📁 Location: ~/.neorust/wallets/MyWallet.json
🔑 Default Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
⚠️  Please backup your wallet file and recovery phrase securely!

Recovery Phrase:
abandon ability able about above absent absorb abstract absurd abuse access accident

🔐 Keep this phrase safe - it's the only way to recover your wallet!
```

#### **`wallet open`**
Open and unlock an existing wallet.

```bash
neo-cli wallet open [OPTIONS] <PATH>
```

**Options:**
```bash
-p, --password <PASSWORD>   Wallet password
--password-prompt           Prompt for password securely
--readonly                  Open in read-only mode
--timeout <SECONDS>         Auto-lock timeout [default: 300]
```

**Examples:**
```bash
# Open wallet with password prompt
neo-cli wallet open ./my-wallet.json --password-prompt

# Open wallet in read-only mode
neo-cli wallet open ./my-wallet.json --readonly

# Open with custom timeout
neo-cli wallet open ./my-wallet.json --timeout 600
```

#### **`wallet close`**
Close and lock the currently open wallet.

```bash
neo-cli wallet close [OPTIONS]
```

**Options:**
```bash
--force     Force close without confirmation
--save      Save changes before closing
```

#### **`wallet list`**
List all available wallets.

```bash
neo-cli wallet list [OPTIONS]
```

**Options:**
```bash
--path <PATH>       Search path for wallets [default: ~/.neorust/wallets]
--format <FORMAT>   Output format (table, json, yaml) [default: table]
--detailed          Show detailed information
```

**Output:**
```
📁 Available Wallets:
┌─────────────────┬──────────────────────────────────────┬─────────────┬─────────────┐
│ Name            │ Path                                 │ Accounts    │ Last Used   │
├─────────────────┼──────────────────────────────────────┼─────────────┼─────────────┤
│ MyWallet        │ ~/.neorust/wallets/MyWallet.json    │ 1           │ 2 hours ago │
│ TradingWallet   │ ~/.neorust/wallets/Trading.json     │ 3           │ 1 day ago   │
│ ColdStorage     │ ~/.neorust/wallets/Cold.json        │ 1           │ 1 week ago  │
└─────────────────┴──────────────────────────────────────┴─────────────┴─────────────┘
```

#### **`wallet info`**
Display detailed information about the current wallet.

```bash
neo-cli wallet info [OPTIONS]
```

**Options:**
```bash
--format <FORMAT>   Output format (table, json, yaml) [default: table]
--include-keys      Include public keys (dangerous)
--include-balance   Include balance information
```

**Output:**
```
💼 Wallet Information:
├─ Name: MyWallet
├─ Version: 3.0
├─ Accounts: 3
├─ Default Account: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
├─ Encrypted: ✅ Yes
├─ Created: 2024-01-15 14:30:22
├─ Last Modified: 2024-05-31 10:15:33
└─ File Size: 2.1 KB

🔑 Accounts:
├─ Account 1: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc (default)
├─ Account 2: NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c
└─ Account 3: NY9WpJ3qKyqK8gLbTKrP3tJ5cAqd2c
```

#### **`wallet balance`**
Check wallet balance across all accounts.

```bash
neo-cli wallet balance [OPTIONS]
```

**Options:**
```bash
--account <ADDRESS>     Check specific account balance
--asset <ASSET>         Filter by specific asset (NEO, GAS, or contract hash)
--detailed              Show detailed breakdown
--format <FORMAT>       Output format (table, json, yaml) [default: table]
--include-nfts          Include NFT holdings
--fiat-currency <CURR>  Show fiat values (USD, EUR, etc.)
```

**Examples:**
```bash
# Check all balances
neo-cli wallet balance

# Check specific account
neo-cli wallet balance --account NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc

# Check only NEO balance
neo-cli wallet balance --asset NEO

# Detailed balance with fiat values
neo-cli wallet balance --detailed --fiat-currency USD
```

**Output:**
```
💰 Wallet Balance:

┌─────────────────────────────────────┬─────────┬──────────────┬─────────────┬──────────────┐
│ Account                             │ Asset   │ Balance      │ USD Value   │ 24h Change   │
├─────────────────────────────────────┼─────────┼──────────────┼─────────────┼──────────────┤
│ NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc │ NEO     │ 100.00000000 │ $2,567.00   │ +5.2%        │
│                                     │ GAS     │ 1,250.123456 │ $1,875.19   │ +3.1%        │
├─────────────────────────────────────┼─────────┼──────────────┼─────────────┼──────────────┤
│ NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c │ NEO     │ 50.00000000  │ $1,283.50   │ +5.2%        │
│                                     │ GAS     │ 750.000000   │ $1,125.00   │ +3.1%        │
└─────────────────────────────────────┴─────────┴──────────────┴─────────────┴──────────────┘

💎 Total Portfolio Value: $6,850.69 (+4.8% 24h)
```

#### **`wallet send`**
Send tokens to another address.

```bash
neo-cli wallet send [OPTIONS] --to <ADDRESS> --amount <AMOUNT> --asset <ASSET>
```

**Options:**
```bash
-t, --to <ADDRESS>          Recipient address (required)
-a, --amount <AMOUNT>       Amount to send (required)
--asset <ASSET>             Asset to send (NEO, GAS, or contract hash) [default: NEO]
--from <ADDRESS>            Sender address (uses default if not specified)
--fee <AMOUNT>              Custom network fee
--priority <LEVEL>          Transaction priority (low, normal, high) [default: normal]
--message <MESSAGE>         Optional message/memo
--dry-run                   Simulate transaction without sending
--confirm                   Skip confirmation prompt
```

**Examples:**
```bash
# Send NEO tokens
neo-cli wallet send --to NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c --amount 10.0 --asset NEO

# Send GAS with custom fee
neo-cli wallet send --to NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c --amount 100.0 --asset GAS --fee 0.01

# Send with message
neo-cli wallet send --to NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c --amount 5.0 --asset NEO --message "Payment for services"

# Dry run to estimate fees
neo-cli wallet send --to NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c --amount 10.0 --asset NEO --dry-run
```

**Output:**
```
🚀 Preparing Transaction:
├─ From: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
├─ To: NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c
├─ Amount: 10.00000000 NEO
├─ Network Fee: 0.00123456 GAS (estimated)
├─ System Fee: 0.00100000 GAS
└─ Total Cost: 10.00000000 NEO + 0.00223456 GAS

❓ Confirm transaction? [y/N]: y

✅ Transaction sent successfully!
📝 Transaction Hash: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
⏱️  Estimated confirmation time: 15 seconds
🔍 View on explorer: https://explorer.onegate.space/transaction/0x1234...
```

#### **`wallet history`**
View transaction history for the wallet.

```bash
neo-cli wallet history [OPTIONS]
```

**Options:**
```bash
--account <ADDRESS>     Filter by specific account
--asset <ASSET>         Filter by asset type
--limit <COUNT>         Number of transactions to show [default: 20]
--offset <COUNT>        Skip first N transactions [default: 0]
--type <TYPE>           Filter by transaction type (send, receive, contract)
--from-date <DATE>      Start date (YYYY-MM-DD)
--to-date <DATE>        End date (YYYY-MM-DD)
--format <FORMAT>       Output format (table, json, csv) [default: table]
--export <FILE>         Export to file
```

**Examples:**
```bash
# Recent transactions
neo-cli wallet history --limit 10

# Filter by account and asset
neo-cli wallet history --account NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc --asset NEO

# Export to CSV
neo-cli wallet history --format csv --export transactions.csv

# Date range filter
neo-cli wallet history --from-date 2024-01-01 --to-date 2024-01-31
```

---

## 🎨 NFT Commands

### **`neo-cli nft`**

Complete NFT lifecycle management.

#### **`nft deploy`**
Deploy a new NFT collection contract.

```bash
neo-cli nft deploy [OPTIONS] --name <NAME> --symbol <SYMBOL>
```

**Options:**
```bash
-n, --name <NAME>           Collection name (required)
-s, --symbol <SYMBOL>       Collection symbol (required)
--description <DESC>        Collection description
--max-supply <COUNT>        Maximum supply (optional)
--base-uri <URI>            Base URI for metadata
--royalty <PERCENT>         Creator royalty percentage [default: 5]
--mutable                   Allow metadata updates
--burnable                  Allow token burning
--pausable                  Allow contract pausing
--access-control            Enable role-based access
```

**Examples:**
```bash
# Basic collection deployment
neo-cli nft deploy --name "My Art Collection" --symbol "MAC"

# Advanced collection with features
neo-cli nft deploy \
  --name "Premium NFTs" \
  --symbol "PNFT" \
  --description "Exclusive digital art collection" \
  --max-supply 1000 \
  --royalty 10 \
  --burnable \
  --pausable
```

**Output:**
```
🚀 Deploying NFT Collection:
├─ Name: My Art Collection
├─ Symbol: MAC
├─ Max Supply: Unlimited
├─ Royalty: 5%
├─ Features: Standard NEP-11
└─ Estimated Gas: 15.7 GAS

✅ Collection deployed successfully!
📝 Contract Hash: 0x1234567890abcdef1234567890abcdef12345678
🔍 View on explorer: https://explorer.onegate.space/contract/0x1234...
💾 Contract saved to: ./contracts/MAC_contract.json
```

#### **`nft mint`**
Mint new NFTs in a collection.

```bash
neo-cli nft mint [OPTIONS] --contract <HASH> --to <ADDRESS> --token-id <ID>
```

**Options:**
```bash
-c, --contract <HASH>       NFT contract hash (required)
-t, --to <ADDRESS>          Recipient address (required)
--token-id <ID>             Token ID (required)
--metadata <URI>            Metadata URI
--metadata-file <FILE>      Local metadata file
--properties <JSON>         Token properties as JSON
--batch <FILE>              Batch mint from CSV file
--start-id <ID>             Starting token ID for batch
--count <COUNT>             Number of tokens to mint
```

**Examples:**
```bash
# Mint single NFT
neo-cli nft mint \
  --contract 0x1234567890abcdef1234567890abcdef12345678 \
  --to NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc \
  --token-id 001 \
  --metadata ipfs://QmYourMetadataHash

# Batch mint from file
neo-cli nft mint \
  --contract 0x1234567890abcdef1234567890abcdef12345678 \
  --batch ./mint_list.csv

# Mint with local metadata
neo-cli nft mint \
  --contract 0x1234567890abcdef1234567890abcdef12345678 \
  --to NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc \
  --token-id 002 \
  --metadata-file ./metadata/002.json
```

#### **`nft transfer`**
Transfer NFT ownership.

```bash
neo-cli nft transfer [OPTIONS] --contract <HASH> --token-id <ID> --to <ADDRESS>
```

**Options:**
```bash
-c, --contract <HASH>       NFT contract hash (required)
--token-id <ID>             Token ID (required)
-t, --to <ADDRESS>          Recipient address (required)
--from <ADDRESS>            Sender address (uses default if not specified)
--data <DATA>               Additional transfer data
--message <MESSAGE>         Transfer message/memo
```

#### **`nft info`**
Get detailed information about an NFT.

```bash
neo-cli nft info [OPTIONS] --contract <HASH> --token-id <ID>
```

**Options:**
```bash
-c, --contract <HASH>       NFT contract hash (required)
--token-id <ID>             Token ID (required)
--include-metadata          Fetch and display metadata
--include-history           Show ownership history
--format <FORMAT>           Output format (table, json, yaml) [default: table]
```

**Output:**
```
🎨 NFT Information:
├─ Collection: My Art Collection (MAC)
├─ Token ID: 001
├─ Owner: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
├─ Creator: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
├─ Minted: 2024-01-15 14:30:22
├─ Metadata URI: ipfs://QmYourMetadataHash
└─ Contract: 0x1234567890abcdef1234567890abcdef12345678

📊 Metadata:
├─ Name: "Awesome Digital Art #1"
├─ Description: "A unique piece of digital art"
├─ Image: ipfs://QmYourImageHash
├─ Attributes:
│   ├─ Color: Blue (15% rarity)
│   ├─ Style: Abstract (8% rarity)
│   └─ Edition: 1 of 100
└─ Properties:
    ├─ Creator: Artist Name
    └─ Category: Digital Art

🔄 Ownership History:
├─ 2024-01-15: Minted to NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
└─ Current owner since: 2024-01-15 (136 days)
```

---

## 🌐 Network Commands

### **`neo-cli network`**

Network connectivity and blockchain interaction.

#### **`network connect`**
Connect to a Neo N3 network.

```bash
neo-cli network connect [OPTIONS] --network <NETWORK>
```

**Options:**
```bash
-n, --network <NETWORK>     Network name or RPC URL (required)
--timeout <SECONDS>         Connection timeout [default: 30]
--retry-attempts <COUNT>    Number of retry attempts [default: 3]
--health-check              Enable periodic health checks
--set-default               Set as default network
```

**Examples:**
```bash
# Connect to TestNet
neo-cli network connect --network testnet

# Connect to MainNet
neo-cli network connect --network mainnet

# Connect to custom RPC
neo-cli network connect --network https://custom-node.com:443

# Connect with health monitoring
neo-cli network connect --network testnet --health-check --set-default
```

#### **`network status`**
Display current network status and statistics.

```bash
neo-cli network status [OPTIONS]
```

**Options:**
```bash
--detailed              Show detailed network information
--format <FORMAT>       Output format (table, json, yaml) [default: table]
--refresh <SECONDS>     Auto-refresh interval
--export <FILE>         Export status to file
```

**Output:**
```
🌐 Network Status: Neo N3 TestNet

┌─────────────────────┬─────────────────────────────────────────┐
│ Property            │ Value                                   │
├─────────────────────┼─────────────────────────────────────────┤
│ 🔗 RPC Endpoint     │ https://testnet1.neo.coz.io:443       │
│ 📊 Block Height     │ 2,345,678                              │
│ ⏱️  Block Time       │ 15.2s (average)                        │
│ 🔄 TPS              │ 12.7 transactions/second               │
│ 💾 Memory Pool      │ 156 pending transactions               │
│ 🌍 Connected Peers  │ 42 nodes                               │
│ ⛽ Network Fee      │ 0.00123456 GAS (recommended)           │
│ 🔄 Sync Status      │ ✅ Synchronized (100%)                 │
│ 📡 Latency          │ 89ms                                   │
│ ⏰ Last Block       │ 12 seconds ago                         │
│ 🏗️  Protocol Version │ 3.6.0                                  │
└─────────────────────┴─────────────────────────────────────────┘

🎯 Network Health: ✅ Excellent (99.97% uptime)
```

#### **`network list`**
List all configured networks.

```bash
neo-cli network list [OPTIONS]
```

**Options:**
```bash
--format <FORMAT>       Output format (table, json, yaml) [default: table]
--include-custom        Include custom networks
--test-connectivity     Test connection to each network
```

#### **`network add`**
Add a custom network configuration.

```bash
neo-cli network add [OPTIONS] --name <NAME> --rpc <URL>
```

**Options:**
```bash
-n, --name <NAME>           Network name (required)
-r, --rpc <URL>             RPC endpoint URL (required)
--magic <NUMBER>            Network magic number
--description <DESC>        Network description
--explorer <URL>            Block explorer URL
--set-default               Set as default network
```

#### **`network remove`**
Remove a custom network configuration.

```bash
neo-cli network remove [OPTIONS] --name <NAME>
```

#### **`network test`**
Test connectivity to a network.

```bash
neo-cli network test [OPTIONS] --network <NETWORK>
```

**Options:**
```bash
-n, --network <NETWORK>     Network to test (required)
--timeout <SECONDS>         Test timeout [default: 10]
--detailed                  Show detailed test results
--benchmark                 Run performance benchmark
```

---

## 🔧 Tools Commands

### **`neo-cli tools`**

Developer utilities and helper functions.

#### **`tools encode`**
Encode data in various formats.

```bash
neo-cli tools encode [OPTIONS] --input <DATA> --format <FORMAT>
```

**Options:**
```bash
-i, --input <DATA>          Input data (required)
-f, --format <FORMAT>       Output format (base64, hex, base58) (required)
--input-file <FILE>         Read input from file
--output-file <FILE>        Write output to file
--line-length <LENGTH>      Line length for formatted output
```

**Examples:**
```bash
# Encode string to Base64
neo-cli tools encode --input "Hello, Neo N3!" --format base64

# Encode file to hex
neo-cli tools encode --input-file contract.nef --format hex

# Encode with line breaks
neo-cli tools encode --input "Long text..." --format base64 --line-length 64
```

#### **`tools decode`**
Decode data from various formats.

```bash
neo-cli tools decode [OPTIONS] --input <DATA> --format <FORMAT>
```

**Options:**
```bash
-i, --input <DATA>          Input data (required)
-f, --format <FORMAT>       Input format (base64, hex, base58) (required)
--input-file <FILE>         Read input from file
--output-file <FILE>        Write output to file
--validate                  Validate format before decoding
```

#### **`tools hash`**
Generate cryptographic hashes.

```bash
neo-cli tools hash [OPTIONS] --input <DATA> --algorithm <ALGO>
```

**Options:**
```bash
-i, --input <DATA>          Input data (required)
-a, --algorithm <ALGO>      Hash algorithm (sha256, ripemd160, keccak256) (required)
--input-file <FILE>         Read input from file
--output-format <FORMAT>    Output format (hex, base64) [default: hex]
--uppercase                 Use uppercase hex output
```

**Examples:**
```bash
# Hash string with SHA256
neo-cli tools hash --input "NeoRust" --algorithm sha256

# Hash file
neo-cli tools hash --input-file contract.nef --algorithm sha256

# Multiple algorithms
neo-cli tools hash --input "test" --algorithm sha256
neo-cli tools hash --input "test" --algorithm ripemd160
```

#### **`tools random`**
Generate cryptographically secure random data.

```bash
neo-cli tools random [OPTIONS] --length <LENGTH>
```

**Options:**
```bash
-l, --length <LENGTH>       Length in bytes (required)
-f, --format <FORMAT>       Output format (hex, base64, base58) [default: hex]
--count <COUNT>             Number of random values to generate [default: 1]
--seed <SEED>               Custom seed (for testing only)
```

**Examples:**
```bash
# Generate 32 random bytes
neo-cli tools random --length 32

# Generate multiple random values
neo-cli tools random --length 16 --count 5 --format base64

# Generate random private key
neo-cli tools random --length 32 --format hex
```

#### **`tools validate-address`**
Validate Neo N3 address format.

```bash
neo-cli tools validate-address [OPTIONS] --address <ADDRESS>
```

**Options:**
```bash
-a, --address <ADDRESS>     Address to validate (required)
--detailed                  Show detailed validation info
--format <FORMAT>           Output format (table, json) [default: table]
```

**Output:**
```
🔍 Address Validation: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc

✅ Valid Neo N3 Address
├─ Format: Base58Check
├─ Version: 0x35 (Neo N3)
├─ Script Hash: 1234567890abcdef1234567890abcdef12345678
├─ Checksum: ✅ Valid
└─ Address Type: Standard (P2PKH)
```

#### **`tools generate-address`**
Generate new Neo N3 addresses.

```bash
neo-cli tools generate-address [OPTIONS]
```

**Options:**
```bash
--count <COUNT>             Number of addresses to generate [default: 1]
--format <FORMAT>           Output format (table, json, csv) [default: table]
--include-keys              Include private keys (dangerous)
--save-to-file <FILE>       Save to file
--multisig <M> <N>          Generate M-of-N multisig address
```

---

## 📊 Advanced Commands

### **`neo-cli contract`**

Smart contract operations.

#### **`contract invoke`**
Invoke a smart contract method.

```bash
neo-cli contract invoke [OPTIONS] --contract <HASH> --method <METHOD>
```

**Options:**
```bash
-c, --contract <HASH>       Contract hash (required)
-m, --method <METHOD>       Method name (required)
--params <JSON>             Method parameters as JSON array
--signers <ADDRESSES>       Signer addresses
--witnesses <WITNESSES>     Custom witnesses
--dry-run                   Simulate without sending transaction
--gas-limit <AMOUNT>        Maximum gas to consume
```

#### **`contract deploy`**
Deploy a smart contract.

```bash
neo-cli contract deploy [OPTIONS] --file <NEF_FILE> --manifest <MANIFEST_FILE>
```

**Options:**
```bash
-f, --file <NEF_FILE>       Contract NEF file (required)
-m, --manifest <MANIFEST>   Contract manifest file (required)
--data <DATA>               Deployment data
--update-counter <COUNT>    Update counter for upgrades
```

### **`neo-cli block`**

Blockchain data queries.

#### **`block get`**
Get block information.

```bash
neo-cli block get [OPTIONS] <BLOCK_IDENTIFIER>
```

**Options:**
```bash
--format <FORMAT>           Output format (table, json, yaml) [default: table]
--include-transactions      Include transaction details
--verbose                   Show verbose information
```

#### **`block search`**
Search for blocks by criteria.

```bash
neo-cli block search [OPTIONS]
```

**Options:**
```bash
--from-height <HEIGHT>      Starting block height
--to-height <HEIGHT>        Ending block height
--from-time <TIMESTAMP>     Starting timestamp
--to-time <TIMESTAMP>       Ending timestamp
--limit <COUNT>             Maximum results [default: 10]
```

---

## 🎯 Usage Examples

### **Complete Workflow Examples**

#### **Setting up a new wallet and making first transaction**
```bash
# 1. Create new wallet
neo-cli wallet create --name "MyFirstWallet" --password-prompt

# 2. Connect to TestNet
neo-cli network connect --network testnet --set-default

# 3. Check wallet info
neo-cli wallet info

# 4. Check balance (should be 0)
neo-cli wallet balance

# 5. Get test tokens from faucet (external step)
# Visit https://neowish.ngd.network/ and request tokens

# 6. Check balance again
neo-cli wallet balance

# 7. Send tokens to another address
neo-cli wallet send \
  --to NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c \
  --amount 10.0 \
  --asset NEO

# 8. Check transaction history
neo-cli wallet history --limit 5
```

#### **NFT Collection Creation and Minting**
```bash
# 1. Deploy NFT collection
neo-cli nft deploy \
  --name "My Art Collection" \
  --symbol "MAC" \
  --description "Unique digital artworks" \
  --max-supply 1000 \
  --royalty 5

# 2. Mint first NFT
neo-cli nft mint \
  --contract 0x1234567890abcdef1234567890abcdef12345678 \
  --to NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc \
  --token-id 001 \
  --metadata ipfs://QmYourMetadataHash

# 3. Check NFT info
neo-cli nft info \
  --contract 0x1234567890abcdef1234567890abcdef12345678 \
  --token-id 001 \
  --include-metadata

# 4. Transfer NFT
neo-cli nft transfer \
  --contract 0x1234567890abcdef1234567890abcdef12345678 \
  --token-id 001 \
  --to NX8GVjjjhyZNhMhmdBbg1KrP3tJ5cAqd2c
```

---

## 🆘 Command Help

### **Getting Help**

Every command supports the `--help` flag for detailed usage information:

```bash
# General help
neo-cli --help

# Command-specific help
neo-cli wallet --help
neo-cli wallet create --help
neo-cli nft mint --help
```

### **Common Options**

Most commands support these common options:
- `--help, -h`: Show help information
- `--verbose, -v`: Enable verbose output
- `--quiet, -q`: Suppress non-essential output
- `--format <FORMAT>`: Choose output format (table, json, yaml, csv)
- `--config <FILE>`: Use custom configuration file

### **Exit Codes**

The CLI uses standard exit codes:
- `0`: Success
- `1`: General error
- `2`: Invalid arguments
- `3`: Network error
- `4`: Authentication error
- `5`: Insufficient funds

---

**Master the NeoRust CLI and unlock the full power of Neo N3! 🚀** 