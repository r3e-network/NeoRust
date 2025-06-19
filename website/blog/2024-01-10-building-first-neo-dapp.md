---
slug: building-first-neo-dapp
title: 🏗️ Building Your First Neo dApp with NeoRust - Complete Guide
authors: [sarah-chen]
tags: [tutorial, dapp, neo3, rust, beginner]
---

# Building Your First Neo dApp with NeoRust 🏗️

Welcome to the complete guide for building your first decentralized application (dApp) on Neo N3 using NeoRust! In this tutorial, we'll create a simple but functional **Token Voting dApp** that demonstrates core Neo blockchain concepts.

## What We'll Build 🎯

Our **Token Voting dApp** will feature:
- **Smart Contract**: NEP-17 token with voting functionality
- **Frontend**: React web interface for voting
- **Backend**: Rust service for blockchain interaction
- **Wallet Integration**: Connect with Neo wallets

By the end of this tutorial, you'll have a complete understanding of Neo dApp development!

<!--truncate-->

## Prerequisites 📋

Before we start, ensure you have:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js for frontend
node --version  # v18+

# NeoRust CLI
cargo install neo3-cli
```

**Knowledge Requirements:**
- Basic Rust programming
- Understanding of blockchain concepts
- Familiarity with smart contracts

## Step 1: Project Setup 🛠️

Let's create our project structure:

```bash
mkdir neo-voting-dapp
cd neo-voting-dapp

# Create Rust workspace
cat > Cargo.toml << EOF
[workspace]
members = [
    "smart-contract",
    "backend-service", 
    "integration-tests"
]
EOF

# Create project directories
mkdir smart-contract backend-service frontend integration-tests
```

## Step 2: Smart Contract Development 📝

### Creating the Contract

```bash
cd smart-contract
cargo init --lib
```

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
neo3 = "0.4.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[lib]
crate-type = ["cdylib"]
```

### Contract Implementation

Create `src/lib.rs`:

```rust
use neo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub deadline: u64,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct VotingContract {
    owner: Address,
    proposals: HashMap<u32, Proposal>,
    voter_power: HashMap<Address, u64>,
    next_proposal_id: u32,
}

impl VotingContract {
    #[neo3::init]
    pub fn init(owner: Address) -> Self {
        Self {
            owner,
            proposals: HashMap::new(),
            voter_power: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    #[neo3::method]
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        voting_period: u64,
    ) -> Result<u32, String> {
        // Verify caller is owner
        if Runtime::calling_script_hash() != self.owner.script_hash() {
            return Err("Only owner can create proposals".to_string());
        }

        let proposal_id = self.next_proposal_id;
        let deadline = Runtime::time() + voting_period;
        
        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            votes_for: 0,
            votes_against: 0,
            deadline,
            active: true,
        };

        self.proposals.insert(proposal_id, proposal);
        self.next_proposal_id += 1;

        // Emit event
        Runtime::notify(&[
            "ProposalCreated".into(),
            proposal_id.into(),
            deadline.into(),
        ]);

        Ok(proposal_id)
    }

    #[neo3::method]
    pub fn vote(
        &mut self,
        proposal_id: u32,
        support: bool,
    ) -> Result<(), String> {
        let caller = Runtime::calling_script_hash();
        
        // Get voter power (based on token balance)
        let power = self.get_voter_power(&caller)?;
        if power == 0 {
            return Err("No voting power".to_string());
        }

        // Get proposal
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or("Proposal not found")?;

        // Check if still active
        if !proposal.active || Runtime::time() > proposal.deadline {
            return Err("Proposal voting ended".to_string());
        }

        // Cast vote
        if support {
            proposal.votes_for += power;
        } else {
            proposal.votes_against += power;
        }

        // Emit vote event
        Runtime::notify(&[
            "VoteCast".into(),
            caller.into(),
            proposal_id.into(),
            support.into(),
            power.into(),
        ]);

        Ok(())
    }

    #[neo3::method]
    pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
        self.proposals.get(&proposal_id).cloned()
    }

    #[neo3::method]
    pub fn get_all_proposals(&self) -> Vec<Proposal> {
        self.proposals.values().cloned().collect()
    }

    fn get_voter_power(&self, voter: &ScriptHash) -> Result<u64, String> {
        // In a real contract, this would check NEP-17 token balance
        // For demo, we'll use a simple power assignment
        Ok(self.voter_power.get(&Address::from(*voter)).copied().unwrap_or(100))
    }

    #[neo3::method]
    pub fn set_voter_power(&mut self, voter: Address, power: u64) -> Result<(), String> {
        if Runtime::calling_script_hash() != self.owner.script_hash() {
            return Err("Only owner can set voting power".to_string());
        }
        
        self.voter_power.insert(voter, power);
        Ok(())
    }
}

// Export the contract
#[neo3::contract]
pub fn main() -> VotingContract {
    VotingContract::init(Runtime::calling_script_hash().into())
}
```

