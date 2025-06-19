use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	hash::Hash,
	sync::Arc,
	time::{Duration, Instant},
};
use tokio::sync::RwLock;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
	/// Maximum number of entries in the cache
	pub max_entries: usize,
	/// Default TTL for cache entries
	pub default_ttl: Duration,
	/// Cleanup interval for expired entries
	pub cleanup_interval: Duration,
	/// Enable LRU eviction when cache is full
	pub enable_lru: bool,
}

impl Default for CacheConfig {
	fn default() -> Self {
		Self {
			max_entries: 1000,
			default_ttl: Duration::from_secs(300),     // 5 minutes
			cleanup_interval: Duration::from_secs(60), // 1 minute
			enable_lru: true,
		}
	}
}

/// Cache entry with expiration and access tracking
#[derive(Debug, Clone)]
struct CacheEntry<V> {
	value: V,
	expires_at: Instant,
	last_accessed: Instant,
	access_count: u64,
}

impl<V> CacheEntry<V> {
	fn new(value: V, ttl: Duration) -> Self {
		let now = Instant::now();
		Self { value, expires_at: now + ttl, last_accessed: now, access_count: 1 }
	}

	fn is_expired(&self) -> bool {
		Instant::now() > self.expires_at
	}

	fn access(&mut self) -> &V {
		self.last_accessed = Instant::now();
		self.access_count += 1;
		&self.value
	}
}

/// High-performance cache with TTL and LRU eviction
pub struct Cache<K, V> {
	config: CacheConfig,
	entries: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
	stats: Arc<RwLock<CacheStats>>,
}

/// Cache statistics
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CacheStats {
	pub hits: u64,
	pub misses: u64,
	pub evictions: u64,
	pub expired_removals: u64,
	pub current_size: usize,
	pub max_size_reached: u64,
}

impl CacheStats {
	pub fn hit_rate(&self) -> f64 {
		if self.hits + self.misses == 0 {
			0.0
		} else {
			self.hits as f64 / (self.hits + self.misses) as f64
		}
	}
}

impl<K, V> Cache<K, V>
where
	K: Hash + Eq + Clone + Send + Sync + 'static,
	V: Clone + Send + Sync + 'static,
{
	/// Create a new cache with the given configuration
	pub fn new(config: CacheConfig) -> Self {
		Self {
			config,
			entries: Arc::new(RwLock::new(HashMap::new())),
			stats: Arc::new(RwLock::new(CacheStats::default())),
		}
	}

	/// Get a value from the cache
	pub async fn get(&self, key: &K) -> Option<V> {
		let mut entries = self.entries.write().await;
		let mut stats = self.stats.write().await;

		if let Some(entry) = entries.get_mut(key) {
			if entry.is_expired() {
				entries.remove(key);
				stats.expired_removals += 1;
				stats.misses += 1;
				stats.current_size = entries.len();
				None
			} else {
				stats.hits += 1;
				Some(entry.access().clone())
			}
		} else {
			stats.misses += 1;
			None
		}
	}

	/// Insert a value into the cache with default TTL
	pub async fn insert(&self, key: K, value: V) {
		self.insert_with_ttl(key, value, self.config.default_ttl).await;
	}

	/// Insert a value into the cache with custom TTL
	pub async fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
		let mut entries = self.entries.write().await;
		let mut stats = self.stats.write().await;

		// Check if we need to evict entries
		if entries.len() >= self.config.max_entries && !entries.contains_key(&key) {
			if self.config.enable_lru {
				self.evict_lru(&mut entries, &mut stats);
			} else {
				// Simple eviction - remove first entry
				if let Some(first_key) = entries.keys().next().cloned() {
					entries.remove(&first_key);
					stats.evictions += 1;
				}
			}
			stats.max_size_reached += 1;
		}

		let entry = CacheEntry::new(value, ttl);
		entries.insert(key, entry);
		stats.current_size = entries.len();
	}

	/// Remove a value from the cache
	pub async fn remove(&self, key: &K) -> Option<V> {
		let mut entries = self.entries.write().await;
		let mut stats = self.stats.write().await;

		let result = entries.remove(key).map(|entry| entry.value);
		stats.current_size = entries.len();
		result
	}

	/// Clear all entries from the cache
	pub async fn clear(&self) {
		let mut entries = self.entries.write().await;
		let mut stats = self.stats.write().await;

		entries.clear();
		stats.current_size = 0;
	}

	/// Get cache statistics
	pub async fn stats(&self) -> CacheStats {
		let stats = self.stats.read().await;
		CacheStats {
			hits: stats.hits,
			misses: stats.misses,
			evictions: stats.evictions,
			expired_removals: stats.expired_removals,
			current_size: stats.current_size,
			max_size_reached: stats.max_size_reached,
		}
	}

	/// Clean up expired entries
	pub async fn cleanup_expired(&self) {
		let mut entries = self.entries.write().await;
		let mut stats = self.stats.write().await;

		let initial_size = entries.len();
		entries.retain(|_, entry| !entry.is_expired());
		let removed = initial_size - entries.len();

		stats.expired_removals += removed as u64;
		stats.current_size = entries.len();
	}

	/// Get current cache size
	pub async fn size(&self) -> usize {
		let entries = self.entries.read().await;
		entries.len()
	}

	/// Check if cache contains a key
	pub async fn contains_key(&self, key: &K) -> bool {
		let entries = self.entries.read().await;
		entries.contains_key(key)
	}

	/// Evict least recently used entry
	fn evict_lru(&self, entries: &mut HashMap<K, CacheEntry<V>>, stats: &mut CacheStats) {
		if let Some((lru_key, _)) = entries
			.iter()
			.min_by_key(|(_, entry)| entry.last_accessed)
			.map(|(k, v)| (k.clone(), v.last_accessed))
		{
			entries.remove(&lru_key);
			stats.evictions += 1;
		}
	}

	/// Start background cleanup task
	pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
		let cache = Cache {
			config: self.config.clone(),
			entries: Arc::clone(&self.entries),
			stats: Arc::clone(&self.stats),
		};

		tokio::spawn(async move {
			let mut interval = tokio::time::interval(cache.config.cleanup_interval);
			loop {
				interval.tick().await;
				cache.cleanup_expired().await;
			}
		})
	}
}

