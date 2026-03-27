//! # Distributed Tracing (Stub)
//!
//! Distributed tracing infrastructure for the NeoRust SDK.
//!
//! **Status:** Stub implementation. The OpenTelemetry integration requires the
//! `opentelemetry`, `opentelemetry-otlp`, `opentelemetry-http`, and
//! `tracing-opentelemetry` crates which are not currently dependencies.
//!
//! All public functions in this module are no-ops. The convenience macros
//! (`trace_transaction!`, `trace_rpc!`, `trace_contract!`) execute their body
//! but do not create OpenTelemetry spans.
//!
//! When the required dependencies are added, this module will provide OTLP export
//! to backends such as Jaeger, with automatic context propagation via HTTP headers.

/// Initialize tracing system.
///
/// **Stub:** Does not configure an OpenTelemetry exporter or tracing subscriber
/// (requires `opentelemetry` and related dependencies). Returns `Ok(())`
/// unconditionally.
pub fn init(_endpoint: &str, _log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
	// no-op: requires opentelemetry, opentelemetry-otlp, tracing-opentelemetry dependencies
	Ok(())
}

/// Add event to current span (stub -- currently a no-op).
pub fn add_event(_name: &str, _attributes: Vec<(&str, String)>) {
	// no-op: requires opentelemetry dependency
}

/// Set span status (stub -- currently a no-op).
pub fn set_status(_success: bool, _message: Option<&str>) {
	// no-op: requires opentelemetry dependency
}

/// Record an error in the current span (stub -- currently a no-op).
pub fn record_error(_error: &dyn std::error::Error) {
	// no-op: requires opentelemetry dependency
}

/// Shutdown tracing system (stub -- currently a no-op).
pub fn shutdown() {
	// no-op: requires opentelemetry dependency
}

/// Execute a block in a transaction tracing context (stub -- runs the body without
/// creating an OpenTelemetry span).
///
/// Requires `opentelemetry` and `tracing-opentelemetry` to produce real spans.
#[macro_export]
macro_rules! trace_transaction {
	($tx_type:expr, $network:expr, $body:block) => {{
		// no-op span: requires opentelemetry dependency
		let _ = ($tx_type, $network);
		$body
	}};
}

/// Execute a block in an RPC tracing context (stub -- runs the body without
/// creating an OpenTelemetry span).
///
/// Requires `opentelemetry` and `tracing-opentelemetry` to produce real spans.
#[macro_export]
macro_rules! trace_rpc {
	($method:expr, $endpoint:expr, $body:block) => {{
		// no-op span: requires opentelemetry dependency
		let _ = ($method, $endpoint);
		$body
	}};
}

/// Execute a block in a contract tracing context (stub -- runs the body without
/// creating an OpenTelemetry span).
///
/// Requires `opentelemetry` and `tracing-opentelemetry` to produce real spans.
#[macro_export]
macro_rules! trace_contract {
	($contract:expr, $operation:expr, $body:block) => {{
		// no-op span: requires opentelemetry dependency
		let _ = ($contract, $operation);
		$body
	}};
}
