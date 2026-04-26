//! # Distributed Tracing
//!
//! Lightweight tracing integration built on the SDK's existing `tracing`
//! dependency. Applications that need OTLP export can layer their own subscriber
//! on top; these helpers create spans and structured events without adding a web
//! or exporter dependency to the SDK.

use tracing::Level;

/// Initialize a default tracing subscriber.
pub fn init(endpoint: &str, log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
	let level = match log_level.to_ascii_lowercase().as_str() {
		"trace" => Level::TRACE,
		"debug" => Level::DEBUG,
		"warn" => Level::WARN,
		"error" => Level::ERROR,
		_ => Level::INFO,
	};

	tracing_subscriber::fmt()
		.with_max_level(level)
		.try_init()
		.map_err(|e| format!("Failed to initialize tracing subscriber: {e}"))?;

	if !endpoint.is_empty() {
		tracing::info!(endpoint = endpoint, "Tracing initialized");
	} else {
		tracing::info!("Tracing initialized");
	}
	Ok(())
}

/// Add event to current span.
pub fn add_event(name: &str, attributes: Vec<(&str, String)>) {
	let details = attributes
		.into_iter()
		.map(|(key, value)| format!("{key}={value}"))
		.collect::<Vec<_>>()
		.join(" ");
	tracing::info!(event = name, details = details);
}

/// Set span status.
pub fn set_status(success: bool, message: Option<&str>) {
	if success {
		tracing::info!(message = message.unwrap_or("ok"), "Span status");
	} else {
		tracing::warn!(message = message.unwrap_or("failed"), "Span status");
	}
}

/// Record an error in the current span.
pub fn record_error(error: &dyn std::error::Error) {
	tracing::error!(error = %error, "Span error");
}

/// Flush tracing resources owned by this module.
pub fn shutdown() {
	tracing::debug!("Tracing shutdown requested");
}

/// Execute a block in a transaction tracing context.
#[macro_export]
macro_rules! trace_transaction {
	($tx_type:expr, $network:expr, $body:block) => {{
		let span = tracing::info_span!("neo.transaction", tx_type = $tx_type, network = $network);
		let _guard = span.enter();
		$body
	}};
}

/// Execute a block in an RPC tracing context.
#[macro_export]
macro_rules! trace_rpc {
	($method:expr, $endpoint:expr, $body:block) => {{
		let span = tracing::info_span!("neo.rpc", method = $method, endpoint = $endpoint);
		let _guard = span.enter();
		$body
	}};
}

/// Execute a block in a contract tracing context.
#[macro_export]
macro_rules! trace_contract {
	($contract:expr, $operation:expr, $body:block) => {{
		let span =
			tracing::info_span!("neo.contract", contract = $contract, operation = $operation);
		let _guard = span.enter();
		$body
	}};
}
