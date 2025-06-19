/*!
# Test Utilities

Common testing utilities and helper functions for the NeoRust test suite.
*/

use neo3::prelude::*;
use std::time::Duration;
use tokio::time::timeout;

/// Test configuration constants
pub const TEST_TIMEOUT_SECS: u64 = 30;
pub const TESTNET_ENDPOINT: &str = "https://testnet1.neo.org:443";
pub const MAINNET_ENDPOINT: &str = "https://mainnet1.neo.org:443";

/// Create a test RPC client for TestNet
pub fn create_test_client() -> Result<RpcClient, Box<dyn std::error::Error>> {
    let provider = HttpProvider::new(TESTNET_ENDPOINT)?;
    Ok(RpcClient::new(provider))
}

/// Create a test RPC client with custom timeout
pub fn create_test_client_with_timeout(timeout_secs: u64) -> Result<RpcClient, Box<dyn std::error::Error>> {
    let mut provider = HttpProvider::new(TESTNET_ENDPOINT)?;
    provider.set_timeout(Duration::from_secs(timeout_secs));
    Ok(RpcClient::new(provider))
}

/// Execute an async test with timeout
pub async fn with_timeout<F, T>(future: F) -> Result<T, Box<dyn std::error::Error>>
where
    F: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    timeout(Duration::from_secs(TEST_TIMEOUT_SECS), future)
        .await
        .map_err(|_| "Test timeout".into())?
}

/// Skip test if network is not available
pub async fn require_network() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_test_client()?;
    client.get_block_count().await?;
    Ok(())
}

/// Generate a random test private key
pub fn generate_test_key() -> Result<PrivateKey, Box<dyn std::error::Error>> {
    PrivateKey::random()
}

/// Create a test account with random keys
pub fn create_test_account() -> Result<Account, Box<dyn std::error::Error>> {
    let private_key = generate_test_key()?;
    Account::from_private_key(private_key)
}

/// Wait for transaction confirmation
pub async fn wait_for_transaction(
    client: &RpcClient,
    tx_hash: &TxId,
    max_attempts: u32,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    for _ in 0..max_attempts {
        if let Ok(tx) = client.get_transaction(tx_hash).await {
            return Ok(tx);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Err("Transaction not found after waiting".into())
}

/// Check if running in CI environment
pub fn is_ci() -> bool {
    std::env::var("CI").is_ok()
}

/// Skip test if in CI environment (for tests requiring manual setup)
#[macro_export]
macro_rules! skip_if_ci {
    () => {
        if crate::common::test_utils::is_ci() {
            println!("Skipping test in CI environment");
            return;
        }
    };
}

/// Retry an async operation with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_attempts: u32,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
    E: std::fmt::Debug,
{
    let mut delay = initial_delay;
    
    for attempt in 0..max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_attempts - 1 {
                    return Err(e);
                }
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
        }
    }
    
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_test_client() {
        let client = create_test_client().unwrap();
        // Test that we can make a basic call
        let result = client.get_block_count().await;
        // Don't assert success as network might not be available
        println!("Block count result: {:?}", result);
    }

    #[test]
    fn test_generate_test_key() {
        let key1 = generate_test_key().unwrap();
        let key2 = generate_test_key().unwrap();
        
        // Keys should be different
        assert_ne!(key1.to_hex(), key2.to_hex());
        
        // Keys should be valid length
        assert_eq!(key1.to_bytes().len(), 32);
        assert_eq!(key2.to_bytes().len(), 32);
    }

    #[test]
    fn test_create_test_account() {
        let account1 = create_test_account().unwrap();
        let account2 = create_test_account().unwrap();
        
        // Accounts should be different
        assert_ne!(account1.address(), account2.address());
        
        // Addresses should be valid
        assert!(account1.address().is_valid());
        assert!(account2.address().is_valid());
    }
}