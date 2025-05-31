# Developer Tools

<div className="hero hero--primary">
  <div className="container">
    <h1 className="hero__title">🔧 Developer Tools</h1>
    <p className="hero__subtitle">
      Built-in utilities for blockchain development
    </p>
    <p>
      Powerful development tools integrated directly into the NeoRust Desktop GUI for efficient blockchain development.
    </p>
  </div>
</div>

## 🛠️ Tools Overview

The NeoRust GUI includes a comprehensive suite of developer tools designed to streamline blockchain development and debugging.

![Developer Tools](../static/img/developer-tools.svg)

### **Tool Categories**

<div className="row">
  <div className="col col--3">
    <div className="card">
      <div className="card__header">
        <h3>🔤 Encoding</h3>
      </div>
      <div className="card__body">
        <ul>
          <li>Base64 encoding/decoding</li>
          <li>Hex conversion</li>
          <li>Base58 operations</li>
          <li>URL encoding</li>
        </ul>
      </div>
    </div>
  </div>
  
  <div className="col col--3">
    <div className="card">
      <div className="card__header">
        <h3>🔐 Hashing</h3>
      </div>
      <div className="card__body">
        <ul>
          <li>SHA256 hashing</li>
          <li>RIPEMD160</li>
          <li>Keccak256</li>
          <li>Custom hash functions</li>
        </ul>
      </div>
    </div>
  </div>
  
  <div className="col col--3">
    <div className="card">
      <div className="card__header">
        <h3>🔑 Cryptography</h3>
      </div>
      <div className="card__body">
        <ul>
          <li>Key generation</li>
          <li>Digital signatures</li>
          <li>Address derivation</li>
          <li>Encryption/decryption</li>
        </ul>
      </div>
    </div>
  </div>
  
  <div className="col col--3">
    <div className="card">
      <div className="card__header">
        <h3>📡 Network</h3>
      </div>
      <div className="card__body">
        <ul>
          <li>RPC testing</li>
          <li>Transaction builder</li>
          <li>Contract interaction</li>
          <li>Network debugging</li>
        </ul>
      </div>
    </div>
  </div>
</div>

---

## 🔤 Encoding & Decoding Tools

### **Base64 Encoder/Decoder**

![Base64 Tool](../static/img/base64-tool.png)

#### **Features**
- **Bidirectional Conversion**: Encode and decode in real-time
- **File Support**: Encode files to Base64 strings
- **Validation**: Automatic format validation
- **Copy to Clipboard**: One-click copying

#### **Usage Examples**
```
Input: "Hello, Neo N3!"
Base64: SGVsbG8sIE5lbyBOMyE=

Input: SGVsbG8sIE5lbyBOMyE=
Decoded: "Hello, Neo N3!"
```

### **Hex Converter**

#### **String to Hex**
```
Input: "NeoRust"
Hex: 4e656f52757374
Length: 14 characters (7 bytes)
```

#### **Hex to String**
```
Input: 4e656f52757374
String: "NeoRust"
Encoding: UTF-8
```

#### **Advanced Hex Operations**
- **Byte Array Display**: Visual byte representation
- **Endianness Toggle**: Little/big endian conversion
- **Checksum Calculation**: Automatic integrity checks
- **Hex Dump View**: Traditional hex dump format

### **Base58 Operations**

#### **Base58 Encoding**
```
Input: "Neo N3 Address"
Base58: 2NEpo7TZRRrLZSi2U
Checksum: Included
```

#### **Base58Check**
```
Input: Raw bytes + checksum
Base58Check: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
Validation: ✅ Valid Neo address
```

### **URL Encoding**

#### **URL Encode/Decode**
```
Input: "Hello World & More!"
URL Encoded: Hello%20World%20%26%20More%21

Input: Hello%20World%20%26%20More%21
URL Decoded: "Hello World & More!"
```

---

## 🔐 Cryptographic Hash Functions

### **SHA256 Hashing**

![Hash Tool](../static/img/hash-tool.png)

#### **Text Hashing**
```
Input: "NeoRust is awesome!"
SHA256: a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
Length: 64 characters (32 bytes)
```

