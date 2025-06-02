import React, { useState } from 'react';
import Layout from '@theme/Layout';
import CodeBlock from '@theme/CodeBlock';
import Link from '@docusaurus/Link';
import clsx from 'clsx';
import styles from './examples.module.css';

// Example categories
const categories = [
  {
    id: 'basic',
    title: 'Basic Operations',
    description: 'Essential operations like connecting to the network, creating accounts, and checking balances.',
    icon: '🚀',
  },
  {
    id: 'wallet',
    title: 'Wallet Management',
    description: 'Create, load, and manage Neo wallets with the NEP-6 standard.',
    icon: '💰',
  },
  {
    id: 'tokens',
    title: 'Token Operations',
    description: 'Work with NEP-17 tokens, transfers, and balance checking.',
    icon: '🪙',
  },
  {
    id: 'contracts',
    title: 'Smart Contracts',
    description: 'Deploy and interact with smart contracts on Neo N3.',
    icon: '📋',
  },
  {
    id: 'advanced',
    title: 'Advanced Patterns',
    description: 'Advanced techniques including batch operations and multi-sig transactions.',
    icon: '🎯',
  },
  {
    id: 'neo-x',
    title: 'Neo X Integration',
    description: 'Cross-chain operations and EVM compatibility features.',
    icon: '🔗',
  },
];

