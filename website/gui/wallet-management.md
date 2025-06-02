# Wallet Management

Complete guide to managing Neo wallets with the NeoRust Desktop GUI.

## Overview 💼

The NeoRust Desktop GUI provides comprehensive wallet management capabilities with enterprise-grade security and user-friendly interface. Whether you're a newcomer to blockchain or an experienced developer, our wallet management system offers the tools you need.

### Key Features
- **Multi-wallet support** - Manage multiple wallets simultaneously
- **Secure storage** - Bank-grade encryption for private keys
- **Backup & recovery** - Multiple backup options for peace of mind
- **Hardware wallet integration** - Support for Ledger devices
- **Real-time monitoring** - Live balance and transaction updates

## Creating Wallets 🆕

### Create New Wallet

1. **Launch the Application**
   - Open NeoRust Desktop GUI
   - Click "Create New Wallet" on the welcome screen

2. **Security Setup**
   - Generate a strong password (12+ characters recommended)
   - Enable two-factor authentication if available
   - Choose encryption strength (AES-256 default)

3. **Seed Phrase Generation**
   ```
   ⚠️ CRITICAL: Your seed phrase is the master key to your wallet
   - Write down all 12/24 words in order
   - Store in a secure, offline location
   - Never share with anyone
   - Test recovery before adding funds
   ```

4. **Verification**
   - Re-enter your seed phrase to confirm
   - Set up additional security questions
   - Complete wallet creation

### Import Existing Wallet

#### From Seed Phrase
1. Click "Import Wallet" → "From Seed Phrase"
2. Enter your 12/24 word seed phrase
3. Set new password for this device
4. Verify addresses match your expectations

#### From Private Key
1. Click "Import Wallet" → "From Private Key"
2. Enter WIF (Wallet Import Format) private key
3. Set password and security options
4. Wallet ready for use

#### From Keystore File
1. Click "Import Wallet" → "From Keystore"
2. Select your NEP-6 keystore file
3. Enter keystore password
4. Set new device password

## Wallet Security 🔒

### Security Best Practices