#### **File Hashing**
```
File: contract.nef (1.2 KB)
SHA256: 7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730
Verification: ✅ Integrity confirmed
```

### **RIPEMD160 Hashing**

#### **Address Generation**
```
Public Key: 03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816
SHA256: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
RIPEMD160: abcdef1234567890abcdef1234567890abcdef12
Neo Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
```

### **Keccak256 (Ethereum Compatible)**

#### **Ethereum Address Derivation**
```
Public Key: 04b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816...
Keccak256: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
ETH Address: 0x1234567890abcdef1234567890abcdef12345678
```

### **Custom Hash Functions**

#### **Multiple Hash Algorithms**
- **MD5**: Legacy support (not recommended for security)
- **SHA1**: Legacy support
- **SHA3**: Modern secure hashing
- **Blake2b**: High-performance hashing
- **Whirlpool**: Alternative secure hash

---

## 🔑 Cryptographic Operations

### **Key Generation Tools**

![Key Generation](../static/img/key-generation.png)

#### **Private Key Generation**
```
Entropy Source: Cryptographically secure random
Private Key: KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr (WIF)
Hex Format: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
Validation: ✅ Valid secp256r1 key
```

#### **Public Key Derivation**
```
Private Key: KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr
Public Key: 03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816
Compressed: ✅ Yes (33 bytes)
Uncompressed: 04b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816...
```

#### **Address Generation**
```
Public Key: 03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816
Script Hash: 1234567890abcdef1234567890abcdef12345678
Neo Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
Address Type: Standard (single signature)
```

### **Digital Signature Tools**

#### **Message Signing**
```
Message: "I own this Neo address"
Private Key: [Hidden for security]
Signature: 3045022100a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456022012345678901234567890abcdef1234567890abcdef1234567890abcdef123456
Format: DER encoded
```

#### **Signature Verification**
```
Message: "I own this Neo address"
Public Key: 03b4af8d061b6b320cce6c63bc4ec7894dce107bfc5f5ef5c68a93b4ad1e136816
Signature: 3045022100a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456...
Verification: ✅ Valid signature
```

### **Encryption/Decryption**

#### **AES Encryption**
```
Plaintext: "Secret message for Neo N3"
Key: 256-bit AES key (auto-generated)
IV: Random initialization vector
Ciphertext: U2FsdGVkX1+1234567890abcdef1234567890abcdef1234567890abcdef123456
Mode: AES-256-CBC
```

#### **RSA Operations**
```
Key Size: 2048 bits
Public Key: -----BEGIN PUBLIC KEY-----...
Private Key: -----BEGIN PRIVATE KEY-----...
Encryption: PKCS#1 v1.5 padding
Max Message: 245 bytes
```

---

## 📡 Network & RPC Tools

### **RPC Client Tester**

![RPC Tester](../static/img/rpc-tester.png)

#### **Connection Testing**
```
Endpoint: https://testnet1.neo.coz.io:443
Method: getversion
Parameters: []
Response Time: 142ms
Status: ✅ Connected

Response:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tcpport": 20333,
    "wsport": 20334,
    "nonce": 1234567890,
    "useragent": "/Neo:3.6.0/"
  }
}
```

#### **Custom RPC Calls**
```
Method: getblockcount
Parameters: []
Expected Response: Integer (block height)

Method: getblock
Parameters: [1000000, 1]
Expected Response: Block object with transactions
```

#### **Batch RPC Requests**
```
Batch Request:
[
  {"method": "getblockcount", "params": []},
  {"method": "getversion", "params": []},
  {"method": "getpeers", "params": []}
]

Response Time: 89ms (parallel execution)
Success Rate: 100% (3/3 requests)
```

### **Transaction Builder**

![Transaction Builder](../static/img/transaction-builder.png)

#### **Visual Transaction Construction**
```
Transaction Builder:
├─ Version: 0
├─ Nonce: 1234567890 (random)
├─ System Fee: 1000000 (0.01 GAS)
├─ Network Fee: 1234567 (0.01234567 GAS)
├─ Valid Until Block: 2345678
├─ Signers: [NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc]
└─ Script: [Contract invocation bytecode]
```

