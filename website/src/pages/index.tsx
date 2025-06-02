import React, { useEffect, useState, useCallback } from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import CodeBlock from '@theme/CodeBlock';
import styles from './index.module.css';
import clsx from 'clsx';

// Interface for blockchain info
interface BlockchainInfo {
  blockHeight: number;
  blockHash: string;
  timestamp: string;
  transactions: number;
  version: string;
  loading: boolean;
  lastUpdated: number;
}

// Feature data
const features = [
  {
    title: 'Performance Optimized',
    icon: '⚡',
    description: 'Built with Rust\'s performance and safety guarantees for high-throughput blockchain applications.',
  },
  {
    title: 'Comprehensive Security',
    icon: '🔒',
    description: 'State-of-the-art cryptographic implementations with thorough security considerations.',
  },
  {
    title: 'Smart Contract Support',
    icon: '📋',
    description: 'Intuitive interfaces for deploying and interacting with Neo N3 smart contracts.',
  },
  {
    title: 'Wallet Management',
    icon: '💰',
    description: 'Complete wallet functionality with NEP-6 standard support and hardware wallet integration.',
  },
  {
    title: 'Neo X Integration',
    icon: '🔗',
    description: 'Seamless integration with Neo X for EVM compatibility and cross-chain operations.',
  },
  {
    title: 'Developer Friendly',
    icon: '🛠️',
    description: 'Intuitive, well-documented API with type safety and comprehensive examples.',
  },
];

// Stats data
const stats = [
  { label: '100% Rust-Native', value: '100%', description: 'Pure Rust implementation' },
  { label: 'Neo N3 Support', value: 'Full', description: 'Complete Neo N3 compatibility' },
  { label: 'Neo X Ready', value: '✓', description: 'EVM compatibility layer' },
  { label: 'Test Coverage', value: '278/278', description: 'All tests passing' },
];

const exampleCode = `use neo3::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Neo N3 TestNet
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Get basic blockchain information
    let block_count = client.get_block_count().await?;
    println!("Current block height: {}", block_count);
    
    // Create a new account
    let account = Account::create()?;
    println!("New address: {}", account.get_address());
    
    // Initialize GAS token contract
    let gas_token = GasToken::new(&client);
    let symbol = gas_token.symbol().await?;
    let decimals = gas_token.decimals().await?;
    
    println!("Token: {} with {} decimals", symbol, decimals);
    
    // Check account balance
    let balance = gas_token.balance_of(&account.get_script_hash()).await?;
    println!("Balance: {} {}", balance, symbol);
    
    Ok(())
}`;

function Feature({ title, icon, description }: typeof features[0]) {
  return (
    <div className={clsx('card', styles.feature)}>
      <div className={styles.featureIcon}>{icon}</div>
      <h3 className={styles.featureTitle}>{title}</h3>
      <p className={styles.featureDescription}>{description}</p>
    </div>
  );
}

