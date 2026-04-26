//! # Metrics
//!
//! In-process metrics collection for the NeoRust SDK. Applications can export
//! snapshots to Prometheus, OpenTelemetry, logs, or their own monitoring stack.

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

static METRICS: OnceCell<Arc<MetricsRegistry>> = OnceCell::new();

/// Snapshot of the current metrics registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricsSnapshot {
	pub counters: HashMap<String, f64>,
	pub gauges: HashMap<String, f64>,
	pub histograms: HashMap<String, Vec<f64>>,
}

#[derive(Debug, Default)]
struct MetricsRegistry {
	counters: RwLock<HashMap<String, f64>>,
	gauges: RwLock<HashMap<String, f64>>,
	histograms: RwLock<HashMap<String, Vec<f64>>>,
}

/// Initialize metrics collection.
pub fn init(port: u16) -> Result<(), Box<dyn std::error::Error>> {
	let registry = Arc::new(MetricsRegistry::default());
	METRICS.set(registry).map_err(|_| "Metrics already initialized")?;
	set_gauge("metrics.configured_port", port as f64);
	Ok(())
}

fn with_registry<F>(f: F)
where
	F: FnOnce(&MetricsRegistry),
{
	if let Some(registry) = METRICS.get() {
		f(registry);
	}
}

/// Increment a named counter.
pub fn increment_counter(name: &str, value: f64) {
	with_registry(|registry| {
		let mut counters = registry.counters.write().unwrap_or_else(|e| e.into_inner());
		*counters.entry(name.to_string()).or_insert(0.0) += value;
	});
}

/// Set a named gauge.
pub fn set_gauge(name: &str, value: f64) {
	with_registry(|registry| {
		let mut gauges = registry.gauges.write().unwrap_or_else(|e| e.into_inner());
		gauges.insert(name.to_string(), value);
	});
}

/// Observe a histogram value.
pub fn observe_histogram(name: &str, value: f64) {
	with_registry(|registry| {
		let mut histograms = registry.histograms.write().unwrap_or_else(|e| e.into_inner());
		histograms.entry(name.to_string()).or_default().push(value);
	});
}

/// Return a point-in-time snapshot of all metrics.
pub fn snapshot() -> MetricsSnapshot {
	let Some(registry) = METRICS.get() else {
		return MetricsSnapshot::default();
	};

	MetricsSnapshot {
		counters: registry.counters.read().unwrap_or_else(|e| e.into_inner()).clone(),
		gauges: registry.gauges.read().unwrap_or_else(|e| e.into_inner()).clone(),
		histograms: registry.histograms.read().unwrap_or_else(|e| e.into_inner()).clone(),
	}
}

/// Record a transaction.
pub fn record_transaction(tx_type: &str, network: &str, duration: f64, success: bool) {
	let outcome = if success { "success" } else { "failure" };
	increment_counter(&format!("transactions.{network}.{tx_type}.{outcome}"), 1.0);
	observe_histogram(&format!("transactions.{network}.{tx_type}.duration_seconds"), duration);
}

/// Record an RPC request.
pub fn record_rpc_request(method: &str, endpoint: &str, duration: f64, success: bool) {
	let outcome = if success { "success" } else { "failure" };
	increment_counter(&format!("rpc.{endpoint}.{method}.{outcome}"), 1.0);
	observe_histogram(&format!("rpc.{endpoint}.{method}.duration_seconds"), duration);
}

/// Update blockchain metrics.
pub fn update_blockchain_metrics(network: &str, height: u64, synced: bool, nodes: u64) {
	set_gauge(&format!("blockchain.{network}.height"), height as f64);
	set_gauge(&format!("blockchain.{network}.nodes"), nodes as f64);
	set_gauge(&format!("blockchain.{network}.synced"), if synced { 1.0 } else { 0.0 });
}

/// Record contract invocation.
pub fn record_contract_invocation(contract: &str, method: &str, gas_used: f64) {
	increment_counter(&format!("contract.{contract}.{method}.invocations"), 1.0);
	observe_histogram(&format!("contract.{contract}.{method}.gas"), gas_used);
}

/// Shutdown metrics collection.
pub fn shutdown() {
	if let Some(registry) = METRICS.get() {
		registry.counters.write().unwrap_or_else(|e| e.into_inner()).clear();
		registry.gauges.write().unwrap_or_else(|e| e.into_inner()).clear();
		registry.histograms.write().unwrap_or_else(|e| e.into_inner()).clear();
	}
}

/// Increment a named counter.
#[macro_export]
macro_rules! counter {
	($name:expr, $value:expr) => {
		$crate::monitoring::metrics::increment_counter($name, $value as f64);
	};
}

/// Set a named gauge value.
#[macro_export]
macro_rules! gauge {
	($name:expr, $value:expr) => {
		$crate::monitoring::metrics::set_gauge($name, $value as f64);
	};
}

/// Observe a named histogram value.
#[macro_export]
macro_rules! histogram {
	($name:expr, $value:expr) => {
		$crate::monitoring::metrics::observe_histogram($name, $value as f64);
	};
}