// Example data
const examples = {
  basic: [
    {
      title: 'Connect to Neo Network',
      description: 'Establish connection to Neo N3 network and get blockchain info.',
      tags: ['connection', 'rpc', 'basics'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Neo N3 TestNet
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Get blockchain information
    let block_count = client.get_block_count().await?;
    let version = client.get_version().await?;
    
    println!("Current block height: {}", block_count);
    println!("Node version: {}", version.useragent);
    println!("Network: {}", version.network);
    
    Ok(())
}`
    },
    {
      title: 'Create New Account',
      description: 'Generate a new account with private key and address.',
      tags: ['account', 'keys', 'address'],
      code: `use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new account
    let account = Account::create()?;
    
    println!("Address: {}", account.get_address());
    println!("Public Key: {}", hex::encode(account.get_public_key().encode_point(true)));
    println!("Script Hash: {}", account.get_script_hash());
    
    // Export private key (be careful with this!)
    let private_key = account.get_private_key();
    println!("Private Key: {}", hex::encode(private_key.as_bytes()));
    
    Ok(())
}`
    },
    {
      title: 'Check Account Balance',
      description: 'Check GAS and NEO balances for any account.',
      tags: ['balance', 'gas', 'neo'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let address = "NXXXXxxxXXXxxxXXXxxxXXXxxxXXXxxx";
    let script_hash = address.to_script_hash()?;
    
    // Check GAS balance
    let gas_token = GasToken::new(&client);
    let gas_balance = gas_token.balance_of(&script_hash).await?;
    let gas_decimals = gas_token.decimals().await?;
    println!("GAS Balance: {} ({})", gas_balance, gas_decimals);
    
    // Check NEO balance  
    let neo_token = NeoToken::new(&client);
    let neo_balance = neo_token.balance_of(&script_hash).await?;
    println!("NEO Balance: {}", neo_balance);
    
    Ok(())
}`
    },
  ],
  wallet: [
    {
      title: 'Create NEP-6 Wallet',
      description: 'Create a new wallet following NEP-6 standard.',
      tags: ['nep6', 'wallet', 'json'],
      code: `use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new wallet
    let mut wallet = Wallet::new();
    wallet.set_name("MyWallet".to_string());
    
    // Add accounts to wallet
    let account1 = Account::create()?;
    let account2 = Account::create()?;
    
    wallet.add_account(account1);
    wallet.add_account(account2);
    
    // Set default account
    if let Some(first_account) = wallet.get_accounts().first() {
        wallet.set_default_account(first_account.get_script_hash());
    }
    
    println!("Wallet '{}' created with {} accounts", 
             wallet.get_name(), 
             wallet.get_accounts().len());
    
    // Save to NEP-6 format
    let password = "secure_password";
    let nep6_wallet = wallet.to_nep6(password)?;
    
    let json = serde_json::to_string_pretty(&nep6_wallet)?;
    std::fs::write("wallet.json", json)?;
    
    Ok(())
}`
    },
    {
      title: 'Load Existing Wallet',
      description: 'Load and decrypt an existing NEP-6 wallet.',
      tags: ['nep6', 'load', 'decrypt'],
      code: `use neo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load wallet from file
    let wallet_json = std::fs::read_to_string("wallet.json")?;
    let nep6_wallet: Nep6Wallet = serde_json::from_str(&wallet_json)?;
    
    println!("Loaded wallet: {}", nep6_wallet.name);
    println!("Version: {}", nep6_wallet.version);
    println!("Accounts: {}", nep6_wallet.accounts.len());
    
    // Decrypt wallet with password
    let password = "secure_password";
    let wallet = Wallet::from_nep6(&nep6_wallet, password)?;
    
    // List all accounts
    for account in wallet.get_accounts() {
        println!("Account: {}", account.get_address());
        if let Some(label) = account.get_label() {
            println!("  Label: {}", label);
        }
    }
    
    Ok(())
}`
    },
  ],
  tokens: [
    {
      title: 'Transfer GAS Tokens',
      description: 'Transfer GAS tokens between accounts.',
      tags: ['transfer', 'gas', 'transaction'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Load sender account
    let sender = Account::from_private_key("your_private_key_here")?;
    let signer = AccountSigner::new(sender);
    
    // Prepare transfer
    let gas_token = GasToken::new(&client);
    let recipient = "NRecipientAddressHere".to_script_hash()?;
    let amount = 1_000_000_000; // 10 GAS (8 decimals)
    
    // Send transfer transaction
    let tx_hash = gas_token
        .transfer(&signer, &recipient, amount, None)
        .await?;
    
    println!("Transfer transaction sent: {}", tx_hash);
    println!("Monitor at: https://testnet.neotube.io/transaction/{}", tx_hash);
    
    Ok(())
}`
    },
    {
      title: 'Custom NEP-17 Token',
      description: 'Interact with custom NEP-17 tokens.',
      tags: ['nep17', 'custom', 'token'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Custom token contract hash
    let token_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    let token = Nep17Token::new(&client, token_hash);
    
    // Get token info
    let symbol = token.symbol().await?;
    let decimals = token.decimals().await?;
    let total_supply = token.total_supply().await?;
    
    println!("Token Symbol: {}", symbol);
    println!("Decimals: {}", decimals);
    println!("Total Supply: {}", total_supply);
    
    // Check balance
    let address = "NAddressHere".to_script_hash()?;
    let balance = token.balance_of(&address).await?;
    println!("Balance: {} {}", balance, symbol);
    
    Ok(())
}`
    },
  ],
  contracts: [
    {
      title: 'Invoke Contract Method',
      description: 'Call smart contract methods with parameters.',
      tags: ['contract', 'invoke', 'parameters'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    
    // Read-only contract call
    let result = client
        .invoke_function(
            &contract_hash,
            "getValue",
            vec![ContractParameter::String("key1".to_string())],
            vec![], // No signers for read-only
        )
        .await?;
    
    println!("Contract Result:");
    println!("State: {}", result.state);
    println!("Gas Consumed: {}", result.gas_consumed);
    
    if let Some(stack) = result.stack {
        for item in stack {
            println!("Stack Item: {:?}", item);
        }
    }
    
    Ok(())
}`
    },
    {
      title: 'State-Changing Contract Call',
      description: 'Execute contract methods that modify blockchain state.',
      tags: ['contract', 'state', 'transaction'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Load account for signing
    let account = Account::from_private_key("your_private_key")?;
    let signer = AccountSigner::new(account);
    
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    
    // State-changing contract call
    let tx_hash = client
        .invoke_function_tx(
            &signer,
            &contract_hash,
            "setValue",
            vec![
                ContractParameter::String("key1".to_string()),
                ContractParameter::String("new_value".to_string()),
            ],
            None, // Use default fees
        )
        .await?;
    
    println!("Transaction sent: {}", tx_hash);
    println!("View at: https://testnet.neotube.io/transaction/{}", tx_hash);
    
    Ok(())
}`
    },
  ],
  advanced: [
    {
      title: 'Multi-Operation Transaction',
      description: 'Combine multiple operations in a single transaction.',
      tags: ['multi-op', 'batch', 'transaction'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    let account = Account::from_private_key("your_private_key")?;
    let signer = AccountSigner::new(account);
    
    // Create transaction builder
    let mut tx_builder = TransactionBuilder::new(&client);
    
    // Add multiple operations
    let gas_token = GasToken::new(&client);
    let recipient1 = "NAddress1".to_script_hash()?;
    let recipient2 = "NAddress2".to_script_hash()?;
    
    // Add transfers
    tx_builder.add_transfer(&gas_token.script_hash(), &recipient1, 1_000_000_000)?;
    tx_builder.add_transfer(&gas_token.script_hash(), &recipient2, 2_000_000_000)?;
    
    // Add contract call
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    tx_builder.add_contract_call(
        &contract_hash,
        "updateValue",
        vec![ContractParameter::String("batch_update".to_string())],
    )?;
    
    // Build and send transaction
    let tx = tx_builder.build_and_sign(&signer).await?;
    let tx_hash = client.send_raw_transaction(&tx).await?;
    
    println!("Multi-operation transaction: {}", tx_hash);
    
    Ok(())
}`
    },
  ],
  'neo-x': [
    {
      title: 'Cross-Chain Bridge',
      description: 'Bridge assets between Neo N3 and Neo X networks.',
      tags: ['bridge', 'cross-chain', 'neo-x'],
      code: `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Neo N3 connection
    let n3_provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let n3_client = RpcClient::new(n3_provider);
    
    // Neo X connection  
    let neo_x_provider = HttpProvider::new("https://neoxt4seed1.ngd.network:443")?;
    let neo_x_client = RpcClient::new(neo_x_provider);
    
    let account = Account::from_private_key("your_private_key")?;
    let signer = AccountSigner::new(account);
    
    // Bridge contract on Neo N3
    let bridge_contract = "0xBridgeContractHashHere".parse()?;
    
    // Initiate bridge transfer
    let amount = 10_000_000_000; // 100 GAS
    let neo_x_address = "0xYourNeoXAddressHere";
    
    let tx_hash = n3_client
        .invoke_function_tx(
            &signer,
            &bridge_contract,
            "bridgeToNeoX",
            vec![
                ContractParameter::Integer(amount.into()),
                ContractParameter::String(neo_x_address.to_string()),
            ],
            None,
        )
        .await?;
    
    println!("Bridge transaction initiated: {}", tx_hash);
    println!("Monitor for completion on both networks");
    
    Ok(())
}`
    },
  ],
};

// Copy to clipboard functionality
const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text).then(() => {
    // Could add a toast notification here
  });
};

export default function Examples(): JSX.Element {
  const [activeCategory, setActiveCategory] = useState('basic');

  const currentCategory = categories.find(cat => cat.id === activeCategory);
  const currentExamples = examples[activeCategory] || [];

  return (
    <Layout
      title="Examples"
      description="Comprehensive examples and tutorials for NeoRust SDK - Learn by example with real-world code samples">
      
      {/* Header */}
      <header className={styles.examplesHeader}>
        <div className="container">
          <h1 className={clsx('gradient-text', styles.pageTitle)}>
            Code Examples
          </h1>
          <p className={styles.pageSubtitle}>
            Learn NeoRust SDK through practical examples. From basic operations to advanced patterns, 
            explore real-world code samples that you can use in your projects.
          </p>
        </div>
      </header>

      <main className={styles.examplesMain}>
        <div className="container">
          <div className={styles.examplesContent}>
            
            {/* Category Navigation */}
            <div className={styles.categoryNav}>
              <div className={styles.categoryButtons}>
                {categories.map((category) => (
                  <button
                    key={category.id}
                    onClick={() => setActiveCategory(category.id)}
                    className={clsx(
                      'btn',
                      styles.categoryButton,
                      activeCategory === category.id ? 'btn-primary' : 'btn-secondary'
                    )}
                  >
                    <span>{category.icon}</span>
                    {category.title}
                  </button>
                ))}
              </div>
            </div>

            {/* Category Description */}
            {currentCategory && (
              <div className={styles.categoryDescription}>
                <div className={styles.categoryInfo}>
                  <h2 className={styles.categoryTitle}>
                    {currentCategory.icon} {currentCategory.title}
                  </h2>
                  <p className={styles.categorySubtitle}>
                    {currentCategory.description}
                  </p>
                </div>
              </div>
            )}

            {/* Examples Grid */}
            <div className={styles.examplesGrid}>
              {currentExamples.map((example, index) => (
                <div key={index} className={clsx('card', styles.example)}>
                  <div className={styles.exampleHeader}>
                    <div>
                      <h3 className={styles.exampleTitle}>{example.title}</h3>
                      <p className={styles.exampleDescription}>{example.description}</p>
                      <div className={styles.tags}>
                        {example.tags.map((tag) => (
                          <span key={tag} className={styles.tag}>
                            {tag}
                          </span>
                        ))}
                      </div>
                    </div>
                    <button
                      onClick={() => copyToClipboard(example.code)}
                      className={clsx('btn btn-secondary', styles.copyButton)}
                      title="Copy to clipboard"
                    >
                      📋
                    </button>
                  </div>
                  <div className={styles.codeContainer}>
                    <CodeBlock language="rust" showLineNumbers>
                      {example.code}
                    </CodeBlock>
                  </div>
                </div>
              ))}
            </div>

            {/* CTA Section */}
            <div className={styles.ctaSection}>
              <div className={styles.ctaContent}>
                <h2 className={styles.ctaTitle}>Ready to Build?</h2>
                <p className={styles.ctaSubtitle}>
                  Start building your Neo N3 application with NeoRust SDK. 
                  Check out our comprehensive documentation and guides.
                </p>
                <div className={styles.ctaButtons}>
                  <Link to="/docs/intro" className={clsx('btn btn-primary', styles.ctaButton)}>
                    📚 Read Documentation
                  </Link>
                  <Link to="/sdk" className={clsx('btn btn-secondary', styles.ctaButton)}>
                    🦀 Explore SDK
                  </Link>
                  <a 
                    href="https://github.com/R3E-Network/NeoRust" 
                    className={clsx('btn btn-secondary', styles.ctaButton)}
                    target="_blank" 
                    rel="noopener noreferrer"
                  >
                    ⭐ View on GitHub
                  </a>
                </div>
              </div>
            </div>

            {/* Resources Section */}
            <div className={styles.resourcesSection}>
              <h2 className={styles.sectionTitle}>Additional Resources</h2>
              <div className={styles.resourcesGrid}>
                <div className={clsx('card', styles.resource)}>
                  <div className={styles.resourceIcon}>📖</div>
                  <h3 className={styles.resourceTitle}>API Documentation</h3>
                  <p className={styles.resourceDescription}>
                    Comprehensive API reference with detailed method documentation.
                  </p>
                  <a href="https://docs.rs/neo3" className={styles.resourceLink}>
                    View API Docs →
                  </a>
                </div>
                
                <div className={clsx('card', styles.resource)}>
                  <div className={styles.resourceIcon}>🎓</div>
                  <h3 className={styles.resourceTitle}>Tutorials</h3>
                  <p className={styles.resourceDescription}>
                    Step-by-step tutorials for building complete applications.
                  </p>
                  <Link to="/docs/tutorials" className={styles.resourceLink}>
                    Start Learning →
                  </Link>
                </div>
                
                <div className={clsx('card', styles.resource)}>
                  <div className={styles.resourceIcon}>💬</div>
                  <h3 className={styles.resourceTitle}>Community</h3>
                  <p className={styles.resourceDescription}>
                    Join our community for help, discussions, and contributions.
                  </p>
                  <a href="https://github.com/R3E-Network/NeoRust/discussions" className={styles.resourceLink}>
                    Join Discussion →
                  </a>
                </div>
              </div>
            </div>

          </div>
        </div>
      </main>
    </Layout>
  );
} 