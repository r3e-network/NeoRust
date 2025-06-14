use assert_cmd::prelude::*;
use hex;
use predicates::prelude::*;
use sha2::{Digest, Sha256};
use std::{
	fs,
	io::Write,
	path::PathBuf,
	process::{Command, Output},
};
use tempfile::{NamedTempFile, TempDir};

pub struct CliTest {
	/// Temporary directory for test files
	pub temp_dir: TempDir,
	/// Path to the cli binary
	pub binary_path: PathBuf,
}

impl CliTest {
	/// Create a new test environment
	pub fn new() -> Self {
		let bin_path = std::env::current_dir().unwrap();
		let temp_dir = TempDir::new().expect("Failed to create temp directory");

		Self { temp_dir, binary_path: bin_path }
	}

	/// Run a CLI command with the given arguments
	pub fn run(&self, args: &[&str]) -> Output {
		Command::new("cargo")
			.arg("run")
			.arg("--")
			.args(args)
			.current_dir(&self.binary_path)
			.output()
			.expect("Failed to execute command")
	}

	/// Alias for run to match what's used in tests
	pub fn run_command(&self, args: &[&str]) -> Output {
		self.run(args)
	}

	/// Run a CLI command with input
	pub fn run_with_input(&self, args: &[&str], input: &str) -> Output {
		let mut child = Command::new("cargo")
			.arg("run")
			.arg("--")
			.args(args)
			.current_dir(&self.binary_path)
			.stdin(std::process::Stdio::piped())
			.stdout(std::process::Stdio::piped())
			.stderr(std::process::Stdio::piped())
			.spawn()
			.expect("Failed to spawn CLI command");

		if let Some(mut stdin) = child.stdin.take() {
			stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
		}

		child.wait_with_output().expect("Failed to wait for command")
	}

	/// Create a temporary file with the given content
	pub fn create_temp_file(&self, content: &str) -> PathBuf {
		let mut file = NamedTempFile::new_in(&self.temp_dir).unwrap();
		file.write_all(content.as_bytes()).unwrap();
		let path = file.into_temp_path();
		let path_buf = path.to_path_buf();
		path.keep().unwrap();
		path_buf
	}

	/// Create a config file for testing
	pub fn create_config_file(&self, content: &str) -> PathBuf {
		self.create_temp_file(content)
	}
}

/// Helper function to assert that command was successful
pub fn assert_success(output: &Output) {
	assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
}

/// Helper function to assert command output contains a string
pub fn assert_output_contains(output: &Output, expected: &str) {
	let stdout = String::from_utf8_lossy(&output.stdout);
	assert!(
		stdout.contains(expected),
		"Expected output to contain '{}', but got:\n{}",
		expected,
		stdout
	);
}

/// Helper function to assert command output matches a regular expression
pub fn assert_output_matches(output: &Output, pattern: &str) {
	let stdout = String::from_utf8_lossy(&output.stdout);
	let re = regex::Regex::new(pattern).unwrap();
	assert!(re.is_match(&stdout), "Expected output to match '{}', but got:\n{}", pattern, stdout);
}

/// Helper to create a script hash from a string
pub fn script_hash_from_string(s: &str) -> String {
	// Use SHA256 for proper hashing instead of a simple sum
	let mut hasher = Sha256::new();
	hasher.update(s.as_bytes());
	let result = hasher.finalize();
	format!("0x{}", hex::encode(result))
}

/// Compute a simple hash of a string for testing purposes
pub fn simple_hash(s: &str) -> String {
	// Use SHA256 for proper hashing instead of a simple sum
	let mut hasher = Sha256::new();
	hasher.update(s.as_bytes());
	let result = hasher.finalize();
	format!("0x{}", hex::encode(result))
}
