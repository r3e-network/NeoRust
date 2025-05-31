# Your First Neo Wallet

<div className="hero hero--primary">
  <div className="container">
    <h1 className="hero__title">💼 Create Your First Wallet</h1>
    <p className="hero__subtitle">
      Secure wallet creation and management guide
    </p>
    <p>
      Learn how to safely create, backup, and manage your Neo N3 wallet across all NeoRust interfaces.
    </p>
  </div>
</div>

## 🔐 Security First

Before creating your wallet, understand these critical security concepts:

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🔑 Private Keys</h3>
      </div>
      <div className="card__body">
        <p>Your private key controls your funds. Never share it with anyone.</p>
        <ul>
          <li>✅ Store offline securely</li>
          <li>✅ Use hardware wallets</li>
          <li>❌ Never share online</li>
          <li>❌ Don't store in plain text</li>
        </ul>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>📝 Recovery Phrase</h3>
      </div>
      <div className="card__body">
        <p>12-24 words that can restore your entire wallet.</p>
        <ul>
          <li>✅ Write on paper</li>
          <li>✅ Store in safe place</li>
          <li>❌ Don't take screenshots</li>
          <li>❌ Don't store digitally</li>
        </ul>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🔒 Passwords</h3>
      </div>
      <div className="card__body">
        <p>Strong passwords protect your encrypted wallet files.</p>
        <ul>
          <li>✅ Use 12+ characters</li>
          <li>✅ Mix letters, numbers, symbols</li>
          <li>✅ Use password manager</li>
          <li>❌ Don't reuse passwords</li>
        </ul>
      </div>
    </div>
  </div>
</div>

---

## 🖥️ Desktop GUI Wallet Creation

### **Step 1: Launch the Application**

```bash
# Start the desktop GUI
cd NeoRust/neo-gui
npm run dev
```

Open your browser to `http://localhost:1420`

### **Step 2: Welcome Screen**

![Welcome Screen](../static/img/wallet-welcome.svg)

1. **First Time**: Click "Create New Wallet"
2. **Existing User**: Click "Import Existing Wallet"

### **Step 3: Wallet Setup**

![Wallet Setup](../static/img/wallet-setup.png)

1. **Wallet Name**: Enter a descriptive name
   ```
   Example: "MyMainWallet" or "TradingWallet"
   ```

2. **Strong Password**: Create a secure password
   ```
   Requirements:
   - Minimum 8 characters
   - Mix of uppercase, lowercase
   - Numbers and symbols
   - Not a common password
   ```

3. **Confirm Password**: Re-enter your password

4. **Click "Create Wallet"**

### **Step 4: Recovery Phrase Backup**

![Recovery Phrase](../static/img/wallet-recovery.png)

:::danger Critical Step
Your recovery phrase is the ONLY way to restore your wallet if you lose access. Follow these steps carefully:
:::

1. **Write Down Words**: Copy all 12 words in exact order
2. **Verify Writing**: Double-check each word
3. **Store Securely**: Put paper in a safe place
4. **Test Recovery**: Consider testing recovery on TestNet first

**Example Recovery Phrase:**
```
abandon ability able about above absent absorb abstract absurd abuse access accident
```

5. **Confirm Backup**: Check "I have safely stored my recovery phrase"
6. **Click "Continue"**

### **Step 5: Wallet Created Successfully**

![Wallet Success](../static/img/wallet-success.png)

🎉 **Congratulations!** Your wallet is now created.

**Your wallet includes:**
- **Address**: `NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc`
- **Private Key**: Encrypted and stored securely
- **Recovery Phrase**: Backed up offline

---

## 💻 CLI Wallet Creation

### **Step 1: Create Wallet**

```bash
# Create a new wallet with interactive prompts
neo-cli wallet create --name "MyFirstWallet"

# Or specify all options
neo-cli wallet create \
  --name "MyFirstWallet" \
  --path "./wallets/my-wallet.json" \
  --password-prompt
```

### **Step 2: Interactive Setup**

```
🔐 Creating new wallet: MyFirstWallet

Enter wallet password: ********
Confirm password: ********

✅ Generating secure keys...
✅ Creating wallet file...
✅ Wallet created successfully!

📁 Location: ~/.neorust/wallets/MyFirstWallet.json
🔑 Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc

⚠️  IMPORTANT: Backup your wallet file securely!
```

### **Step 3: Backup Recovery Information**

```bash
# Export recovery phrase
neo-cli wallet export-recovery --wallet "MyFirstWallet"

# Output:
# 🔐 Recovery Phrase for MyFirstWallet:
# abandon ability able about above absent absorb abstract absurd abuse access accident
# 
# ⚠️  Store this phrase securely offline!
# ⚠️  Anyone with this phrase can access your funds!

# Export private key (optional, for advanced users)
neo-cli wallet export-key --address "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc"
```

