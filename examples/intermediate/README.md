# Intermediate Examples

These examples cover more complex Neo N3 operations for application development.

## 📚 Examples

### Smart Contract Deployment (`deploy_contract.rs`)
Learn how to deploy smart contracts to Neo N3.

**What you'll learn:**
- Preparing NEF files and manifests
- Calculating deployment costs
- Contract deployment workflow
- Verifying deployment success

### Contract Interaction (`invoke_contract.rs`)
Interact with deployed smart contracts.

**What you'll learn:**
- Reading contract state (test invokes)
- Modifying contract state (invoke transactions)
- Handling contract parameters
- Processing contract events

### NEP-17 Token Operations (`nep17_operations.rs`)
Work with Neo N3 token standards.

**What you'll learn:**
- Querying token information
- Transferring tokens between accounts
- Checking token balances
- Understanding token decimals

### Multi-Signature Wallets (`multi_sig_wallet.rs`)
Create and use multi-signature wallets for enhanced security.

**What you'll learn:**
- Creating multi-sig accounts
- Collecting signatures from multiple parties
- Broadcasting multi-sig transactions
- Security best practices

## 🎯 Prerequisites

Before starting these examples, make sure you've completed:
- [Basic Examples](../basic/) - Fundamental operations
- Understanding of smart contract concepts
- Familiarity with Neo N3 transaction types

## 🔧 Setup

Some intermediate examples require additional setup:

### For Contract Deployment:
- NEF files and manifest files
- Sufficient GAS for deployment fees
- TestNet account with test tokens

### For Multi-Sig Operations:
- Multiple accounts/keys for testing
- Understanding of signature collection process

## 💡 Best Practices

- Always test contract deployment on TestNet first
- Verify contract hash and manifest before deployment
- Use proper error handling for contract interactions
- Implement transaction status monitoring
- Keep private keys secure in multi-sig scenarios

## 🚀 Next Steps

After mastering intermediate examples:
- [Advanced Examples](../advanced/) - Production patterns and DeFi integration
- Deploy your own contracts to TestNet
- Build real applications using these patterns