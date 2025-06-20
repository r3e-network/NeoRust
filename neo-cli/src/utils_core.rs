// Core utilities for CLI operations
use colored::*;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
	io::{self, Write},
	time::Duration,
};

/// Print an informational message with icon
pub fn print_info(message: &str) {
	println!("{} {}", "ℹ".bright_blue(), message);
}

/// Print a success message with icon
pub fn print_success(message: &str) {
	println!("{} {}", "✅".bright_green(), message.bright_green());
}

/// Print an error message with icon
pub fn print_error(message: &str) {
	eprintln!("{} {}", "❌".bright_red(), message.bright_red());
}

/// Print a warning message with icon
pub fn print_warning(message: &str) {
	println!("{} {}", "⚠️".bright_yellow(), message.bright_yellow());
}

/// Print a debug message (only in verbose mode)
pub fn print_debug(message: &str) {
	if std::env::var("RUST_LOG").unwrap_or_default().contains("debug") {
		println!("{} {}", "🔍".bright_magenta(), message.bright_black());
	}
}

/// Create a beautiful table for displaying data
pub fn create_table() -> Table {
	let mut table = Table::new();
	table
		.load_preset(UTF8_FULL)
		.set_content_arrangement(ContentArrangement::Dynamic)
		.set_header(vec![
			Cell::new("Field").add_attribute(Attribute::Bold).fg(Color::Cyan),
			Cell::new("Value").add_attribute(Attribute::Bold).fg(Color::Cyan),
		]);
	table
}

/// Create a progress bar with custom style
pub fn create_progress_bar(len: u64, message: &str) -> ProgressBar {
	let pb = ProgressBar::new(len);
	pb.set_style(
		ProgressStyle::default_bar()
			.template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
			.unwrap()
			.progress_chars("█▉▊▋▌▍▎▏ "),
	);
	pb.set_message(message.to_string());
	pb
}

/// Create an indeterminate spinner
pub fn create_spinner(message: &str) -> ProgressBar {
	let pb = ProgressBar::new_spinner();
	pb.set_style(
		ProgressStyle::default_spinner()
			.template("{spinner:.green} {msg}")
			.unwrap()
			.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
	);
	pb.set_message(message.to_string());
	pb.enable_steady_tick(Duration::from_millis(120));
	pb
}