#### **Script Builder**
```
Script Operations:
├─ PUSHDATA1: "transfer"
├─ PUSHDATA1: from_address_hash
├─ PUSHDATA1: to_address_hash
├─ PUSHDATA1: amount
├─ PUSH4: 4 (parameter count)
├─ PACK: Create array
├─ PUSHDATA1: contract_hash
└─ SYSCALL: System.Contract.Call
```

#### **Transaction Simulation**
```
Simulation Results:
├─ Gas Consumed: 9977780 (0.09977780 GAS)
├─ VM State: HALT
├─ Stack Result: [{"type": "Boolean", "value": true}]
├─ Notifications: 1 event emitted
├─ Storage Changes: 2 items modified
└─ Execution Time: 1.2ms
```

### **Contract Interaction Tools**

#### **Contract Inspector**
```
Contract: 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5 (NEO Token)
├─ Name: NeoToken
├─ Symbol: NEO
├─ Decimals: 0
├─ Total Supply: 100,000,000
├─ Methods: 15 public methods
├─ Events: 2 event types
└─ Storage: 1,247 items
```

#### **Method Invocation**
```
Contract: NEO Token
Method: balanceOf
Parameters:
├─ account: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc (Hash160)

Invocation Result:
├─ Return Type: Integer
├─ Value: 100000000000 (1000.00000000 NEO)
├─ Gas Cost: 0.0031 GAS
└─ Execution: Successful
```

#### **Event Log Viewer**
```
Recent Events:
├─ Transfer (NEO): 5.0 NEO from NX8... to NY9...
├─ Transfer (GAS): 100.0 GAS from NZ1... to NA2...
├─ Mint (NFT): Token #1337 to NB3...
└─ Burn (Token): 1000 FLUND from NC4...

Filter Options:
├─ Contract: All / Specific contract
├─ Event Type: All / Transfer / Mint / Burn
├─ Time Range: Last 24h / 7d / 30d
└─ Address: All / Specific address
```

---

## 🔍 Debugging & Analysis Tools

### **Transaction Analyzer**

![Transaction Analyzer](../static/img/transaction-analyzer.png)

#### **Transaction Breakdown**
```
Transaction: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
├─ Block: 1,234,567 (confirmed)
├─ Size: 456 bytes
├─ Network Fee: 0.00123456 GAS
├─ System Fee: 0.01000000 GAS
├─ Signers: 1 account
├─ Witnesses: 1 signature
└─ Script Length: 234 bytes

Execution Details:
├─ VM State: HALT (successful)
├─ Gas Consumed: 9,977,780
├─ Stack Items: 1 result
├─ Notifications: 2 events
└─ Storage Changes: 3 modifications
```

#### **Script Disassembly**
```
OpCode Breakdown:
0000: PUSHDATA1 08 "transfer"
0009: PUSHDATA1 14 from_address_hash
001F: PUSHDATA1 14 to_address_hash
0035: PUSHDATA1 08 amount_bytes
003F: PUSH4
0040: PACK
0041: PUSHDATA1 14 contract_hash
0057: SYSCALL System.Contract.Call
005C: RET
```

### **Network Monitor**

#### **Real-time Network Stats**
```
Network Health:
├─ Block Height: 2,345,678
├─ Block Time: 15.2s (average)
├─ TPS: 12.7 transactions/second
├─ Memory Pool: 156 pending transactions
├─ Connected Peers: 42 nodes
├─ Network Hash Rate: 1.2 TH/s
└─ Consensus: 7/7 nodes participating

Performance Metrics:
├─ Latency: 89ms (to nearest node)
├─ Sync Status: 100% synchronized
├─ Data Usage: 2.1 MB/hour
└─ Uptime: 99.97% (last 30 days)
```

#### **Peer Analysis**
```
Connected Peers:
├─ 203.0.113.1:20333 (Consensus Node)
├─ 198.51.100.2:20333 (Full Node)
├─ 192.0.2.3:20333 (RPC Node)
└─ ... (39 more peers)

Peer Statistics:
├─ Average Latency: 125ms
├─ Fastest Peer: 45ms
├─ Slowest Peer: 890ms
├─ Geographic Distribution: 15 countries
└─ Version Distribution: 95% latest version
```

