//! Code generation module for Neo dApp templates
//!
//! Provides functionality to generate new projects from templates

use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
	pub template: TemplateMetadata,
	pub files: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMetadata {
	pub name: String,
	pub description: String,
	pub version: String,
	pub author: String,
}

/// Available project templates
#[derive(Debug)]
pub enum ProjectTemplate {
	BasicDapp,
	Nep17Token,
	NftCollection,
	DefiProtocol,
	OracleConsumer,
}

impl ProjectTemplate {
	/// Get template display name
	pub fn display_name(&self) -> &str {
		match self {
			ProjectTemplate::BasicDapp => "Basic Neo dApp",
			ProjectTemplate::Nep17Token => "NEP-17 Token",
			ProjectTemplate::NftCollection => "NFT Collection (NEP-11)",
			ProjectTemplate::DefiProtocol => "DeFi Protocol",
			ProjectTemplate::OracleConsumer => "Oracle Consumer",
		}
	}
}

/// Generate a new project from a template
pub fn generate_project(
	template_type: ProjectTemplate,
	project_name: &str,
	target_dir: Option<PathBuf>,
) -> Result<()> {
	println!("🔧 {} project: {}", "Generating".cyan(), project_name.green());

	// Determine target directory
	let target = target_dir.unwrap_or_else(|| PathBuf::from(".")).join(project_name);

	// Check if directory already exists
	if target.exists() {
		return Err(anyhow::anyhow!("Directory '{}' already exists", target.display()));
	}

	// Load template
	let template = load_template(&template_type)?;

	// Create project directory
	fs::create_dir_all(&target).context("Failed to create project directory")?;

	// Generate files from template
	for (file_path, content) in &template.files {
		generate_file(&target, file_path, content, project_name)?;
	}

	println!("✅ {} created successfully!", format!("Project '{}'", project_name).green().bold());

	// Print next steps
	print_next_steps(project_name, &template_type);

	Ok(())
}

/// Load a template from file
fn load_template(template_type: &ProjectTemplate) -> Result<Template> {
	// For embedded templates, we'll use a match statement
	// In production, these would be loaded from files
	let template_content = match template_type {
		ProjectTemplate::BasicDapp => {
			include_str!("../templates/basic_dapp.toml")
		},
		ProjectTemplate::Nep17Token => {
			include_str!("../templates/nep17_token.toml")
		},
		ProjectTemplate::NftCollection => {
			include_str!("../templates/nft_collection.toml")
		},
		ProjectTemplate::DefiProtocol => {
			include_str!("../templates/defi_protocol.toml")
		},
		ProjectTemplate::OracleConsumer => {
			include_str!("../templates/oracle_consumer.toml")
		},
	};

	toml::from_str(template_content).context("Failed to parse template")
}

/// Generate a single file from template
fn generate_file(
	target_dir: &Path,
	file_path: &str,
	content: &str,
	project_name: &str,
) -> Result<()> {
	let file_path = target_dir.join(file_path);

	// Create parent directories if needed
	if let Some(parent) = file_path.parent() {
		fs::create_dir_all(parent).context("Failed to create directory")?;
	}

	// Replace template variables
	let content = content
		.replace("{{project_name}}", project_name)
		.replace("{{neo3_version}}", neo3::VERSION);

	// Write file
	fs::write(&file_path, content)
		.with_context(|| format!("Failed to write file: {}", file_path.display()))?;

	println!(
		"  📄 Created: {}",
		file_path
			.strip_prefix(target_dir)
			.unwrap_or(&file_path)
			.display()
			.to_string()
			.dimmed()
	);

	Ok(())
}

/// Print next steps after project generation
fn print_next_steps(project_name: &str, template_type: &ProjectTemplate) {
	println!("\n{}", "Next steps:".cyan().bold());
	println!("  1. cd {}", project_name.green());
	println!("  2. cargo build");
	println!("  3. cargo test");

	match template_type {
		ProjectTemplate::Nep17Token => {
			println!("  4. Compile contract: neo3-boa contracts/token.py");
			println!("  5. Deploy contract: neo-cli contract deploy");
		},
		ProjectTemplate::NftCollection => {
			println!("  4. Design your NFT metadata");
			println!("  5. Deploy NFT contract");
		},
		ProjectTemplate::DefiProtocol => {
			println!("  4. Configure liquidity pools");
			println!("  5. Deploy DeFi contracts");
		},
		_ => {
			println!("  4. Configure your application");
			println!("  5. Deploy to Neo blockchain");
		},
	}

	println!("\n📚 Documentation: {}", "https://github.com/R3E-Network/NeoRust".blue());
}

