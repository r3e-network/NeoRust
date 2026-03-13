use neo3::neo_clients::{HttpProvider, RpcClient};
use neo3::neo_x::{NeoXProvider, NeoXWallet};
use neo3::sdk::unified::EcosystemClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize N3 Provider (Optional if you just want EVM, but useful for bridge/unified)
    let n3_provider = HttpProvider::new("https://mainnet1.neo.org:443")?;
    let n3_client = RpcClient::new(n3_provider);

    // 2. Setup Neo X EVM components
    let neo_x_provider = NeoXProvider::new("https://rpc.neo-x.org", Some(&n3_client));
    
    // Create a new randomized EVM Wallet (or load from PK)
    let evm_wallet = NeoXWallet::create_random();
    println!("Created new EVM Wallet address: {:?}", evm_wallet.address());

    // 3. Create the Unified Client for Neo X
    let ecosystem_client = EcosystemClient::new_neox(evm_wallet, neo_x_provider);

    // 4. Get Balance directly via the Unified Interface
    match ecosystem_client.get_balance().await {
        Ok(balance) => println!("Neo X Balance: {} Wei", balance),
        Err(e) => println!("Error getting balance: {}", e),
    }

    // 5. Example of Anti-MEV Client setup
    let anti_mev_wallet = NeoXWallet::create_random();
    let _anti_mev_client = EcosystemClient::new_neox_anti_mev(anti_mev_wallet);
    
    // Any transactions sent through anti_mev_client will route through the protected mempool endpoint
    println!("Anti-MEV client initialized successfully.");

    Ok(())
}