### **Step 4: Verify Wallet**

```bash
# List all wallets
neo-cli wallet list

# Open wallet
neo-cli wallet open --path "./wallets/MyFirstWallet.json"

# Check wallet info
neo-cli wallet info

# Output:
# 💼 Wallet Information
# ├─ Name: MyFirstWallet
# ├─ Accounts: 1
# ├─ Default Address: NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc
# └─ Status: Unlocked
```

---

## 📚 SDK Wallet Creation

### **Step 1: Basic Wallet Creation**

```rust
use neo3::prelude::*;

fn create_wallet() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new wallet
    let mut wallet = Wallet::new();
    wallet.set_name("MySDKWallet".to_string());
    
    // Create an account
    let account = Account::create()?;
    let address = account.get_address();
    
    println!("Created account: {}", address);
    
    // Add account to wallet
    wallet.add_account(account);
    
    // Save wallet to file
    wallet.save_to_file("./my-wallet.json")?;
    
    println!("Wallet saved successfully!");
    
    Ok(())
}
```

### **Step 2: Secure Wallet with Password**

```rust
use neo3::prelude::*;

fn create_secure_wallet() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = Wallet::new();
    wallet.set_name("SecureWallet".to_string());
    
    // Create account
    let account = Account::create()?;
    wallet.add_account(account);
    
    // Encrypt wallet with password
    let password = "your-secure-password";
    wallet.encrypt_accounts(password);
    
    // Save encrypted wallet
    wallet.save_to_file("./secure-wallet.json")?;
    
    println!("Encrypted wallet created!");
    
    Ok(())
}
```

### **Step 3: Generate Multiple Accounts**

```rust
use neo3::prelude::*;

fn create_multi_account_wallet() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = Wallet::new();
    wallet.set_name("MultiAccountWallet".to_string());
    
    // Create multiple accounts
    for i in 0..5 {
        let account = Account::create()?;
        wallet.add_account(account);
        
        let address = wallet.get_accounts().last().unwrap().get_address();
        println!("Account {}: {}", i + 1, address);
    }
    
    // Set default account
    wallet.set_default_account(0)?;
    
    // Save wallet
    wallet.save_to_file("./multi-wallet.json")?;
    
    Ok(())
}
```

### **Step 4: Load and Use Wallet**

```rust
use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load wallet from file
    let wallet = Wallet::from_file("./my-wallet.json")?;
    
    // Get default account
    let account = wallet.get_default_account()?;
    
    // Connect to network
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    // Check balance
    let neo_hash = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5".parse()?;
    let neo_token = Nep17Contract::new(neo_hash, client);
    
    let balance = neo_token.balance_of(account.get_script_hash()).await?;
    println!("NEO Balance: {}", balance);
    
    Ok(())
}
```

---

## 🔄 Wallet Import and Recovery

### **From Recovery Phrase**

#### **Desktop GUI**
1. **Welcome Screen**: Click "Import Existing Wallet"
2. **Import Method**: Select "Recovery Phrase"
3. **Enter Phrase**: Type your 12-word recovery phrase
4. **Set Password**: Create new password for this device
5. **Import**: Click "Import Wallet"

#### **CLI**
```bash
# Import from recovery phrase
neo-cli wallet import-recovery \
  --phrase "abandon ability able about above absent absorb abstract absurd abuse access accident" \
  --name "RecoveredWallet" \
  --password-prompt
```

#### **SDK**
```rust
use neo3::prelude::*;

fn import_from_recovery() -> Result<(), Box<dyn std::error::Error>> {
    let recovery_phrase = "abandon ability able about above absent absorb abstract absurd abuse access accident";
    
    // Import account from recovery phrase
    let account = Account::from_recovery_phrase(recovery_phrase)?;
    
    // Create wallet
    let mut wallet = Wallet::new();
    wallet.set_name("RecoveredWallet".to_string());
    wallet.add_account(account);
    
    // Save wallet
    wallet.save_to_file("./recovered-wallet.json")?;
    
    Ok(())
}
```

### **From Private Key**

#### **CLI**
```bash
# Import from WIF (Wallet Import Format)
neo-cli wallet import-key \
  --wif "KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr" \
  --label "ImportedAccount"
```

#### **SDK**
```rust
use neo3::prelude::*;

fn import_from_private_key() -> Result<(), Box<dyn std::error::Error>> {
    let wif = "KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr";
    
    // Import account from WIF
    let account = Account::from_wif(wif)?;
    
    // Create wallet
    let mut wallet = Wallet::new();
    wallet.add_account(account);
    
    Ok(())
}
```

### **From Wallet File**

