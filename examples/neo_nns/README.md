# Neo N3 Name Service (NNS) Examples

This directory contains examples demonstrating how to interact with the Neo Name Service (NNS) on the Neo N3 blockchain using modern NeoRust SDK patterns.

## ✅ Production-Ready Examples

All examples have been updated to use modern NeoRust SDK APIs and compile successfully.

### Available Examples

| Example | Description | Status |
|---------|-------------|--------|
| **nns_operations.rs** | NNS domain operations and concepts | ✅ Working |

## 🚀 Quick Start

### Prerequisites

1. **Rust Environment**: Ensure you have Rust 1.70+ installed
2. **Neo N3 TestNet**: Examples connect to Neo N3 TestNet by default
3. **Dependencies**: All required dependencies are included in the workspace

### Running Examples

```bash
# Navigate to the NNS directory
cd examples/neo_nns

# Run the NNS operations example
cargo run --example nns_operations

# Check compilation
cargo check --example nns_operations
```

## 📋 NNS Overview

### **What is Neo Name Service (NNS)?**

Neo Name Service (NNS) is a decentralized domain name system built on the Neo N3 blockchain that:

- **Maps Human-Readable Names**: Converts complex blockchain addresses to simple names like `alice.neo`
- **Supports Multiple Records**: Handles various record types (A, TXT, CNAME, MX, SRV)
- **Enables Easy Transfers**: Simplifies sending transactions to memorable names
- **Provides Ownership Control**: Domain owners have full control over their domains

### **Key Features**

- 🏷️ **Domain Registration**: Register `.neo` domains on the blockchain
- 📋 **Record Management**: Set and update DNS-like records
- 🔄 **Domain Renewal**: Extend domain ownership periods
- 🔍 **Name Resolution**: Convert domain names to blockchain addresses
- 💰 **Ownership Transfer**: Transfer domain ownership to other addresses

## 🔧 Modern API Patterns

The NNS example follows these modern NeoRust SDK patterns:

### **Standard Imports**
```rust
use neo3::prelude::*;
use neo3::neo_clients::APITrait;
use std::str::FromStr;
```

### **Provider Setup**
```rust
let provider = providers::HttpProvider::new("https://testnet1.neo.org:443/")?;
let client = providers::RpcClient::new(provider);
```

### **Conceptual Implementation**
The current example demonstrates NNS concepts and workflow patterns rather than direct contract interaction, as the specific NNS contract APIs are being modernized.

## 🏗️ NNS Operations

### **Domain Registration Process**

1. **Check Availability**: Verify if a domain name is available
2. **Calculate Fees**: Determine registration costs in GAS
3. **Prepare Transaction**: Create registration transaction
4. **Sign & Submit**: Sign with owner wallet and broadcast
5. **Confirmation**: Wait for blockchain confirmation

### **Record Management Types**

| Record Type | Purpose | Example |
|-------------|---------|---------|
| **A** | Points to IPv4 address | `192.168.1.1` |
| **TXT** | Stores text information | `"Hello, Neo N3!"` |
| **CNAME** | Alias to another domain | `www.example.neo` |
| **MX** | Mail server information | `mail.example.neo` |
| **SRV** | Service location data | `_service._tcp.example.neo` |

### **Domain Lifecycle**

```
Registration → Active Use → Renewal → Transfer/Expiry
     ↓             ↓           ↓            ↓
   Pay fees    Set records  Extend time   Change owner
```

## 🧪 Testing

### **Compilation Tests**
```bash
# Test example compiles successfully
cargo check --example nns_operations
```

### **Runtime Tests**
```bash
# Run NNS operations demonstration
cargo run --example nns_operations
```

## 🔗 Network Configuration

Examples default to Neo N3 TestNet:
- **TestNet RPC**: `https://testnet1.neo.org:443/`
- **NNS Contract**: `0x50ac1c37690cc2cfc594472833cf57505d5f46de`

For MainNet usage:
- **MainNet RPC**: `https://mainnet1.neo.org:443/`
- **NNS Contract**: Same hash on MainNet

## 💡 Best Practices

### **Domain Management**
- 🔐 **Security**: Use hardware wallets for valuable domains
- ⏰ **Monitoring**: Track domain expiration dates
- 💰 **Budget**: Plan for renewal costs
- 📋 **Records**: Keep DNS records updated
- 🔄 **Backups**: Maintain backup access methods

### **Development Workflow**
1. **Test First**: Always test operations on TestNet
2. **Validate Names**: Check domain name format and availability
3. **Fee Planning**: Calculate total costs including network fees
4. **Error Handling**: Implement comprehensive error management
5. **User Experience**: Provide clear feedback during operations

## 🛡️ Security Considerations

### **Domain Security**
- **Private Key Safety**: Never expose domain owner private keys
- **Multi-Signature**: Consider multi-sig wallets for valuable domains
- **Record Validation**: Verify record data before setting
- **Transfer Caution**: Double-check recipient addresses for transfers

### **Smart Contract Interaction**
- **Contract Verification**: Always verify NNS contract addresses
- **Transaction Review**: Review all transaction details before signing
- **Gas Limits**: Set appropriate gas limits for operations
- **Network Confirmation**: Wait for sufficient confirmations

## 📚 Additional Resources

### **Neo N3 Documentation**
- [Neo Name Service Overview](https://docs.neo.org/)
- [Domain Registration Guide](https://docs.neo.org/)
- [DNS Record Types](https://docs.neo.org/)

### **NeoRust SDK**
- [SDK Documentation](../../README.md)
- [Contract Interaction Examples](../neo_smart_contracts/)
- [Transaction Examples](../neo_transactions/)

### **Community Resources**
- [NNS Community](https://neo.org/nns)
- [Domain Marketplace](https://neons.name/)
- [Registration Tools](https://neotube.io/nns)

## 🤝 Contributing

To improve NNS examples:

1. **Modern Patterns**: Use current NeoRust SDK APIs
2. **Clear Documentation**: Explain NNS concepts thoroughly
3. **Error Handling**: Implement robust error management
4. **Testing**: Verify examples work on TestNet
5. **Security**: Follow domain management best practices

## ⚠️ Important Notes

- **Educational Purpose**: Current examples demonstrate concepts and workflows
- **API Evolution**: Direct NNS contract interaction APIs are being modernized
- **TestNet First**: Always test domain operations on TestNet
- **Cost Awareness**: Domain registration and renewal require GAS fees
- **Ownership**: Domain ownership is permanent until transferred or expired

---

**Status**: ✅ Compiling successfully with modern NeoRust SDK (Last updated: December 2024) 