### **Performance Profiler**

#### **Gas Usage Analysis**
```
Gas Consumption Breakdown:
├─ Contract Calls: 65% (6.5M gas)
├─ Storage Operations: 20% (2.0M gas)
├─ Cryptographic Operations: 10% (1.0M gas)
├─ System Calls: 3% (0.3M gas)
└─ Other Operations: 2% (0.2M gas)

Optimization Suggestions:
├─ Reduce storage writes: -15% gas
├─ Batch operations: -8% gas
├─ Optimize contract calls: -12% gas
└─ Use efficient data types: -5% gas
```

---

## 🧪 Testing & Simulation

### **Smart Contract Tester**

![Contract Tester](../static/img/contract-tester.png)

#### **Unit Testing Framework**
```
Test Suite: NEP-17 Token Contract
├─ Test 1: Deploy Contract ✅ Passed
├─ Test 2: Initial Supply ✅ Passed
├─ Test 3: Transfer Tokens ✅ Passed
├─ Test 4: Approve Allowance ✅ Passed
├─ Test 5: Transfer From ✅ Passed
├─ Test 6: Burn Tokens ✅ Passed
└─ Test 7: Mint Tokens ❌ Failed

Results: 6/7 tests passed (85.7%)
Coverage: 92% of contract methods
Execution Time: 2.3 seconds
```

#### **Scenario Testing**
```
Scenario: Multi-signature Wallet
├─ Setup: 3-of-5 multisig wallet
├─ Step 1: Create proposal ✅
├─ Step 2: First signature ✅
├─ Step 3: Second signature ✅
├─ Step 4: Third signature ✅
├─ Step 5: Execute transaction ✅
└─ Cleanup: Reset state ✅

Result: All steps completed successfully
Gas Used: 15.7M (estimated)
Time: 45 seconds (simulated)
```

### **Load Testing**

#### **Transaction Stress Test**
```
Load Test Configuration:
├─ Concurrent Users: 100
├─ Transactions per User: 10
├─ Test Duration: 60 seconds
├─ Transaction Type: NEP-17 transfers
└─ Network: TestNet

Results:
├─ Total Transactions: 1,000
├─ Successful: 987 (98.7%)
├─ Failed: 13 (1.3%)
├─ Average Response Time: 2.1s
├─ Peak TPS: 16.7
└─ Error Rate: 1.3%
```

---

## 📊 Data Analysis Tools

### **Blockchain Explorer Integration**

![Explorer Integration](../static/img/explorer-integration.png)

#### **Address Analytics**
```
Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
├─ Balance: 150.0 NEO, 2,500.0 GAS
├─ Transaction Count: 247
├─ First Activity: 2024-01-15
├─ Last Activity: 2024-05-31
├─ Unique Counterparties: 89
├─ Average Transaction: 12.5 NEO
└─ Largest Transaction: 500.0 NEO

Activity Pattern:
├─ Most Active Day: Monday
├─ Peak Hour: 14:00-15:00 UTC
├─ Transaction Frequency: 2.1/day
└─ Growth Rate: +15% monthly
```

#### **Contract Analytics**
```
Contract: 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
├─ Total Calls: 1,234,567
├─ Unique Users: 45,678
├─ Daily Active Users: 1,234
├─ Gas Consumption: 12.5M GAS total
├─ Success Rate: 99.2%
├─ Average Gas per Call: 10,123
└─ Peak Usage: 15:00 UTC daily

Popular Methods:
├─ transfer: 45% of calls
├─ balanceOf: 30% of calls
├─ approve: 15% of calls
└─ allowance: 10% of calls
```

### **Market Data Integration**

#### **Price Analysis**
```
NEO Price Analysis:
├─ Current Price: $25.67 USD
├─ 24h Change: +5.2% (+$1.27)
├─ 7d Change: +12.8% (+$2.91)
├─ Market Cap: $2.567B USD
├─ Volume (24h): $125.7M USD
├─ All-time High: $196.85 (Jan 2018)
└─ All-time Low: $0.036 (Oct 2016)

Technical Indicators:
├─ RSI (14): 67.3 (Bullish)
├─ MACD: Positive divergence
├─ Moving Averages: Above 50-day MA
└─ Support/Resistance: $24.50 / $28.00
```