#### Strong Passwords
- **Minimum 12 characters**
- **Mix of uppercase, lowercase, numbers, symbols**
- **Unique to this wallet** (don't reuse)
- **Use password manager** for complex passwords

#### Two-Factor Authentication
```
Available 2FA Methods:
✅ TOTP (Google Authenticator, Authy)
✅ SMS (less secure, not recommended)
✅ Hardware keys (YubiKey, etc.)
✅ Biometric (if supported by device)
```

#### Hardware Wallet Integration
**Supported Devices:**
- Ledger Nano S/X/S Plus
- Future: Trezor support planned

**Setup Process:**
1. Connect hardware wallet via USB
2. Install Neo app on device
3. Follow GUI prompts to pair
4. Transactions require device confirmation

### Backup & Recovery

#### Seed Phrase Backup
```bash
# Best practices for seed phrase storage:
1. Write on paper (waterproof ink)
2. Store in fireproof safe
3. Consider metal backup plates
4. Split storage (partial phrases in different locations)
5. Test recovery annually
```

#### Encrypted Backup
1. **File Export**
   - Go to Settings → Backup
   - Choose "Encrypted Backup"
   - Select destination folder
   - Backup includes: addresses, labels, settings

2. **Cloud Backup** (Optional)
   - Enable encrypted cloud sync
   - Choose provider (Google Drive, iCloud, etc.)
   - Automatic backup scheduling
   - Zero-knowledge encryption

#### Recovery Process
1. **From Seed Phrase**
   ```
   Steps:
   1. Install NeoRust GUI on new device
   2. Select "Recover Wallet"
   3. Enter seed phrase
   4. Set new password
   5. Wait for blockchain sync
   ```

2. **From Backup File**
   ```
   Steps:
   1. File → Import Backup
   2. Select backup file
   3. Enter backup password
   4. Verify addresses
   5. Sync complete
   ```

## Account Management 👥

### Multiple Accounts per Wallet

#### Creating Accounts
1. Open wallet settings
2. Click "Add Account"
3. Choose derivation path (or use default)
4. Label the account (e.g., "Savings", "Trading")
5. Account ready for use

#### Account Types
- **Standard Account**: Regular Neo address for general use
- **Multi-signature Account**: Requires multiple signatures
- **Contract Account**: For smart contract interactions
- **Watch-only Account**: Monitor without spending ability

### Address Management

#### Address Labels
```
Organize your addresses:
- Personal: "My Main Address"
- Business: "Company Treasury" 
- Exchange: "Binance Withdrawal"
- DeFi: "Flamingo Pool"
```

#### QR Code Generation
- Click any address to generate QR code
- Includes address and optional amount
- Perfect for receiving payments
- Print or share digitally

## Transaction Management 📊

### Transaction History

#### Viewing Transactions
- **All Transactions**: Complete history across all accounts
- **Filter by Account**: Focus on specific addresses
- **Search**: Find transactions by hash, address, or amount
- **Export**: CSV/PDF reports for accounting

#### Transaction Details
Each transaction shows:
- **Amount & Token**: NEO, GAS, or custom tokens
- **Addresses**: From/to with labels if available
- **Status**: Confirmed, pending, or failed
- **Block Information**: Height, timestamp, confirmations
- **Gas Fee**: Network fee paid

### Pending Transactions
- **Real-time Monitoring**: Track unconfirmed transactions
- **Fee Bumping**: Increase fee for faster confirmation
- **Cancellation**: Cancel unconfirmed transactions
- **Rebroadcast**: Resend stuck transactions

## Address Book 📇

### Managing Contacts

#### Adding Contacts
1. Click "Address Book" in main menu
2. Select "Add Contact"
3. Enter address and label
4. Optional: Add notes, categories
5. Save contact

#### Contact Categories
```
Organize by purpose:
🏢 Business Partners
👨‍👩‍👧‍👦 Family & Friends  
🏦 Exchanges
💱 DeFi Protocols
🎮 Gaming Platforms
```

#### Import/Export Contacts
- **Import**: From CSV, vCard, or other wallets
- **Export**: Backup contacts to external file
- **Sync**: Cross-device contact synchronization

## Advanced Features 🚀

### Multi-signature Wallets

#### Creating Multi-sig
1. Navigate to "Advanced" → "Multi-signature"
2. Set required signatures (e.g., 2 of 3)
3. Add co-signer public keys
4. Deploy multi-sig contract
5. Share contract address with co-signers

#### Using Multi-sig
- **Creating Transactions**: Initiate with your signature
- **Signing**: Co-signers add their signatures
- **Broadcasting**: Automatic when threshold reached
- **Monitoring**: Track signature collection progress

### Watch-only Wallets
```
Use cases:
✅ Monitor exchange balances
✅ Track team treasury
✅ Audit partner wallets
✅ Portfolio tracking
```

Setup:
1. "Add Wallet" → "Watch-only"
2. Enter public address
3. Set monitoring preferences
4. View-only access (no spending)

### Wallet Analytics

#### Balance Overview
- **Total Portfolio Value**: USD/fiat equivalent
- **Asset Allocation**: Pie chart of holdings
- **Performance**: Gains/losses over time
- **Yield Tracking**: Staking and DeFi rewards

#### Transaction Analytics
- **Spending Patterns**: Category-based analysis
- **Monthly Reports**: Income vs expenses
- **Tax Preparation**: Capital gains calculations
- **Custom Reports**: Flexible date ranges and filters

## Network Management 🌐

### Network Selection
- **MainNet**: Production network for real transactions
- **TestNet**: Free testing environment
- **Custom RPC**: Connect to private or local networks

### Node Configuration
```json
Custom RPC Settings:
{
  "mainnet": "https://rpc10.n3.nspcc.ru:10331",
  "testnet": "https://rpc.t5.n3.nspcc.ru:20331",
  "local": "http://localhost:20332"
}
```

## Troubleshooting 🔧

### Common Issues

#### Wallet Won't Open
1. **Check password**: Ensure caps lock, keyboard language
2. **File corruption**: Restore from backup
3. **Version compatibility**: Update to latest version
4. **Permissions**: Ensure read/write access to wallet folder

#### Missing Transactions
1. **Sync Status**: Wait for complete blockchain sync
2. **Network Issues**: Check internet connection
3. **RPC Problems**: Try different RPC endpoint
4. **Refresh**: Force refresh transaction history

#### Balance Showing Zero
1. **Address Verification**: Confirm you're viewing correct address
2. **Network Selection**: Ensure correct network (MainNet vs TestNet)
3. **Sync Progress**: Check synchronization status
4. **Node Issues**: Switch to different RPC node

### Performance Optimization

#### Faster Syncing
- **Select Fast Sync**: Skip full blockchain download
- **Reliable RPC**: Use well-connected nodes
- **Bandwidth**: Ensure stable internet connection
- **Hardware**: SSD storage recommended

#### Memory Usage
- **Close Unused Wallets**: Reduce memory footprint
- **Cache Settings**: Adjust cache size in preferences
- **Update Software**: Latest version has optimizations

## Security Checklist ✅

### Daily Security
- [ ] Password manager for wallet passwords
- [ ] Antivirus software updated
- [ ] OS security updates installed
- [ ] Suspicious activity monitoring

### Weekly Security
- [ ] Backup verification test
- [ ] Software update check
- [ ] Transaction history review
- [ ] Access log examination

### Monthly Security
- [ ] Full backup to offline storage
- [ ] Security audit of practices
- [ ] Hardware wallet firmware update
- [ ] Recovery procedure test

### Emergency Procedures

#### Compromised Device
1. **Immediate**: Transfer funds to new wallet
2. **Assessment**: Determine extent of compromise
3. **Recovery**: Restore from clean backup
4. **Prevention**: Enhanced security measures

#### Lost Seed Phrase
1. **Don't Panic**: Check all storage locations
2. **Partial Recovery**: Some words may be recoverable
3. **Professional Help**: Consider data recovery services
4. **Prevention**: Multiple backup locations

## Pro Tips 💡

### Efficiency Tips
- **Keyboard Shortcuts**: Learn common shortcuts
- **Batch Operations**: Group multiple transactions
- **Templates**: Save common transaction patterns
- **Automation**: Set up recurring transactions

### Privacy Tips
- **Address Rotation**: Use new addresses regularly
- **Mixing Services**: Enhance transaction privacy
- **VPN Usage**: Protect network traffic
- **Coin Selection**: Manual UTXO management

### Organization Tips
```
Best Practices:
📁 Separate wallets by purpose
🏷️ Consistent labeling system
📊 Regular balance reconciliation
📋 Transaction categorization
📅 Periodic security reviews
```

## Getting Help 🆘

### Support Resources
- **Documentation**: [Complete guides](../docs)
- **Video Tutorials**: [YouTube channel](https://youtube.com/neorust)
- **Community Forum**: [forum.neorust.org](https://forum.neorust.org)
- **Discord Chat**: [Real-time help](https://discord.gg/neo-rust)

### Reporting Issues
1. **Bug Reports**: GitHub issues with logs
2. **Feature Requests**: Community voting platform
3. **Security Issues**: Responsible disclosure program
4. **General Questions**: Community forums

---

**Ready to master wallet management?** Start with creating your first secure wallet and explore all the powerful features NeoRust GUI offers! 🚀 