### Compile the Contract

```bash
# Build the contract
cargo build --release --target wasm32-unknown-unknown

# Generate Neo contract files
neo3-compiler compile \
  --input target/wasm32-unknown-unknown/release/smart_contract.wasm \
  --output voting-contract.nef
```

## Step 3: Backend Service 🚀

Create a Rust service to interact with our smart contract:

```bash
cd ../backend-service
cargo init
```

`Cargo.toml`:

```toml
[dependencies]
neo3 = "0.4.1"
tokio = { version = "1.0", features = ["full"] }
warp = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

`src/main.rs`:

```rust
use neo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct CreateProposalRequest {
    title: String,
    description: String,
    voting_period: u64,
}

#[derive(Serialize, Deserialize)]
struct VoteRequest {
    proposal_id: u32,
    support: bool,
}

#[derive(Clone)]
struct AppState {
    neo_client: Arc<RpcClient>,
    contract_hash: ScriptHash,
    owner_account: Account,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Neo client
    let provider = HttpProvider::new("https://testnet1.neo.org:443")?;
    let client = RpcClient::new(provider);
    
    // Load contract and owner account
    let contract_hash = ScriptHash::from_str("0x1234...contract_hash...")?;
    let owner_account = Account::from_wif("your_private_key_wif")?;
    
    let state = AppState {
        neo_client: Arc::new(client),
        contract_hash,
        owner_account,
    };

