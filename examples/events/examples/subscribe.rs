/// Neo N3 Event Subscription Example
///
/// This example demonstrates how to subscribe to real-time events (notifications)
/// from Neo N3 smart contracts using WebSocket connections and streaming.
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("📡 Neo N3 Event Subscription Example");
	println!("====================================");

	println!("\n📚 Understanding Event Subscriptions:");
	println!("   • Real-time monitoring of blockchain events");
	println!("   • WebSocket-based streaming connections");
	println!("   • Efficient processing of high-volume events");
	println!("   • Automatic reconnection and error handling");

	println!("\n🔌 Connection Types:");
	println!("   🌐 WebSocket Streams:");
	println!("     • Real-time block notifications");
	println!("     • Transaction pool monitoring");
	println!("     • Custom event subscriptions");
	println!("     • Low-latency event delivery");

	println!("\n   📡 RPC Polling:");
	println!("     • Regular block height checks");
	println!("     • Periodic application log queries");
	println!("     • Batch event processing");
	println!("     • Configurable polling intervals");

	println!("\n🎯 Subscription Patterns:");

	println!("\n   💰 Token Transfer Subscription:");
	println!("   ```rust");
	println!("   let subscription = EventSubscription::new()");
	println!("       .contract_hash(GAS_TOKEN_HASH)");
	println!("       .event_name(\"Transfer\")");
	println!("       .min_amount(1_000_000_000) // > 10 GAS");
	println!("       .subscribe();");
	println!("   ");
	println!("   while let Some(event) = subscription.next().await {");
	println!("       println!(\"Large GAS transfer: {:?}\", event);");
	println!("       update_balance_cache(&event);");
	println!("   }");
	println!("   ```");

	println!("\n   🏪 DeFi Activity Subscription:");
	println!("   ```rust");
	println!("   let defi_subscription = EventSubscription::new()");
	println!("       .contracts(vec![FLAMINGO_SWAP_HASH, BURGERNEO_HASH])");
	println!("       .events(vec![\"Swap\", \"AddLiquidity\", \"RemoveLiquidity\"])");
	println!("       .subscribe();");
	println!("   ");
	println!("   tokio::spawn(async move {");
	println!("       while let Some(event) = defi_subscription.next().await {");
	println!("           process_defi_event(event).await;");
	println!("       }");
	println!("   });");
	println!("   ```");

	println!("\n   🗳️ Governance Event Subscription:");
	println!("   ```rust");
	println!("   let governance_sub = EventSubscription::new()");
	println!("       .contract_hash(NEO_TOKEN_HASH)");
	println!("       .events(vec![\"Vote\", \"CandidateStateChanged\"])");
	println!("       .subscribe();");
	println!("   ```");

	println!("\n⚡ High-Performance Streaming:");
	println!("   🚀 Concurrent Processing:");
	println!("   ```rust");
	println!("   let (tx, mut rx) = tokio::sync::mpsc::channel(1000);");
	println!("   ");
	println!("   // Producer: Event subscription");
	println!("   tokio::spawn(async move {");
	println!("       while let Some(event) = subscription.next().await {");
	println!("           if tx.send(event).await.is_err() {");
	println!("               break; // Receiver dropped");
	println!("           }");
	println!("       }");
	println!("   });");
	println!("   ");
	println!("   // Consumer: Event processing");
	println!("   while let Some(event) = rx.recv().await {");
	println!("       tokio::spawn(process_event(event));");
	println!("   }");
	println!("   ```");

	println!("\n🔄 Reconnection and Error Handling:");
	println!("   🛠️ Robust Subscription:");
	println!("   ```rust");
	println!("   let mut retry_count = 0;");
	println!("   const MAX_RETRIES: u32 = 5;");
	println!("   ");
	println!("   loop {");
	println!("       match EventSubscription::connect().await {");
	println!("           Ok(mut subscription) => {");
	println!("               retry_count = 0;");
	println!("               while let Some(event) = subscription.next().await {");
	println!("                   match process_event(event).await {");
	println!("                       Ok(_) => {},");
	println!("                       Err(e) => log::error!(\"Processing error: {}\", e),");
	println!("                   }");
	println!("               }");
	println!("           }");
	println!("           Err(e) => {");
	println!("               retry_count += 1;");
	println!("               if retry_count > MAX_RETRIES {");
	println!("                   return Err(e);");
	println!("               }");
	println!("               let delay = Duration::from_secs(2_u64.pow(retry_count));");
	println!("               tokio::time::sleep(delay).await;");
	println!("           }");
	println!("       }");
	println!("   }");
	println!("   ```");

	println!("\n📊 Event Processing Strategies:");

	println!("\n   ⚡ Stream Processing:");
	println!("     • Process events as they arrive");
	println!("     • Low latency for time-sensitive operations");
	println!("     • Real-time UI updates");
	println!("     • Immediate alerting");

	println!("\n   📦 Batch Processing:");
	println!("     • Accumulate events in batches");
	println!("     • Process in bulk for efficiency");
	println!("     • Database bulk operations");
	println!("     • Analytics computations");

	println!("\n   🎯 Selective Processing:");
	println!("     • Filter events before processing");
	println!("     • Priority-based event handling");
	println!("     • Resource-efficient operation");
	println!("     • Reduced system load");

	println!("\n🛡️ Production Considerations:");

	println!("\n   🔒 Security:");
	println!("     • Validate all incoming event data");
	println!("     • Sanitize parameters before processing");
	println!("     • Implement rate limiting");
	println!("     • Monitor for suspicious patterns");

	println!("\n   📈 Scalability:");
	println!("     • Horizontal scaling with multiple subscribers");
	println!("     • Load balancing across event processors");
	println!("     • Event deduplication mechanisms");
	println!("     • Backpressure handling");

	println!("\n   🎯 Monitoring:");
	println!("     • Track subscription health");
	println!("     • Monitor event processing latency");
	println!("     • Alert on connection failures");
	println!("     • Measure throughput metrics");

	println!("\n💼 Real-World Applications:");

	println!("\n   🏦 Financial Applications:");
	println!("     • Real-time trading alerts");
	println!("     • Portfolio balance updates");
	println!("     • Risk monitoring systems");
	println!("     • Compliance reporting");

	println!("\n   🎮 Gaming Platforms:");
	println!("     • Item transfer notifications");
	println!("     • Achievement unlocks");
	println!("     • Marketplace activities");
	println!("     • Tournament updates");

	println!("\n   🏪 E-commerce DApps:");
	println!("     • Order status updates");
	println!("     • Payment confirmations");
	println!("     • Inventory changes");
	println!("     • Customer notifications");

	// Simulate some subscription activity
	println!("\n🔄 Simulating Event Subscription Activity...");
	for i in 1..=3 {
		println!("   📡 Simulated event {} received at {:?}", i, std::time::Instant::now());
		tokio::time::sleep(Duration::from_millis(500)).await;
	}

	println!("\n⚠️ Common Subscription Pitfalls:");
	println!("   • Not handling connection drops gracefully");
	println!("   • Missing event deduplication");
	println!("   • Blocking event processing loops");
	println!("   • Insufficient error handling");
	println!("   • Memory leaks from unclosed subscriptions");

	println!("\n🚀 For advanced subscription patterns:");
	println!("   • Neo N3 WebSocket documentation");
	println!("   • Event streaming architectures");
	println!("   • Production deployment guides");

	println!("\n🎉 Event subscription concepts covered!");
	println!("💡 Build robust, scalable event-driven applications");
	println!("   with these real-time subscription patterns.");

	Ok(())
}