/// Prompt user for yes/no confirmation with custom message
pub fn prompt_yes_no(message: &str) -> Result<bool, io::Error> {
	Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt(message)
		.default(false)
		.interact()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Prompt user for password input
pub fn prompt_password(message: &str) -> Result<String, io::Error> {
	Password::with_theme(&ColorfulTheme::default())
		.with_prompt(message)
		.interact()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Prompt user for text input
pub fn prompt_input(message: &str) -> Result<String, io::Error> {
	Input::with_theme(&ColorfulTheme::default())
		.with_prompt(message)
		.interact_text()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Prompt user for text input with default value
pub fn prompt_input_with_default(message: &str, default: &str) -> Result<String, io::Error> {
	Input::with_theme(&ColorfulTheme::default())
		.with_prompt(message)
		.default(default.to_string())
		.interact_text()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Prompt user to select from a list of options
pub fn prompt_select(message: &str, options: &[&str]) -> Result<usize, io::Error> {
	Select::with_theme(&ColorfulTheme::default())
		.with_prompt(message)
		.items(options)
		.default(0)
		.interact()
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Display a formatted key-value pair
pub fn display_key_value(key: &str, value: &str) {
	println!("{}: {}", key.bright_cyan().bold(), value.bright_white());
}

/// Display a formatted key-value pair with custom colors
pub fn display_key_value_colored(key: &str, value: &str, key_color: Color, value_color: Color) {
	let key_colored = match key_color {
		Color::Red => key.bright_red(),
		Color::Green => key.bright_green(),
		Color::Blue => key.bright_blue(),
		Color::Yellow => key.bright_yellow(),
		Color::Magenta => key.bright_magenta(),
		Color::Cyan => key.bright_cyan(),
		_ => key.bright_white(),
	};

	let value_colored = match value_color {
		Color::Red => value.bright_red(),
		Color::Green => value.bright_green(),
		Color::Blue => value.bright_blue(),
		Color::Yellow => value.bright_yellow(),
		Color::Magenta => value.bright_magenta(),
		Color::Cyan => value.bright_cyan(),
		_ => value.bright_white(),
	};

	println!("{}: {}", key_colored.bold(), value_colored);
}

/// Format a large number with thousands separators
pub fn format_number(num: u64) -> String {
	num.to_string()
		.as_bytes()
		.rchunks(3)
		.rev()
		.map(std::str::from_utf8)
		.collect::<Result<Vec<&str>, _>>()
		.unwrap()
		.join(",")
}

/// Format bytes in human-readable format
pub fn format_bytes(bytes: u64) -> String {
	const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
	let mut size = bytes as f64;
	let mut unit_index = 0;

	while size >= 1024.0 && unit_index < UNITS.len() - 1 {
		size /= 1024.0;
		unit_index += 1;
	}

	if unit_index == 0 {
		format!("{} {}", size as u64, UNITS[unit_index])
	} else {
		format!("{:.2} {}", size, UNITS[unit_index])
	}
}

/// Format duration in human-readable format
pub fn format_duration(seconds: u64) -> String {
	let days = seconds / 86400;
	let hours = (seconds % 86400) / 3600;
	let minutes = (seconds % 3600) / 60;
	let secs = seconds % 60;

	if days > 0 {
		format!("{}d {}h {}m {}s", days, hours, minutes, secs)
	} else if hours > 0 {
		format!("{}h {}m {}s", hours, minutes, secs)
	} else if minutes > 0 {
		format!("{}m {}s", minutes, secs)
	} else {
		format!("{}s", secs)
	}
}

/// Print a section header
pub fn print_section_header(title: &str) {
	println!();
	println!("{}", title.bright_green().bold().underline());
	println!("{}", "─".repeat(title.len()).bright_green());
}

/// Print a subsection header
pub fn print_subsection_header(title: &str) {
	println!();
	println!("{}", title.bright_cyan().bold());
}

/// Clear the terminal screen
pub fn clear_screen() {
	print!("\x1B[2J\x1B[1;1H");
	io::stdout().flush().unwrap();
}

/// Wait for user to press Enter
pub fn wait_for_enter(message: Option<&str>) {
	let msg = message.unwrap_or("Press Enter to continue...");
	print!("{} ", msg.bright_yellow());
	io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
}

/// Ensure account is loaded with proper validation
pub fn ensure_account_loaded() -> Result<(), crate::errors::CliError> {
	// This will be implemented when we have proper account management
	Ok(())
}

/// Display a loading animation for async operations
pub async fn with_loading<F, T>(message: &str, future: F) -> T
where
	F: std::future::Future<Output = T>,
{
	let spinner = create_spinner(message);
	let result = future.await;
	spinner.finish_with_message(format!("{} ✅", message));
	result
}

/// Display error details in a formatted way
pub fn display_error_details(error: &dyn std::error::Error) {
	print_error(&format!("Error: {}", error));

	let mut source = error.source();
	let mut level = 1;

	while let Some(err) = source {
		println!(
			"{}{} Caused by: {}",
			"  ".repeat(level),
			"└─".bright_red(),
			err.to_string().bright_red()
		);
		source = err.source();
		level += 1;
	}
}

/// Create a status indicator
pub fn status_indicator(status: &str) -> ColoredString {
	match status.to_lowercase().as_str() {
		"success" | "ok" | "active" | "online" => "●".bright_green(),
		"warning" | "pending" | "syncing" => "●".bright_yellow(),
		"error" | "failed" | "offline" => "●".bright_red(),
		"info" | "unknown" => "●".bright_blue(),
		_ => "●".bright_white(),
	}
}