#### **CLI**
```bash
# Import wallet file
neo-cli wallet import-file \
  --file "./backup-wallet.json" \
  --password "original-password"
```

---

## 🌐 Network Configuration

### **Connect to TestNet (Recommended for First Wallet)**

#### **Desktop GUI**
1. **Network Dropdown**: Click network selector (top right)
2. **Select TestNet**: Choose "Neo N3 TestNet"
3. **Wait for Sync**: Application will connect automatically

#### **CLI**
```bash
# Connect to TestNet
neo-cli network connect --network testnet

# Verify connection
neo-cli network status
```

#### **SDK**
```rust
use neo3::prelude::*;

async fn connect_testnet() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.coz.io:443")?;
    let client = RpcClient::new(provider);
    
    let version = client.get_version().await?;
    println!("Connected to: {}", version.useragent);
    
    Ok(())
}
```

---

## 💰 Getting Test Tokens

### **Step 1: Copy Your Address**

Your wallet address looks like: `NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc`

### **Step 2: Visit TestNet Faucet**

Go to: [Neo TestNet Faucet](https://neowish.ngd.network/)

### **Step 3: Request Tokens**

1. **Paste Address**: Enter your wallet address
2. **Select Tokens**: Choose NEO and GAS
3. **Complete Captcha**: Verify you're human
4. **Request**: Click "Request Tokens"

### **Step 4: Verify Receipt**

```bash
# Check balance (CLI)
neo-cli wallet balance

# Or in GUI: Check Dashboard
```

You should see:
- **NEO**: 100.00000000
- **GAS**: 1000.00000000

---

## 🔒 Security Best Practices

### **Backup Strategy**

<div className="row">
  <div className="col col--6">
    <h4>✅ Do This</h4>
    <ul>
      <li>Write recovery phrase on paper</li>
      <li>Store in multiple secure locations</li>
      <li>Use fireproof/waterproof storage</li>
      <li>Test recovery process</li>
      <li>Keep wallet files encrypted</li>
      <li>Use hardware wallets for large amounts</li>
    </ul>
  </div>
  <div className="col col--6">
    <h4>❌ Never Do This</h4>
    <ul>
      <li>Store recovery phrase digitally</li>
      <li>Share private keys</li>
      <li>Use weak passwords</li>
      <li>Store large amounts on hot wallets</li>
      <li>Ignore backup warnings</li>
      <li>Use untrusted wallet software</li>
    </ul>
  </div>
</div>

### **Password Security**

```bash
# Generate secure password (CLI)
neo-cli tools random --length 16 --format base64

# Example output: K7gH9mP2xQ8vN3wR
```

### **Multi-Signature Setup (Advanced)**

```rust
use neo3::prelude::*;

fn create_multisig_wallet() -> Result<(), Box<dyn std::error::Error>> {
    // Create multiple accounts
    let account1 = Account::create()?;
    let account2 = Account::create()?;
    let account3 = Account::create()?;
    
    // Create 2-of-3 multisig account
    let public_keys = vec![
        account1.get_public_key(),
        account2.get_public_key(),
        account3.get_public_key(),
    ];
    
    let multisig_account = Account::create_multisig(2, public_keys)?;
    
    println!("Multisig address: {}", multisig_account.get_address());
    
    Ok(())
}
```

---

## 🎯 Next Steps

<div className="row">
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>💸 Send Transactions</h3>
      </div>
      <div className="card__body">
        <p>Learn how to send NEO, GAS, and other tokens safely.</p>
      </div>
      <div className="card__footer">
        <a href="../gui/wallet-management" className="button button--primary">Send Tokens →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🎨 Explore NFTs</h3>
      </div>
      <div className="card__body">
        <p>Discover how to mint, buy, and manage NFTs on Neo N3.</p>
      </div>
      <div className="card__footer">
        <a href="../gui/nft-operations" className="button button--primary">Try NFTs →</a>
      </div>
    </div>
  </div>
  
  <div className="col col--4">
    <div className="card">
      <div className="card__header">
        <h3>🔧 Developer Tools</h3>
      </div>
      <div className="card__body">
        <p>Use built-in tools for encoding, hashing, and debugging.</p>
      </div>
      <div className="card__footer">
        <a href="../gui/developer-tools" className="button button--primary">Use Tools →</a>
      </div>
    </div>
  </div>
</div>

### **Need Help?**

- **Security Questions**: [Security Best Practices](../advanced/security)
- **Technical Issues**: [Troubleshooting Guide](../cli/troubleshooting)
- **Community Support**: [GitHub Discussions](https://github.com/R3E-Network/NeoRust/discussions)

---

**Your wallet is ready! Start exploring the Neo N3 ecosystem! 🚀** 