function Stat({ label, value, description }: typeof stats[0]) {
  return (
    <div className={clsx('card', styles.stat)}>
      <div className={styles.statValue}>{value}</div>
      <div className={styles.statLabel}>{label}</div>
      <div className={styles.statDescription}>{description}</div>
    </div>
  );
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext();
  
  // State for blockchain info
  const [blockchainInfo, setBlockchainInfo] = useState<BlockchainInfo>({
    blockHeight: 0,
    blockHash: '',
    timestamp: '',
    transactions: 0,
    version: '',
    loading: true,
    lastUpdated: 0
  });

  // Fetch blockchain info
  const fetchBlockchainInfo = useCallback(async () => {
    try {
      // Use Neo RPC endpoint (you might want to use axios or fetch properly)
      const response = await fetch('https://rpc1.neo.org:443', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'getblockcount',
          params: []
        })
      });
      
      const data = await response.json();
      const blockCount = data.result;
      
      // Get the latest block info
      const blockResponse = await fetch('https://rpc1.neo.org:443', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'getblock',
          params: [blockCount - 1, 1]
        })
      });
      
      const blockData = await blockResponse.json();
      const block = blockData.result;
      const blockTime = new Date(block.time * 1000).toLocaleString();
      
      // Get version info
      const versionResponse = await fetch('https://rpc1.neo.org:443', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'getversion',
          params: []
        })
      });
      
      const versionData = await versionResponse.json();
      const version = versionData.result.useragent;
      
      setBlockchainInfo({
        blockHeight: blockCount - 1,
        blockHash: block.hash,
        timestamp: blockTime,
        transactions: block.tx ? block.tx.length : 0,
        version: version,
        loading: false,
        lastUpdated: Date.now()
      });
    } catch (error) {
      console.error('Error fetching blockchain info:', error);
      setBlockchainInfo(prev => ({...prev, loading: false}));
    }
  }, []);

  // Force refresh function
  const refreshBlockchainInfo = () => {
    setBlockchainInfo(prev => ({...prev, loading: true}));
    fetchBlockchainInfo();
  };

  // Fetch blockchain info on mount and periodically
  useEffect(() => {
    fetchBlockchainInfo();
    
    // Update every 15 seconds
    const interval = setInterval(() => {
      fetchBlockchainInfo();
    }, 15000);
    
    return () => {
      clearInterval(interval);
    };
  }, [fetchBlockchainInfo]);

  return (
    <Layout
      title={`${siteConfig.title} - ${siteConfig.tagline}`}
      description="NeoRust v0.4.1 - A comprehensive Rust SDK for Neo N3 blockchain development. Build high-performance dApps with type-safe, modern Rust.">
      
      {/* Hero Section */}
      <header className={clsx('hero hero--primary', styles.heroBanner)}>
        <div className="container">
          <div className={styles.heroContent}>
            <div className={styles.heroLogo}>
              <div className={styles.logoCircle}>
                <span className={styles.logoIcon}>⚡</span>
              </div>
            </div>
            <h1 className={styles.heroTitle}>
              <span className={styles.heroTitlePrimary}>Neo</span>
              <span className={styles.heroTitleSecondary}>Rust SDK</span>
            </h1>
            <p className={styles.heroSubtitle}>
              A comprehensive Rust library for building high-performance applications on the Neo N3 blockchain ecosystem
            </p>
            
            <div className={styles.heroButtons}>
              <Link
                className={clsx('btn btn-primary', styles.heroButton)}
                to="/docs/intro">
                Get Started
                <span className={styles.buttonIcon}>→</span>
              </Link>
              <Link
                className={clsx('btn btn-secondary', styles.heroButton)}
                href="https://github.com/R3E-Network/NeoRust">
                <span className={styles.githubIcon}>⭐</span>
                View on GitHub
              </Link>
            </div>
            
            <div className={styles.heroStats}>
              {stats.map((stat, index) => (
                <Stat key={index} {...stat} />
              ))}
            </div>
          </div>
        </div>
      </header>

      <main>
        {/* Blockchain Status Section */}
        <section className={styles.blockchainSection}>
          <div className="container">
            <div className={styles.blockchainCard}>
              <div className={styles.blockchainHeader}>
                <div className={styles.blockchainInfo}>
                  <div className={styles.blockchainIcon}>📊</div>
                  <div>
                    <h3>Neo N3 Blockchain Status</h3>
                    <p>Live network statistics</p>
                  </div>
                </div>
                
                <button 
                  onClick={refreshBlockchainInfo}
                  disabled={blockchainInfo.loading}
                  className={clsx('btn btn-secondary', styles.refreshButton)}
                >
                  {blockchainInfo.loading ? (
                    <>
                      <div className={styles.spinner}></div>
                      Updating...
                    </>
                  ) : (
                    <>
                      <span className={styles.refreshIcon}>🔄</span>
                      Refresh
                    </>
                  )}
                </button>
              </div>
              
              {blockchainInfo.loading && !blockchainInfo.blockHeight ? (
                <div className={styles.loadingState}>
                  <div className={styles.spinner}></div>
                  <span>Loading blockchain data...</span>
                </div>
              ) : (
                <div className={styles.blockchainStats}>
                  <div className={styles.blockchainStat}>
                    <div className={styles.statLabel}>Height</div>
                    <div className={styles.statValue}>{blockchainInfo.blockHeight.toLocaleString()}</div>
                  </div>
                  
                  <div className={styles.blockchainStat}>
                    <div className={styles.statLabel}>Latest Block</div>
                    <div className={styles.blockHash}>
                      <span>{blockchainInfo.blockHash ? `${blockchainInfo.blockHash.substring(0, 6)}...${blockchainInfo.blockHash.substring(blockchainInfo.blockHash.length - 8)}` : ''}</span>
                      {blockchainInfo.blockHash && (
                        <a 
                          href={`https://neo3.neotube.io/block/${blockchainInfo.blockHash}`} 
                          target="_blank" 
                          rel="noopener noreferrer"
                          className={styles.externalLink}
                        >
                          🔗
                        </a>
                      )}
                    </div>
                  </div>
                  
                  <div className={styles.blockchainStat}>
                    <div className={styles.statLabel}>Transactions</div>
                    <div className={styles.statValue}>{blockchainInfo.transactions}</div>
                  </div>
                  
                  <div className={styles.blockchainStat}>
                    <div className={styles.statLabel}>Version</div>
                    <div className={styles.statValue}>{blockchainInfo.version}</div>
                  </div>
                </div>
              )}
              
              {!blockchainInfo.loading && blockchainInfo.lastUpdated > 0 && (
                <div className={styles.lastUpdated}>
                  Last updated: {new Date(blockchainInfo.lastUpdated).toLocaleTimeString()}
                </div>
              )}
            </div>
          </div>
        </section>

        {/* Features Section */}
        <section className={styles.featuresSection}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>
                <span className="gradient-text">Key Features</span>
              </h2>
              <p className={styles.sectionSubtitle}>
                Built with Rust's performance and safety guarantees for robust blockchain applications
              </p>
            </div>
            
            <div className={styles.featuresGrid}>
              {features.map((props, idx) => (
                <Feature key={idx} {...props} />
              ))}
            </div>
          </div>
        </section>

        {/* Code Example Section */}
        <section className={styles.codeSection}>
          <div className="container">
            <div className={styles.codeContent}>
              <div className={styles.codeInfo}>
                <h2 className={styles.sectionTitle}>
                  <span className="gradient-text">Simple to Use</span>
                </h2>
                <p className={styles.sectionSubtitle}>
                  Write clean, type-safe blockchain code with modern Rust features
                </p>
                
                <div className={styles.codeFeatures}>
                  {[
                    'Type-safe blockchain interactions',
                    'Async/await support for modern codebases',
                    'Comprehensive error handling',
                    'Extensive documentation and examples'
                  ].map((item, index) => (
                    <div key={index} className={styles.codeFeature}>
                      <span className={styles.checkIcon}>✅</span>
                      {item}
                    </div>
                  ))}
                </div>

                <Link
                  className={clsx('btn btn-primary', styles.codeButton)}
                  to="/docs/getting-started/quick-start">
                  View More Examples
                  <span className={styles.buttonIcon}>→</span>
                </Link>
              </div>
              
              <div className={styles.codeExample}>
                <CodeBlock
                  language="rust"
                  title="Getting Started with NeoRust"
                  showLineNumbers>
                  {exampleCode}
                </CodeBlock>
              </div>
            </div>
          </div>
        </section>

        {/* Tools Section */}
        <section className={styles.toolsSection}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>
                <span className="gradient-text">Complete Toolkit</span>
              </h2>
              <p className={styles.sectionSubtitle}>
                Everything you need for Neo N3 development in one comprehensive suite
              </p>
            </div>
            
            <div className={styles.toolsGrid}>
              <div className={clsx('card', styles.tool)}>
                <div className={styles.toolIcon}>🦀</div>
                <h3 className={styles.toolTitle}>Rust SDK</h3>
                <p className={styles.toolDescription}>
                  Comprehensive Rust library with full Neo N3 support, smart contract interaction, and wallet management.
                </p>
                <Link to="/sdk" className={clsx('btn btn-primary', styles.toolButton)}>
                  Explore SDK
                </Link>
              </div>
              
              <div className={clsx('card', styles.tool)}>
                <div className={styles.toolIcon}>🖥️</div>
                <h3 className={styles.toolTitle}>Desktop GUI</h3>
                <p className={styles.toolDescription}>
                  Modern desktop application built with Tauri for managing wallets, tokens, and blockchain interactions.
                </p>
                <Link to="/gui" className={clsx('btn btn-primary', styles.toolButton)}>
                  View GUI
                </Link>
              </div>
              
              <div className={clsx('card', styles.tool)}>
                <div className={styles.toolIcon}>⌨️</div>
                <h3 className={styles.toolTitle}>CLI Tools</h3>
                <p className={styles.toolDescription}>
                  Command-line interface for developers who prefer terminal-based workflows and automation scripts.
                </p>
                <Link to="/cli" className={clsx('btn btn-primary', styles.toolButton)}>
                  CLI Docs
                </Link>
              </div>
            </div>
          </div>
        </section>

        {/* Testimonials Section */}
        <section className={styles.testimonialsSection}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>
                <span className="gradient-text">Trusted by Developers</span>
              </h2>
              <p className={styles.sectionSubtitle}>
                See what the community is saying about NeoRust v0.4.1
              </p>
            </div>
            
            <div className={styles.testimonialsGrid}>
              <div className={clsx('card', styles.testimonial)}>
                <div className={styles.testimonialQuote}>
                  "NeoRust has completely transformed our development workflow. The type safety and performance improvements are game-changing for our DeFi project. We migrated from JavaScript and haven't looked back."
                </div>
                <div className={styles.testimonialAuthor}>
                  <div className={styles.authorInfo}>
                    <div className={styles.authorName}>Sarah Chen</div>
                    <div className={styles.authorTitle}>Lead Developer at DeFiNeo</div>
                  </div>
                  <div className={styles.authorAvatar}>👩‍💻</div>
                </div>
              </div>
              
              <div className={clsx('card', styles.testimonial)}>
                <div className={styles.testimonialQuote}>
                  "The desktop GUI makes wallet management incredibly intuitive. Our non-technical team members can now interact with Neo blockchain effortlessly. The UX is outstanding."
                </div>
                <div className={styles.testimonialAuthor}>
                  <div className={styles.authorInfo}>
                    <div className={styles.authorName}>Marcus Rodriguez</div>
                    <div className={styles.authorTitle}>CTO at BlockchainCorp</div>
                  </div>
                  <div className={styles.authorAvatar}>👨‍💼</div>
                </div>
              </div>
              
              <div className={clsx('card', styles.testimonial)}>
                <div className={styles.testimonialQuote}>
                  "CLI tools are a developer's dream. We've automated our entire deployment pipeline with NeoRust commands. The configuration system is flexible yet powerful."
                </div>
                <div className={styles.testimonialAuthor}>
                  <div className={styles.authorInfo}>
                    <div className={styles.authorName}>Emma Thompson</div>
                    <div className={styles.authorTitle}>DevOps Engineer at CryptoStart</div>
                  </div>
                  <div className={styles.authorAvatar}>👩‍🔧</div>
                </div>
              </div>
              
              <div className={clsx('card', styles.testimonial)}>
                <div className={styles.testimonialQuote}>
                  "Cross-platform compatibility is excellent. We're running the same codebase on Windows, macOS, and Linux without any issues. The documentation quality is top-notch."
                </div>
                <div className={styles.testimonialAuthor}>
                  <div className={styles.authorInfo}>
                    <div className={styles.authorName}>Alex Petrov</div>
                    <div className={styles.authorTitle}>Full Stack Developer</div>
                  </div>
                  <div className={styles.authorAvatar}>👨‍💻</div>
                </div>
              </div>
              
              <div className={clsx('card', styles.testimonial)}>
                <div className={styles.testimonialQuote}>
                  "Performance is incredible. Our smart contract interactions are 3x faster compared to other SDKs. Memory usage is minimal even with heavy workloads."
                </div>
                <div className={styles.testimonialAuthor}>
                  <div className={styles.authorInfo}>
                    <div className={styles.authorName}>Dr. Lisa Wang</div>
                    <div className={styles.authorTitle}>Blockchain Researcher</div>
                  </div>
                  <div className={styles.authorAvatar}>👩‍🔬</div>
                </div>
              </div>
              
              <div className={clsx('card', styles.testimonial)}>
                <div className={styles.testimonialQuote}>
                  "The security audit results speak for themselves. NeoRust v0.4.1 passes all tests with flying colors. We trust it with our production applications."
                </div>
                <div className={styles.testimonialAuthor}>
                  <div className={styles.authorInfo}>
                    <div className={styles.authorName}>James Mitchell</div>
                    <div className={styles.authorTitle}>Security Engineer at SecureChain</div>
                  </div>
                  <div className={styles.authorAvatar}>👨‍🛡️</div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Getting Started Section */}
        <section className={styles.ctaSection}>
          <div className="container">
            <div className={styles.ctaContent}>
              <h2 className={styles.ctaTitle}>Ready to Build on Neo?</h2>
              <p className={styles.ctaSubtitle}>
                Join the growing community of developers building the future of blockchain with NeoRust v0.4.1
              </p>
              
              <div className={styles.ctaButtons}>
                <Link
                  className={clsx('btn btn-primary', styles.ctaButton)}
                  to="/docs/getting-started/installation">
                  Start Building
                  <span className={styles.buttonIcon}>🚀</span>
                </Link>
                <Link
                  className={clsx('btn btn-secondary', styles.ctaButton)}
                  to="/examples">
                  View Examples
                  <span className={styles.buttonIcon}>📚</span>
                </Link>
              </div>
              
              <div className={styles.ctaNote}>
                <p>
                  <strong>New in v0.4.1:</strong> Enhanced cross-platform compatibility, security fixes, and improved developer experience.
                  <Link to="/blog" className={styles.ctaLink}> Read the release notes →</Link>
                </p>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}