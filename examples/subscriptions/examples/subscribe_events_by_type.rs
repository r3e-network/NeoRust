use neo3::prelude::*;
use std::{collections::HashMap, time::Duration};
use tokio::time::{interval, timeout};

/// Neo N3 Event Subscription by Type Example
///
/// This example demonstrates how to monitor specific contract events on the Neo N3 blockchain.
/// Since Neo N3 doesn't have native WebSocket event subscriptions like Ethereum, we implement
/// polling-based event monitoring with filtering by event type and contract address.
#[tokio::main]
async fn main() -> eyre::Result<()> {
	println!("🎯 Neo N3 Event Subscription by Type Example");
	println!("============================================");

	// 1. Connect to Neo N3 TestNet
	println!("\n1. Connecting to Neo N3 TestNet...");
	let provider = HttpProvider::new("https://testnet1.neo.coz.io:443/")?;
	let client = RpcClient::new(provider);
	println!("   ✅ Connected successfully");

	// 2. Define contract addresses to monitor
	println!("\n2. Setting up contract monitoring...");
	let contracts_to_monitor = vec![
		ContractMonitor {
			name: "NEO Token".to_string(),
			address: ScriptHash::neo(),
			events_of_interest: vec!["Transfer".to_string()],
		},
		ContractMonitor {
			name: "GAS Token".to_string(),
			address: ScriptHash::gas(),
			events_of_interest: vec!["Transfer".to_string()],
		},
	];

	for contract in &contracts_to_monitor {
		println!("   📋 Monitoring {}: 0x{}", contract.name, hex::encode(contract.address.0));
		println!("     Events: {:?}", contract.events_of_interest);
	}

	// 3. Initialize event monitor
	println!("\n3. Initializing event monitor...");
	let mut event_monitor = EventMonitor::new(&client, contracts_to_monitor).await?;
	println!("   ✅ Event monitor initialized");

	// 4. Start event subscription (polling-based)
	println!("\n4. Starting event subscription (monitoring for 60 seconds)...");
	println!("   🔄 Polling for new events every 10 seconds...");

	let mut poll_interval = interval(Duration::from_secs(10));
	let mut events_found = 0;
	let max_events = 10;

	let subscription_result = timeout(Duration::from_secs(65), async {
		loop {
			poll_interval.tick().await;

			match event_monitor.check_for_new_events().await {
				Ok(new_events) => {
					for event in new_events {
						print_event_details(&event);
						events_found += 1;

						if events_found >= max_events {
							return Ok::<(), eyre::Report>(());
						}
					}

					if events_found == 0 {
						println!("   🔍 No new events found in this cycle...");
					}
				},
				Err(e) => {
					println!("   ⚠️  Error checking for events: {}", e);
				},
			}
		}
	})
	.await;

	match subscription_result {
		Ok(_) => println!("\n   ✅ Event subscription completed successfully"),
		Err(_) => println!("\n   ⏰ Event subscription timed out (demonstration ended)"),
	}

	// 5. Display subscription statistics
	println!("\n5. Event Subscription Statistics:");
	let stats = event_monitor.get_statistics();
	println!("   📊 Events monitored: {}", stats.events_processed);
	println!("   🔄 Polling cycles: {}", stats.poll_cycles);
	println!("   📈 Contracts monitored: {}", stats.contracts_monitored);
	println!("   💹 Unique event types seen: {}", stats.unique_event_types.len());

	if !stats.unique_event_types.is_empty() {
		println!("   📋 Event types found: {:?}", stats.unique_event_types);
	}

	// 6. Demonstrate advanced event filtering
	println!("\n6. Advanced event filtering capabilities:");
	demonstrate_event_filtering(&client).await?;

	// 7. Best practices for event monitoring
	println!("\n7. 💡 Neo N3 Event Monitoring Best Practices:");
	println!("   ✅ Use application logs to track contract notifications");
	println!("   ✅ Filter events by contract address and event name");
	println!("   ✅ Implement efficient polling intervals");
	println!("   ✅ Cache processed transactions to avoid reprocessing");
	println!("   ✅ Use block confirmations for critical event handling");
	println!("   ✅ Implement retry logic for network failures");
	println!("   ✅ Monitor multiple contracts simultaneously");

	println!("\n🎉 Event subscription example completed!");
	println!("💡 This demonstrates event monitoring and filtering for Neo N3.");

	Ok(())
}