/// Specialized cache for RPC responses
pub type RpcCache = Cache<String, serde_json::Value>;

impl RpcCache {
	/// Create a new RPC cache with optimized settings
	pub fn new_rpc_cache() -> Self {
		let config = CacheConfig {
			max_entries: 5000,
			default_ttl: Duration::from_secs(30), // 30 seconds for RPC responses
			cleanup_interval: Duration::from_secs(60),
			enable_lru: true,
		};
		Self::new(config)
	}

	/// Cache a block by hash or index
	pub async fn cache_block(&self, identifier: String, block: serde_json::Value) {
		// Blocks are immutable, so cache them for longer
		self.insert_with_ttl(
			format!("block:{}", identifier),
			block,
			Duration::from_secs(3600), // 1 hour
		)
		.await;
	}

	/// Cache a transaction by hash
	pub async fn cache_transaction(&self, tx_hash: String, transaction: serde_json::Value) {
		// Transactions are immutable, so cache them for longer
		self.insert_with_ttl(
			format!("tx:{}", tx_hash),
			transaction,
			Duration::from_secs(3600), // 1 hour
		)
		.await;
	}

	/// Cache contract state
	pub async fn cache_contract_state(&self, contract_hash: String, state: serde_json::Value) {
		// Contract state can change, so shorter TTL
		self.insert_with_ttl(
			format!("contract:{}", contract_hash),
			state,
			Duration::from_secs(60), // 1 minute
		)
		.await;
	}

	/// Cache balance information
	pub async fn cache_balance(&self, address: String, balance: serde_json::Value) {
		// Balances change frequently, so very short TTL
		self.insert_with_ttl(
			format!("balance:{}", address),
			balance,
			Duration::from_secs(10), // 10 seconds
		)
		.await;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tokio::time::{sleep, Duration};

	#[tokio::test]
	async fn test_cache_basic_operations() {
		let cache = Cache::new(CacheConfig::default());

		// Test insert and get
		cache.insert("key1".to_string(), "value1".to_string()).await;
		assert_eq!(cache.get(&"key1".to_string()).await, Some("value1".to_string()));

		// Test miss
		assert_eq!(cache.get(&"nonexistent".to_string()).await, None);

		// Test remove
		assert_eq!(cache.remove(&"key1".to_string()).await, Some("value1".to_string()));
		assert_eq!(cache.get(&"key1".to_string()).await, None);
	}

	#[tokio::test]
	async fn test_cache_expiration() {
		let config = CacheConfig { default_ttl: Duration::from_millis(100), ..Default::default() };
		let cache = Cache::new(config);

		cache.insert("key1".to_string(), "value1".to_string()).await;
		assert_eq!(cache.get(&"key1".to_string()).await, Some("value1".to_string()));

		// Wait for expiration
		sleep(Duration::from_millis(150)).await;
		assert_eq!(cache.get(&"key1".to_string()).await, None);
	}

	#[tokio::test]
	async fn test_cache_stats() {
		let cache = Cache::new(CacheConfig::default());

		// Test hits and misses
		cache.insert("key1".to_string(), "value1".to_string()).await;
		cache.get(&"key1".to_string()).await; // hit
		cache.get(&"nonexistent".to_string()).await; // miss

		let stats = cache.stats().await;
		assert_eq!(stats.hits, 1);
		assert_eq!(stats.misses, 1);
		assert_eq!(stats.hit_rate(), 0.5);
	}

	#[tokio::test]
	async fn test_rpc_cache() {
		let cache = RpcCache::new_rpc_cache();

		let block_data = serde_json::json!({
			"hash": "0x1234",
			"index": 100
		});

		cache.cache_block("100".to_string(), block_data.clone()).await;
		assert_eq!(cache.get(&"block:100".to_string()).await, Some(block_data));
	}
}