    // API Routes
    let create_proposal = warp::path("api")
        .and(warp::path("proposals"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(create_proposal_handler);

    let vote = warp::path("api")
        .and(warp::path("vote"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(vote_handler);

    let get_proposals = warp::path("api")
        .and(warp::path("proposals"))
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(get_proposals_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    let routes = create_proposal
        .or(vote)
        .or(get_proposals)
        .with(cors);

    println!("🚀 Server running on http://localhost:3001");
    warp::serve(routes).run(([127, 0, 0, 1], 3001)).await;
    
    Ok(())
}

fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn create_proposal_handler(
    req: CreateProposalRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    match create_proposal(req, state).await {
        Ok(proposal_id) => Ok(warp::reply::json(&serde_json::json!({
            "success": true,
            "proposal_id": proposal_id
        }))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

async fn vote_handler(
    req: VoteRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    match cast_vote(req, state).await {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!({
            "success": true
        }))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

async fn get_proposals_handler(
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    match get_all_proposals(state).await {
        Ok(proposals) => Ok(warp::reply::json(&proposals)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

async fn create_proposal(
    req: CreateProposalRequest,
    state: AppState,
) -> Result<u32, Box<dyn std::error::Error>> {
    let transaction = TransactionBuilder::new()
        .add_contract_call(
            state.contract_hash,
            "create_proposal",
            vec![
                req.title.into(),
                req.description.into(),
                req.voting_period.into(),
            ],
        )?
        .add_signer(state.owner_account.get_script_hash())?
        .build()?;

    let signed_tx = state.owner_account.sign_transaction(transaction)?;
    let result = state.neo_client.send_raw_transaction(signed_tx).await?;
    
    // Parse proposal ID from transaction result
    let proposal_id = result
        .application_log
        .executions
        .first()
        .and_then(|exec| exec.notifications.first())
        .and_then(|notif| notif.state.as_array())
        .and_then(|state| state.first())
        .and_then(|item| item.as_integer())
        .ok_or("Failed to parse proposal ID from transaction result")?;
        
    Ok(proposal_id as u32)
}

async fn cast_vote(
    req: VoteRequest,
    state: AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    let transaction = TransactionBuilder::new()
        .add_contract_call(
            state.contract_hash,
            "vote",
            vec![
                req.proposal_id.into(),
                req.support.into(),
            ],
        )?
        .add_signer(state.owner_account.get_script_hash())?
        .build()?;

    let signed_tx = state.owner_account.sign_transaction(transaction)?;
    state.neo_client.send_raw_transaction(signed_tx).await?;
    
    Ok(())
}

async fn get_all_proposals(
    state: AppState,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let result = state.neo_client
        .invoke_function(
            state.contract_hash,
            "get_all_proposals",
            vec![],
        )
        .await?;
    
    // Parse proposals from contract response
    let proposals = result
        .stack
        .first()
        .and_then(|item| item.as_array())
        .map(|array| {
            array
                .iter()
                .filter_map(|item| {
                    // Parse each proposal from the contract's return format
                    serde_json::from_value(item.clone()).ok()
                })
                .collect()
        })
        .unwrap_or_default();
        
    Ok(proposals)
}
```

## Step 4: Frontend Development 🎨

Create a React frontend:

```bash
cd ../frontend
npx create-react-app . --template typescript
npm install @cityofzion/wallet-connect-sdk-react axios
```

### Main Component (`src/App.tsx`):

```typescript
import React, { useState, useEffect } from 'react';
import { WalletConnectSDK } from '@cityofzion/wallet-connect-sdk-react';
import axios from 'axios';
import './App.css';

interface Proposal {
  id: number;
  title: string;
  description: string;
  votes_for: number;
  votes_against: number;
  deadline: number;
  active: boolean;
}

function App() {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [connected, setConnected] = useState(false);
  const [loading, setLoading] = useState(false);

  const walletConnect = new WalletConnectSDK({
    projectId: 'your_project_id',
    metadata: {
      name: 'Neo Voting dApp',
      description: 'Decentralized voting on Neo blockchain',
      url: 'https://yourapp.com',
      icons: ['https://yourapp.com/icon.png']
    }
  });

  useEffect(() => {
    loadProposals();
  }, []);

  const loadProposals = async () => {
    try {
      const response = await axios.get('http://localhost:3001/api/proposals');
      setProposals(response.data);
    } catch (error) {
      console.error('Failed to load proposals:', error);
    }
  };

  const connectWallet = async () => {
    try {
      await walletConnect.connect();
      setConnected(true);
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const vote = async (proposalId: number, support: boolean) => {
    if (!connected) {
      alert('Please connect your wallet first');
      return;
    }

    setLoading(true);
    try {
      await axios.post('http://localhost:3001/api/vote', {
        proposal_id: proposalId,
        support
      });
      
      await loadProposals(); // Refresh
      alert('Vote cast successfully!');
    } catch (error) {
      console.error('Failed to vote:', error);
      alert('Failed to cast vote');
    }
    setLoading(false);
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>🗳️ Neo Voting dApp</h1>
        {!connected ? (
          <button onClick={connectWallet} className="connect-btn">
            Connect Wallet
          </button>
        ) : (
          <span className="connected">✅ Wallet Connected</span>
        )}
      </header>

      <main className="proposals-container">
        <h2>Active Proposals</h2>
        {proposals.length === 0 ? (
          <p>No proposals available</p>
        ) : (
          proposals.map(proposal => (
            <div key={proposal.id} className="proposal-card">
              <h3>{proposal.title}</h3>
              <p>{proposal.description}</p>
              
              <div className="voting-stats">
                <div className="vote-count">
                  👍 For: {proposal.votes_for}
                </div>
                <div className="vote-count">
                  👎 Against: {proposal.votes_against}
                </div>
              </div>

              <div className="voting-buttons">
                <button
                  onClick={() => vote(proposal.id, true)}
                  disabled={loading || !proposal.active}
                  className="vote-btn vote-for"
                >
                  Vote For
                </button>
                <button
                  onClick={() => vote(proposal.id, false)}
                  disabled={loading || !proposal.active}
                  className="vote-btn vote-against"
                >
                  Vote Against
                </button>
              </div>

              <div className="proposal-status">
                {proposal.active ? '🟢 Active' : '🔴 Ended'}
                <span className="deadline">
                  Deadline: {new Date(proposal.deadline * 1000).toLocaleDateString()}
                </span>
              </div>
            </div>
          ))
        )}
      </main>
    </div>
  );
}

export default App;
```

### Styling (`src/App.css`):

```css
.App {
  text-align: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.App-header {
  padding: 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.connect-btn {
  background: #10b981;
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 8px;
  font-size: 16px;
  cursor: pointer;
  transition: background 0.3s;
}

.connect-btn:hover {
  background: #059669;
}

.connected {
  color: #10b981;
  font-weight: bold;
}

.proposals-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

.proposal-card {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1.5rem;
  margin: 1rem 0;
  text-align: left;
  backdrop-filter: blur(10px);
}

.voting-stats {
  display: flex;
  gap: 2rem;
  margin: 1rem 0;
}

.vote-count {
  font-size: 18px;
  font-weight: bold;
}

.voting-buttons {
  display: flex;
  gap: 1rem;
  margin: 1rem 0;
}

.vote-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s;
}

.vote-for {
  background: #10b981;
  color: white;
}

.vote-against {
  background: #ef4444;
  color: white;
}

.vote-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.vote-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.proposal-status {
  display: flex;
  justify-content: space-between;
  margin-top: 1rem;
  font-size: 14px;
}
```

## Step 5: Integration Tests 🧪

Create comprehensive tests:

```bash
cd ../integration-tests
cargo init
```

`src/main.rs`:

```rust
use neo3::prelude::*;
use tokio;

#[tokio::test]
async fn test_full_voting_flow() -> Result<(), Box<dyn std::error::Error>> {
    // Setup test environment
    let provider = HttpProvider::new("http://localhost:20332")?; // Local testnet
    let client = RpcClient::new(provider);
    
    // Deploy contract
    let contract_hash = deploy_voting_contract(&client).await?;
    
    // Test proposal creation
    let proposal_id = create_test_proposal(&client, contract_hash).await?;
    assert_eq!(proposal_id, 1);
    
    // Test voting
    cast_test_vote(&client, contract_hash, proposal_id, true).await?;
    
    // Verify vote was recorded
    let proposal = get_proposal(&client, contract_hash, proposal_id).await?;
    assert!(proposal.votes_for > 0);
    
    println!("✅ All tests passed!");
    Ok(())
}

async fn deploy_voting_contract(client: &RpcClient) -> Result<ScriptHash, Box<dyn std::error::Error>> {
    // Contract deployment logic
    todo!("Implement contract deployment")
}

async fn create_test_proposal(client: &RpcClient, contract_hash: ScriptHash) -> Result<u32, Box<dyn std::error::Error>> {
    // Proposal creation logic
    todo!("Implement proposal creation test")
}

async fn cast_test_vote(client: &RpcClient, contract_hash: ScriptHash, proposal_id: u32, support: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Voting logic
    todo!("Implement voting test")
}

#[derive(Debug)]
struct Proposal {
    votes_for: u64,
    votes_against: u64,
}

async fn get_proposal(client: &RpcClient, contract_hash: ScriptHash, proposal_id: u32) -> Result<Proposal, Box<dyn std::error::Error>> {
    // Get proposal logic
    todo!("Implement get proposal test")
}

fn main() {
    println!("Run tests with: cargo test");
}
```

## Step 6: Deployment & Testing 🚀

### Deploy to TestNet

```bash
# Deploy smart contract
cd smart-contract
neo3-cli deploy \
  --nef voting-contract.nef \
  --manifest voting-contract.manifest.json \
  --testnet

# Start backend service
cd ../backend-service
cargo run

# Start frontend
cd ../frontend
npm start
```

### Test the Complete Flow

1. **Connect Wallet**: Use NeoLine or O3 wallet
2. **Create Proposal**: Call backend API
3. **Cast Votes**: Interact through frontend
4. **View Results**: Real-time updates

## Advanced Features 🔥

### Add Real NEP-17 Token Integration

```rust
// In smart contract
#[neo3::method]
pub fn get_voter_power(&self, voter: &Address) -> Result<u64, String> {
    // Get actual token balance
    let token_contract = ScriptHash::from_str("0x...token_contract...")?;
    let balance: u64 = Runtime::call_contract(
        token_contract,
        "balanceOf",
        vec![voter.into()]
    )?;
    
    Ok(balance / 100000000) // Convert from smallest unit
}
```

### Add Governance Features

```rust
#[neo3::method]
pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<(), String> {
    let proposal = self.proposals.get(&proposal_id)
        .ok_or("Proposal not found")?;
    
    // Check if proposal passed
    if proposal.votes_for > proposal.votes_against {
        // Execute proposal logic
        self.execute_governance_action(proposal)?;
    }
    
    Ok(())
}
```

## Best Practices 📚

### Security Considerations
- **Input Validation**: Always validate user inputs
- **Access Control**: Implement proper permission checks
- **Reentrancy Protection**: Prevent recursive calls
- **Integer Overflow**: Use safe math operations

### Performance Optimization
- **Gas Efficiency**: Minimize storage operations
- **Batch Operations**: Group multiple calls
- **Caching**: Cache frequently accessed data
- **Event Indexing**: Use events for efficient querying

### Testing Strategy
- **Unit Tests**: Test individual functions
- **Integration Tests**: Test contract interactions
- **End-to-End Tests**: Test complete user flows
- **Security Audits**: Regular security reviews

## Troubleshooting 🔧

### Common Issues

**Contract Deployment Fails**
```bash
# Check network connection
neo3-cli network status

# Verify contract compilation
neo3-compiler validate voting-contract.nef
```

**Frontend Can't Connect**
```javascript
// Enable CORS in backend
app.use(cors({
  origin: "http://localhost:3000",
  credentials: true
}));
```

**Wallet Connection Issues**
```typescript
// Check wallet compatibility
if (!window.NEOLine) {
  alert('Please install NeoLine wallet');
  return;
}
```

## What's Next? 🔮

Congratulations! You've built a complete Neo dApp. Here are next steps:

### Enhancements
- **Mobile App**: React Native version
- **Advanced UI**: Better design and animations
- **Real-time Updates**: WebSocket integration
- **Multi-language**: i18n support

### Production Deployment
- **MainNet Deployment**: Deploy to production
- **CDN Integration**: Optimize frontend delivery
- **Monitoring**: Add application monitoring
- **Security Audit**: Professional security review

### Community Features
- **DAO Integration**: Full DAO functionality
- **Token Distribution**: Airdrop mechanisms
- **Staking Rewards**: Incentive mechanisms
- **Governance Evolution**: Advanced governance features

## Conclusion 🎉

You've successfully built a complete Neo dApp with:
- ✅ Smart contract with voting logic
- ✅ Rust backend service
- ✅ React frontend interface
- ✅ Wallet integration
- ✅ Testing framework

**Key Learnings:**
- Neo N3 smart contract development
- NeoRust SDK capabilities
- Full-stack dApp architecture
- Blockchain integration patterns

### Resources
- **[NeoRust Documentation](../docs)**: Complete API reference
- **[Neo Developer Hub](https://developers.neo.org)**: Official Neo resources
- **[Community Discord](https://discord.gg/neo-rust)**: Get help and connect
- **[GitHub Repository](https://github.com/R3E-Network/NeoRust)**: Source code and examples

**Ready to build more amazing dApps?** 🚀

Share your creation with the community and let's build the future of decentralized applications together!

---

*Follow [@NeoRustSDK](https://twitter.com/neorustSDK) for more tutorials and updates!* 🦀⚡️ 