/// Contract monitoring configuration
#[derive(Debug, Clone)]
struct ContractMonitor {
	name: String,
	address: ScriptHash,
	events_of_interest: Vec<String>,
}

/// Event information structure
#[derive(Debug, Clone)]
struct EventInfo {
	contract_address: ScriptHash,
	contract_name: String,
	event_name: String,
	block_index: u64,
	transaction_hash: String,
	event_data: serde_json::Value,
	timestamp: u64,
}

/// Event monitoring statistics
#[derive(Debug)]
struct EventStatistics {
	events_processed: u32,
	poll_cycles: u32,
	contracts_monitored: usize,
	unique_event_types: std::collections::HashSet<String>,
}

impl EventStatistics {
	fn new(contracts_monitored: usize) -> Self {
		Self {
			events_processed: 0,
			poll_cycles: 0,
			contracts_monitored,
			unique_event_types: std::collections::HashSet::new(),
		}
	}
}

/// Event monitor for Neo N3
struct EventMonitor<'a> {
	client: &'a RpcClient<HttpProvider>,
	contracts: Vec<ContractMonitor>,
	last_processed_block: u64,
	processed_transactions: std::collections::HashSet<String>,
	statistics: EventStatistics,
}

impl<'a> EventMonitor<'a> {
	async fn new(
		client: &'a RpcClient<HttpProvider>,
		contracts: Vec<ContractMonitor>,
	) -> eyre::Result<Self> {
		let current_block = client.get_block_count().await?;
		let contracts_count = contracts.len();

		Ok(Self {
			client,
			contracts,
			last_processed_block: current_block,
			processed_transactions: std::collections::HashSet::new(),
			statistics: EventStatistics::new(contracts_count),
		})
	}

	async fn check_for_new_events(&mut self) -> eyre::Result<Vec<EventInfo>> {
		self.statistics.poll_cycles += 1;

		let current_block = self.client.get_block_count().await?;
		let mut new_events = Vec::new();

		// Process new blocks
		for block_index in (self.last_processed_block + 1)..=current_block {
			if let Ok(block_data) = self.client.get_block(serde_json::json!(block_index)).await {
				let block_events = self.extract_events_from_block(&block_data, block_index).await;
				new_events.extend(block_events);
			}
		}

		self.last_processed_block = current_block;
		Ok(new_events)
	}

	async fn extract_events_from_block(
		&mut self,
		block_data: &serde_json::Value,
		block_index: u64,
	) -> Vec<EventInfo> {
		let mut events = Vec::new();

		if let Some(transactions) = block_data.get("tx").and_then(|tx| tx.as_array()) {
			for tx in transactions {
				if let Some(tx_hash) = tx.get("hash").and_then(|h| h.as_str()) {
					// Skip if already processed
					if self.processed_transactions.contains(tx_hash) {
						continue;
					}

					// Get application log for this transaction
					if let Ok(app_log) = self.client.get_application_log(tx_hash.to_string()).await
					{
						let tx_events = self.parse_application_log(&app_log, block_index, tx_hash);
						events.extend(tx_events);
						self.processed_transactions.insert(tx_hash.to_string());
					}
				}
			}
		}

		events
	}

	fn parse_application_log(
		&mut self,
		app_log: &serde_json::Value,
		block_index: u64,
		tx_hash: &str,
	) -> Vec<EventInfo> {
		let mut events = Vec::new();

		if let Some(executions) = app_log.get("executions").and_then(|e| e.as_array()) {
			for execution in executions {
				if let Some(notifications) =
					execution.get("notifications").and_then(|n| n.as_array())
				{
					for notification in notifications {
						if let Some(event) =
							self.parse_notification(notification, block_index, tx_hash)
						{
							events.push(event);
							self.statistics.events_processed += 1;
							self.statistics.unique_event_types.insert(event.event_name.clone());
						}
					}
				}
			}
		}

		events
	}

