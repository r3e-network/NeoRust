//! # Metrics (Stub)
//!
//! Metrics collection infrastructure for the NeoRust SDK.
//!
//! **Status:** Stub implementation. The metrics registry, Prometheus export, and HTTP
//! server require the `prometheus` and `warp` crates which are not currently
//! dependencies. All public functions and macros in this module are no-ops.
//!
//! When the required dependencies are added, this module will expose a `/metrics`
//! endpoint serving Prometheus-format counters, gauges, and histograms for
//! transactions, RPC calls, wallet operations, blockchain state, contract
//! invocations, and system resources.

/// Initialize metrics system.
///
/// **Stub:** Does not start a metrics server (requires `prometheus` and `warp`
/// dependencies). Returns `Ok(())` unconditionally.
pub fn init(_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // no-op: requires prometheus and warp dependencies
    Ok(())
}

/// Record a transaction (stub -- currently a no-op).
pub fn record_transaction(_tx_type: &str, _network: &str, _duration: f64, _success: bool) {
    // no-op: requires prometheus dependency
}

/// Record an RPC request (stub -- currently a no-op).
pub fn record_rpc_request(_method: &str, _endpoint: &str, _duration: f64, _success: bool) {
    // no-op: requires prometheus dependency
}

/// Update blockchain metrics (stub -- currently a no-op).
pub fn update_blockchain_metrics(_network: &str, _height: u64, _synced: bool, _nodes: u64) {
    // no-op: requires prometheus dependency
}

/// Record contract invocation (stub -- currently a no-op).
pub fn record_contract_invocation(_contract: &str, _method: &str, _gas_used: f64) {
    // no-op: requires prometheus dependency
}

/// Shutdown metrics system (stub -- currently a no-op).
pub fn shutdown() {
    // no-op: requires prometheus dependency
}

/// Increment a named counter (stub -- currently a no-op).
///
/// Requires the `prometheus` dependency to function.
#[macro_export]
macro_rules! counter {
    ($name:expr, $value:expr) => {
        // no-op: requires prometheus dependency
        let _ = ($name, $value);
    };
}

/// Set a named gauge value (stub -- currently a no-op).
///
/// Requires the `prometheus` dependency to function.
#[macro_export]
macro_rules! gauge {
    ($name:expr, $value:expr) => {
        // no-op: requires prometheus dependency
        let _ = ($name, $value);
    };
}

/// Observe a named histogram value (stub -- currently a no-op).
///
/// Requires the `prometheus` dependency to function.
#[macro_export]
macro_rules! histogram {
    ($name:expr, $value:expr) => {
        // no-op: requires prometheus dependency
        let _ = ($name, $value);
    };
}
