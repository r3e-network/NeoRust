#![allow(dead_code)]
use crate::errors::CliError;
use std::{
	net::{TcpStream, ToSocketAddrs},
	path::Path,
	time::Duration,
};
use url::Url;

/// Validates a container ID format
/// Returns Ok if valid, Error with message otherwise
pub fn validate_container_id(container_id: &str) -> Result<(), CliError> {
	if container_id.len() < 8 {
		return Err(CliError::InvalidInput(
			"Container ID must be at least 8 characters".to_string(),
		));
	}
	if container_id.len() > 64 {
		return Err(CliError::InvalidInput(
			"Container ID appears invalid (too long for NeoFS format)".to_string(),
		));
	}
	if !container_id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
		return Err(CliError::InvalidInput(
			"Container ID must contain only ASCII alphanumeric characters".to_string(),
		));
	}
	Ok(())
}

/// Validates a file path exists
pub fn validate_file_path(path: &Path) -> Result<(), CliError> {
	if !path.exists() {
		return Err(CliError::FileSystem(format!("File not found: {path}", path = path.display())));
	}
	if !path.is_file() {
		return Err(CliError::FileSystem(format!(
			"Path is not a file: {path}",
			path = path.display()
		)));
	}
	Ok(())
}

/// Validates a directory path exists
pub fn validate_directory_path(path: &Path) -> Result<(), CliError> {
	if !path.exists() {
		return Err(CliError::FileSystem(format!(
			"Directory not found: {path}",
			path = path.display()
		)));
	}
	if !path.is_dir() {
		return Err(CliError::FileSystem(format!(
			"Path is not a directory: {path}",
			path = path.display()
		)));
	}
	Ok(())
}

/// Formats file size in human-readable format
pub fn format_size(size: u64) -> String {
	const KB: u64 = 1024;
	const MB: u64 = KB * 1024;
	const GB: u64 = MB * 1024;

	if size < KB {
		format!("{size} B")
	} else if size < MB {
		format!("{:.2} KB", size as f64 / KB as f64)
	} else if size < GB {
		format!("{:.2} MB", size as f64 / MB as f64)
	} else {
		format!("{:.2} GB", size as f64 / GB as f64)
	}
}

/// Validates an endpoint URL
pub fn validate_endpoint(endpoint: &str) -> Result<(), CliError> {
	let parsed = Url::parse(endpoint)
		.map_err(|e| CliError::InvalidInput(format!("Invalid endpoint URL: {e}")))?;
	if parsed.scheme() != "http" && parsed.scheme() != "https" {
		return Err(CliError::InvalidInput(
			"Endpoint must start with http:// or https://".to_string(),
		));
	}
	if parsed.host_str().is_none() {
		return Err(CliError::InvalidInput("Endpoint URL must include a host".to_string()));
	}
	Ok(())
}

/// Extracts storage node info from an endpoint
pub fn get_node_info(endpoint: &str) -> Result<String, CliError> {
	validate_endpoint(endpoint)?;
	let parsed = Url::parse(endpoint)
		.map_err(|e| CliError::InvalidInput(format!("Invalid endpoint URL: {e}")))?;
	let host = parsed.host_str().unwrap_or("<unknown>");
	let port = parsed
		.port_or_known_default()
		.map(|port| port.to_string())
		.unwrap_or_else(|| "<default>".to_string());
	Ok(format!("scheme={} host={} port={}", parsed.scheme(), host, port))
}

/// Checks if an endpoint is available
pub fn check_endpoint_availability(endpoint: &str) -> Result<bool, CliError> {
	validate_endpoint(endpoint)?;
	let parsed = Url::parse(endpoint)
		.map_err(|e| CliError::InvalidInput(format!("Invalid endpoint URL: {e}")))?;
	let host = parsed
		.host_str()
		.ok_or_else(|| CliError::InvalidInput("Endpoint URL must include a host".to_string()))?;
	let port = parsed.port_or_known_default().ok_or_else(|| {
		CliError::InvalidInput("Endpoint URL must include a port or known scheme".to_string())
	})?;
	let Some(socket_addr) = (host, port)
		.to_socket_addrs()
		.map_err(|e| CliError::Network(format!("Failed to resolve endpoint host: {e}")))?
		.next()
	else {
		return Err(CliError::Network("Endpoint host did not resolve to any address".to_string()));
	};

	Ok(TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)).is_ok())
}

/// Formats container/object permissions
pub fn format_permissions(is_public_read: bool, is_public_write: bool) -> String {
	match (is_public_read, is_public_write) {
		(true, true) => "Public read/write".to_string(),
		(true, false) => "Public read only".to_string(),
		(false, true) => "Public write only".to_string(),
		(false, false) => "Private".to_string(),
	}
}