	fn parse_notification(
		&self,
		notification: &serde_json::Value,
		block_index: u64,
		tx_hash: &str,
	) -> Option<EventInfo> {
		let contract_hash_str = notification.get("contract")?.as_str()?;
		let event_name = notification.get("eventname")?.as_str()?.to_string();

		// Convert contract hash string to ScriptHash
		let contract_hash = ScriptHash::from_str(contract_hash_str).ok()?;

		// Check if this contract is being monitored
		let contract_monitor = self.contracts.iter().find(|c| c.address == contract_hash)?;

		// Check if this event type is of interest
		if !contract_monitor.events_of_interest.contains(&event_name) {
			return None;
		}

		let state = notification.get("state").cloned().unwrap_or(serde_json::Value::Null);

		Some(EventInfo {
			contract_address: contract_hash,
			contract_name: contract_monitor.name.clone(),
			event_name,
			block_index,
			transaction_hash: tx_hash.to_string(),
			event_data: state,
			timestamp: chrono::Utc::now().timestamp() as u64,
		})
	}

	fn get_statistics(&self) -> &EventStatistics {
		&self.statistics
	}
}

/// Print detailed event information
fn print_event_details(event: &EventInfo) {
	println!("\n   🎯 New Event Detected:");
	println!(
		"     Contract: {} (0x{})",
		event.contract_name,
		hex::encode(event.contract_address.0)
	);
	println!("     Event: {}", event.event_name);
	println!("     Block: {}", event.block_index);
	println!("     Transaction: {}", event.transaction_hash);

	if !event.event_data.is_null() {
		println!(
			"     Data: {}",
			serde_json::to_string_pretty(&event.event_data)
				.unwrap_or_else(|_| "Invalid JSON".to_string())
		);
	}
}

/// Demonstrate advanced event filtering capabilities
async fn demonstrate_event_filtering(client: &RpcClient<HttpProvider>) -> eyre::Result<()> {
	println!("   🔍 Advanced Event Filtering Demo:");

	// Get a recent transaction to analyze
	let block_count = client.get_block_count().await?;
	let recent_block_index = block_count.saturating_sub(5);

	if let Ok(block_data) = client.get_block(serde_json::json!(recent_block_index)).await {
		if let Some(transactions) = block_data.get("tx").and_then(|tx| tx.as_array()) {
			println!(
				"     📋 Analyzing recent block {} with {} transactions",
				recent_block_index,
				transactions.len()
			);

			let mut event_types = std::collections::HashMap::new();
			let mut contract_activity = std::collections::HashMap::new();

			for tx in transactions.iter().take(3) {
				// Analyze first 3 transactions
				if let Some(tx_hash) = tx.get("hash").and_then(|h| h.as_str()) {
					if let Ok(app_log) = client.get_application_log(tx_hash.to_string()).await {
						analyze_transaction_events(
							&app_log,
							&mut event_types,
							&mut contract_activity,
						);
					}
				}
			}

			if !event_types.is_empty() {
				println!("     📊 Event types found: {:?}", event_types);
			}

			if !contract_activity.is_empty() {
				println!("     🏢 Contract activity: {:?}", contract_activity);
			}
		}
	}

	Ok(())
}

/// Analyze events in a transaction
fn analyze_transaction_events(
	app_log: &serde_json::Value,
	event_types: &mut HashMap<String, u32>,
	contract_activity: &mut HashMap<String, u32>,
) {
	if let Some(executions) = app_log.get("executions").and_then(|e| e.as_array()) {
		for execution in executions {
			if let Some(notifications) = execution.get("notifications").and_then(|n| n.as_array()) {
				for notification in notifications {
					if let Some(event_name) = notification.get("eventname").and_then(|e| e.as_str())
					{
						*event_types.entry(event_name.to_string()).or_insert(0) += 1;
					}

					if let Some(contract) = notification.get("contract").and_then(|c| c.as_str()) {
						*contract_activity.entry(contract.to_string()).or_insert(0) += 1;
					}
				}
			}
		}
	}
}