---

## ⚙️ Configuration & Settings

### **Tool Preferences**

#### **Default Settings**
```
Encoding Preferences:
├─ Default Format: UTF-8
├─ Line Endings: LF (Unix)
├─ Byte Order: Little Endian
└─ Character Encoding: Unicode

Hash Preferences:
├─ Default Algorithm: SHA256
├─ Output Format: Lowercase hex
├─ Include Checksums: Yes
└─ Verify Integrity: Always

Network Preferences:
├─ Default RPC: TestNet
├─ Timeout: 30 seconds
├─ Retry Attempts: 3
└─ Connection Pool: 5 connections
```

#### **Custom Shortcuts**
```
Keyboard Shortcuts:
├─ Ctrl+E: Encode selection
├─ Ctrl+D: Decode selection
├─ Ctrl+H: Hash selection
├─ Ctrl+T: Test RPC call
├─ Ctrl+B: Build transaction
├─ F5: Refresh data
└─ F12: Open developer console
```

### **Export & Import**

#### **Tool Configuration Export**
```json
{
  "version": "1.0",
  "preferences": {
    "encoding": {
      "defaultFormat": "UTF-8",
      "outputCase": "lowercase"
    },
    "network": {
      "defaultEndpoint": "https://testnet1.neo.coz.io:443",
      "timeout": 30000
    },
    "ui": {
      "theme": "dark",
      "fontSize": 14
    }
  },
  "customEndpoints": [
    {
      "name": "Local Node",
      "url": "http://localhost:20332"
    }
  ]
}
```

---

## 🎯 Best Practices

### **Development Workflow**

<div className="row">
  <div className="col col--6">
    <h4>✅ Recommended Practices</h4>
    <ul>
      <li>Test on TestNet before MainNet</li>
      <li>Validate all inputs and outputs</li>
      <li>Use proper error handling</li>
      <li>Monitor gas consumption</li>
      <li>Keep private keys secure</li>
      <li>Document your testing process</li>
    </ul>
  </div>
  <div className="col col--6">
    <h4>❌ Common Pitfalls</h4>
    <ul>
      <li>Skipping input validation</li>
      <li>Hardcoding sensitive data</li>
      <li>Ignoring gas limits</li>
      <li>Not testing edge cases</li>
      <li>Using deprecated methods</li>
      <li>Poor error handling</li>
    </ul>
  </div>
</div>

### **Security Guidelines**

#### **Key Management**
- **Never expose private keys** in tools or logs
- **Use secure random generation** for all cryptographic operations
- **Validate all cryptographic inputs** before processing
- **Clear sensitive data** from memory after use

#### **Network Security**
- **Verify RPC endpoints** before connecting
- **Use HTTPS/TLS** for all network communications
- **Validate all responses** from external services
- **Implement proper timeout handling**

---

## 🆘 Troubleshooting

### **Common Issues**

#### **Encoding Problems**
```
Error: Invalid Base64 input
Solution:
├─ Check for invalid characters
├─ Verify padding (= characters)
├─ Ensure proper line breaks
└─ Try different encoding formats
```

#### **Network Connectivity**
```
Error: RPC endpoint not responding
Solution:
├─ Check internet connection
├─ Verify endpoint URL
├─ Try alternative endpoints
├─ Check firewall settings
└─ Increase timeout values
```

#### **Transaction Building**
```
Error: Insufficient funds for transaction
Solution:
├─ Check account balance
├─ Verify gas calculations
├─ Reduce transaction amount
├─ Use different fee settings
└─ Check network congestion
```

### **Getting Help**

- **📚 Documentation**: [Developer guide](https://neorust.netlify.app/gui/developer-tools)
- **💬 Community**: [Developer Discord](https://discord.gg/neorust-dev)
- **🐛 Bug Reports**: [GitHub issues](https://github.com/R3E-Network/NeoRust/issues)
- **📧 Support**: [Technical support](mailto:dev-support@neorust.io)

---

**Build with confidence using NeoRust's powerful developer tools! 🚀** 