/// List available templates
pub fn list_templates() {
	println!("{}", "Available Project Templates:".cyan().bold());
	println!();

	let templates = vec![
		(ProjectTemplate::BasicDapp, "General purpose blockchain application"),
		(ProjectTemplate::Nep17Token, "Fungible token following NEP-17 standard"),
		(ProjectTemplate::NftCollection, "Non-fungible token collection (NEP-11)"),
		(ProjectTemplate::DefiProtocol, "Decentralized finance protocol"),
		(ProjectTemplate::OracleConsumer, "Application that consumes oracle data"),
	];

	for (template, description) in templates {
		println!(
			"  {} {}",
			format!("• {}", template.display_name()).green().bold(),
			format!("- {}", description).dimmed()
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::TempDir;

	#[test]
	fn test_embedded_templates_parse() {
		let basic =
			load_template(&ProjectTemplate::BasicDapp).expect("basic_dapp.toml should parse");
		assert!(basic.files.contains_key("Cargo.toml"), "basic template should include Cargo.toml");
		assert!(
			basic.files.contains_key("src/main.rs"),
			"basic template should include src/main.rs"
		);

		let nep17 =
			load_template(&ProjectTemplate::Nep17Token).expect("nep17_token.toml should parse");
		assert!(nep17.files.contains_key("Cargo.toml"), "nep17 template should include Cargo.toml");
		assert!(
			nep17.files.contains_key("contracts/token.py"),
			"nep17 template should include contracts/token.py"
		);

		let nft = load_template(&ProjectTemplate::NftCollection)
			.expect("nft_collection.toml should parse");
		assert!(nft.files.contains_key("Cargo.toml"), "nft template should include Cargo.toml");
		assert!(nft.files.contains_key("src/main.rs"), "nft template should include src/main.rs");

		let defi =
			load_template(&ProjectTemplate::DefiProtocol).expect("defi_protocol.toml should parse");
		assert!(defi.files.contains_key("Cargo.toml"), "defi template should include Cargo.toml");
		assert!(defi.files.contains_key("src/main.rs"), "defi template should include src/main.rs");

		let oracle = load_template(&ProjectTemplate::OracleConsumer)
			.expect("oracle_consumer.toml should parse");
		assert!(
			oracle.files.contains_key("Cargo.toml"),
			"oracle template should include Cargo.toml"
		);
		assert!(
			oracle.files.contains_key("src/main.rs"),
			"oracle template should include src/main.rs"
		);
	}

	#[test]
	fn test_project_generation() {
		let temp_dir = TempDir::new().unwrap();
		let project_name = "test_project";

		let result = generate_project(
			ProjectTemplate::BasicDapp,
			project_name,
			Some(temp_dir.path().to_path_buf()),
		);

		assert!(result.is_ok());

		// Check that project directory was created
		let project_dir = temp_dir.path().join(project_name);
		assert!(project_dir.exists());

		// Check that main.rs was created
		let main_file = project_dir.join("src").join("main.rs");
		assert!(main_file.exists());

		// Check that Cargo.toml was created
		let cargo_file = project_dir.join("Cargo.toml");
		assert!(cargo_file.exists());

		let cargo_contents = std::fs::read_to_string(&cargo_file).unwrap();
		assert!(
			!cargo_contents.contains("{{neo3_version}}"),
			"Generated Cargo.toml should not contain unresolved template variables"
		);
		assert!(
			cargo_contents.contains(&format!("neo3 = \"{}\"", neo3::VERSION)),
			"Generated Cargo.toml should pin neo3 dependency to the current SDK version"
		);
	}

	#[test]
	fn test_template_variable_replacement() {
		let content = "name = \"{{project_name}}\"";
		let replaced = content.replace("{{project_name}}", "my_project");
		assert_eq!(replaced, "name = \"my_project\"");
	}
}
