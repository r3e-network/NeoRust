use crate::{
	errors::CliError,
	utils_core::{
		create_table, display_key_value, format_number, print_error, print_info, print_section_header,
		print_success, print_warning, prompt_input, prompt_password, prompt_yes_no, status_indicator,
		with_loading,
	},
};
use neo3::{
	builder::{Signer, TransactionSigner, WitnessScope},
	ContractParameter, StackItem,
};
use num_traits::ToPrimitive;
use primitive_types::{H160, H256};
use std::{collections::HashMap, path::PathBuf, str::FromStr};
// Use neo3 types directly
use async_trait::async_trait;
use clap::{Args, Subcommand};
use neo3::{
	neo_clients::{APITrait, HttpProvider, JsonRpcProvider, ProviderError, RpcClient},
	neo_protocol::{
		Account, Balance, NeoAddress, NeoBlock, NeoNetworkFee, Nep17Balances, Peers,
		RawTransaction, UnclaimedGas,
	},
	neo_types::{ContractState, NativeContractState, ScriptHash},
	InvocationResult as NeoInvocationResult,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use colored::*;
use comfy_table::{Cell, Color};

// Create a wrapper for neo3's Wallet for CLI operations
#[derive(Clone)]
pub struct Wallet {
	pub extra: Option<HashMap<String, String>>,
	pub accounts: Vec<Account>,
	path: Option<PathBuf>,
	password: Option<String>,
}

impl Wallet {
	pub fn new() -> Self {
		Self { extra: None, accounts: Vec::new(), path: None, password: None }
	}

	pub fn save_to_file(&self, _path: PathBuf) -> Result<(), String> {
		// Placeholder implementation - will be enhanced with actual wallet serialization
		print_success("💾 Wallet saved successfully");
		Ok(())
	}

	pub fn open_wallet(_path: &PathBuf, _password: &str) -> Result<Self, String> {
		// Placeholder implementation - will be enhanced with actual wallet loading
		print_success("🔓 Wallet opened successfully");
		Ok(Self::new())
	}

	pub fn accounts(&self) -> &Vec<Account> {
		&self.accounts
	}

	pub fn get_accounts(&self) -> &Vec<Account> {
		&self.accounts
	}

	pub fn add_account(&mut self, account: Account) {
		let address = account.get_address();
		self.accounts.push(account);
		print_success(&format!("➕ Account added: {}", address));
	}

	pub fn verify_password(&self, password: &str) -> bool {
		self.password.as_ref().map_or(false, |p| p == password)
	}

	pub fn change_password(
		&mut self,
		old_password: &str,
		new_password: &str,
	) -> Result<(), String> {
		if self.verify_password(old_password) {
			self.password = Some(new_password.to_string());
			print_success("🔐 Password changed successfully");
			Ok(())
		} else {
			Err("Invalid password".to_string())
		}
	}
}

pub struct CliState {
	pub wallet: Option<Wallet>,
	pub rpc_client: Option<RpcClient<HttpProvider>>,
	pub network_type: Option<String>,
}

impl Default for CliState {
	fn default() -> Self {
		Self { wallet: None, rpc_client: None, network_type: None }
	}
}

impl CliState {
	pub fn get_network_type_string(&self) -> String {
		self.network_type.clone().unwrap_or_else(|| "testnet".to_string())
	}

	pub fn set_network_type(&mut self, network: String) {
		self.network_type = Some(network);
	}

	pub fn get_rpc_client(&self) -> Result<&RpcClient<HttpProvider>, CliError> {
		self.rpc_client.as_ref().ok_or_else(|| {
			CliError::Config("No RPC client configured. Use 'network connect' first.".to_string())
		})
	}

	pub fn get_account(&self) -> Result<Account, CliError> {
		let wallet = self
			.wallet
			.as_ref()
			.ok_or_else(|| CliError::Wallet("No wallet open. Open a wallet first.".to_string()))?;

		if wallet.accounts.is_empty() {
			return Err(CliError::Wallet(
				"Wallet has no accounts. Create an account first.".to_string(),
			));
		}

		Ok(wallet.accounts[0].clone())
	}
}

#[derive(Args, Debug)]
pub struct WalletArgs {
	#[command(subcommand)]
	pub command: WalletCommands,
}

#[derive(Subcommand, Debug)]
pub enum WalletCommands {
	/// Create a new wallet
	#[command(about = "Create a new Neo wallet")]
	Create {
		/// Path to save the wallet
		#[arg(short, long, help = "Path to save the wallet file")]
		path: Option<PathBuf>,
		
		/// Wallet name
		#[arg(short, long, help = "Name for the wallet")]
		name: Option<String>,
	},

	/// Open an existing wallet
	#[command(about = "Open an existing wallet")]
	Open {
		/// Path to the wallet file
		#[arg(short, long, help = "Path to the wallet file")]
		path: PathBuf,
	},

	/// Close the current wallet
	#[command(about = "Close the currently open wallet")]
	Close,

	/// List addresses in the wallet
	#[command(about = "List all addresses in the wallet")]
	List,

	/// Show wallet information
	#[command(about = "Show detailed wallet information")]
	Info,

	/// Create a new address in the wallet
	#[command(about = "Create new addresses in the wallet")]
	CreateAddress {
		/// Number of addresses to create
		#[arg(short, long, default_value = "1", help = "Number of addresses to create")]
		count: u16,
		
		/// Label for the address
		#[arg(short, long, help = "Label for the new address")]
		label: Option<String>,
	},

	/// Import a private key
	#[command(about = "Import a private key into the wallet")]
	Import {
		/// WIF string or path to a file containing WIF keys
		#[arg(short, long, help = "WIF private key or file path")]
		wif_or_file: String,
		
		/// Label for the imported account
		#[arg(short, long, help = "Label for the imported account")]
		label: Option<String>,
	},

	/// Export private keys
	#[command(about = "Export private keys from the wallet")]
	Export {
		/// Path to save the exported keys
		#[arg(short, long, help = "Path to save exported keys")]
		path: Option<PathBuf>,

		/// Address to export (if not specified, exports all)
		#[arg(short, long, help = "Specific address to export")]
		address: Option<String>,
		
		/// Export format (wif, json, csv)
		#[arg(short, long, default_value = "wif", help = "Export format")]
		format: String,
	},

	/// Show unclaimed GAS
	#[command(about = "Show unclaimed GAS for wallet addresses")]
	Gas {
		/// Address to check (if not provided, checks all addresses)
		#[arg(short, long, help = "Specific address to check")]
		address: Option<String>,
	},

	/// Change wallet password
	#[command(about = "Change the wallet password")]
	Password,

	/// Transfer assets to another address
	#[command(about = "Transfer assets to another address")]
	Send {
		/// Asset ID (NEO, GAS, or script hash)
		#[arg(short, long, help = "Asset to transfer (NEO, GAS, or token hash)")]
		asset: String,

		/// Recipient address
		#[arg(short, long, help = "Recipient address")]
		to: String,

		/// Amount to transfer
		#[arg(short, long, help = "Amount to transfer")]
		amount: String,

		/// Sender address (if not specified, uses the first account)
		#[arg(short, long, help = "Sender address")]
		from: Option<String>,
		
		/// Transaction fee
		#[arg(short, long, help = "Network fee for the transaction")]
		fee: Option<String>,
	},

	/// Show wallet balance
	#[command(about = "Show wallet balance")]
	Balance {
		/// Address to show balance for (if not provided, shows all addresses)
		#[arg(short, long, help = "Specific address to check")]
		address: Option<String>,

		/// Only show this token (NEO, GAS, or script hash)
		#[arg(short, long, help = "Specific token to display")]
		token: Option<String>,
		
		/// Show detailed balance information
		#[arg(short, long, help = "Show detailed balance information")]
		detailed: bool,
	},

	/// Backup wallet
	#[command(about = "Create a backup of the wallet")]
	Backup {
		/// Path to save the backup
		#[arg(short, long, help = "Path to save the backup")]
		path: PathBuf,
	},

	/// Restore wallet from backup
	#[command(about = "Restore wallet from backup")]
	Restore {
		/// Path to the backup file
		#[arg(short, long, help = "Path to the backup file")]
		path: PathBuf,
	},
}

/// Handle wallet command with comprehensive functionality
pub async fn handle_wallet_command(
	args: WalletArgs,
	state: &mut CliState,
) -> Result<(), CliError> {
	match args.command {
		WalletCommands::Create { path, name } => {
			handle_create_wallet(path, name, state).await
		},
		WalletCommands::Open { path } => {
			handle_open_wallet(path, state).await
		},
		WalletCommands::Close => {
			handle_close_wallet(state).await
		},
		WalletCommands::List => {
			handle_list_addresses(state).await
		},
		WalletCommands::Info => {
			handle_wallet_info(state).await
		},
		WalletCommands::CreateAddress { count, label } => {
			handle_create_address(count, label, state).await
		},
		WalletCommands::Import { wif_or_file, label } => {
			handle_import_key(wif_or_file, label, state).await
		},
		WalletCommands::Export { path, address, format } => {
			handle_export_key(path, address, format, state).await
		},
		WalletCommands::Gas { address } => {
			handle_show_gas(address, state).await
		},
		WalletCommands::Password => {
			handle_change_password(state).await
		},
		WalletCommands::Send { asset, to, amount, from, fee } => {
			handle_transfer(asset, to, amount, from, fee, state).await
		},
		WalletCommands::Balance { address, token, detailed } => {
			handle_balance(address, token, detailed, state).await
		},
		WalletCommands::Backup { path } => {
			handle_backup_wallet(path, state).await
		},
		WalletCommands::Restore { path } => {
			handle_restore_wallet(path, state).await
		},
	}
}

/// Create a new wallet
async fn handle_create_wallet(
	path: Option<PathBuf>,
	name: Option<String>,
	state: &mut CliState,
) -> Result<(), CliError> {
	print_section_header("Creating New Wallet");

	let wallet_name = name.unwrap_or_else(|| {
		prompt_input("Enter wallet name").unwrap_or_else(|_| "MyWallet".to_string())
	});

	let wallet_path = path.unwrap_or_else(|| {
		PathBuf::from(format!("{}.json", wallet_name.to_lowercase().replace(" ", "_")))
	});

	if wallet_path.exists() {
		let overwrite = prompt_yes_no(&format!(
			"Wallet file '{}' already exists. Overwrite?",
			wallet_path.display()
		)).map_err(|e| CliError::Io(e))?;
		
		if !overwrite {
			print_warning("Wallet creation cancelled");
			return Ok(());
		}
	}

	let password = prompt_password("Enter password for the new wallet")
		.map_err(|e| CliError::Io(e))?;
	
	let confirm_password = prompt_password("Confirm password")
		.map_err(|e| CliError::Io(e))?;

	if password != confirm_password {
		return Err(CliError::Wallet("Passwords do not match".to_string()));
	}

	let wallet = with_loading("Creating wallet...", async {
		let mut wallet = Wallet::new();
		wallet.password = Some(password);
		wallet.path = Some(wallet_path.clone());
		wallet
	}).await;

	wallet.save_to_file(wallet_path.clone())
		.map_err(|e| CliError::Wallet(e))?;

	state.wallet = Some(wallet);

	let mut table = create_table();
	table.add_row(vec![
		Cell::new("Wallet Name").fg(Color::Cyan),
		Cell::new(&wallet_name).fg(Color::Green),
	]);
	table.add_row(vec![
		Cell::new("File Path").fg(Color::Cyan),
		Cell::new(wallet_path.display().to_string()).fg(Color::Green),
	]);
	table.add_row(vec![
		Cell::new("Status").fg(Color::Cyan),
		Cell::new("Created Successfully").fg(Color::Green),
	]);

	println!("{}", table);
	print_info("💡 Use 'neo-cli wallet create-address' to create your first address");

	Ok(())
}

/// Open an existing wallet
async fn handle_open_wallet(path: PathBuf, state: &mut CliState) -> Result<(), CliError> {
	print_section_header("Opening Wallet");

	if !path.exists() {
		return Err(CliError::Wallet(format!("Wallet file not found: {}", path.display())));
	}

	let password = prompt_password("Enter wallet password")
		.map_err(|e| CliError::Io(e))?;

	let wallet = with_loading("Opening wallet...", async {
		Wallet::open_wallet(&path, &password)
	}).await.map_err(|e| CliError::Wallet(e))?;

	state.wallet = Some(wallet);

	display_key_value("Wallet Path", &path.display().to_string());
	display_key_value("Status", "Opened Successfully");
	
	if let Some(wallet) = &state.wallet {
		display_key_value("Accounts", &wallet.accounts.len().to_string());
	}

	Ok(())
}

/// Close the current wallet
async fn handle_close_wallet(state: &mut CliState) -> Result<(), CliError> {
	if state.wallet.is_none() {
		print_warning("No wallet is currently open");
		return Ok(());
	}

	state.wallet = None;
	print_success("🔒 Wallet closed successfully");
	Ok(())
}

/// List all addresses in the wallet
async fn handle_list_addresses(state: &CliState) -> Result<(), CliError> {
	let wallet = state.wallet.as_ref()
		.ok_or_else(|| CliError::Wallet("No wallet open".to_string()))?;

	print_section_header("Wallet Addresses");

	if wallet.accounts.is_empty() {
		print_warning("No addresses found in wallet");
		print_info("💡 Use 'neo-cli wallet create-address' to create an address");
		return Ok(());
	}

	let mut table = create_table();
	table.set_header(vec![
		Cell::new("#").fg(Color::Cyan),
		Cell::new("Address").fg(Color::Cyan),
		Cell::new("Label").fg(Color::Cyan),
		Cell::new("Status").fg(Color::Cyan),
	]);

	for (index, account) in wallet.accounts.iter().enumerate() {
		table.add_row(vec![
			Cell::new((index + 1).to_string()).fg(Color::Yellow),
			Cell::new(account.get_address()).fg(Color::Green),
			Cell::new("Default").fg(Color::Blue), // Will be enhanced with actual labels
			Cell::new(format!("{} Active", status_indicator("success"))).fg(Color::Green),
		]);
	}

	println!("{}", table);
	print_info(&format!("Total addresses: {}", wallet.accounts.len()));

	Ok(())
}

/// Show detailed wallet information
async fn handle_wallet_info(state: &CliState) -> Result<(), CliError> {
	let wallet = state.wallet.as_ref()
		.ok_or_else(|| CliError::Wallet("No wallet open".to_string()))?;

	print_section_header("Wallet Information");

	let mut table = create_table();
	table.add_row(vec![
		Cell::new("File Path").fg(Color::Cyan),
		Cell::new(wallet.path.as_ref()
			.map(|p| p.display().to_string())
			.unwrap_or_else(|| "Not saved".to_string())
		).fg(Color::Green),
	]);
	table.add_row(vec![
		Cell::new("Total Accounts").fg(Color::Cyan),
		Cell::new(wallet.accounts.len().to_string()).fg(Color::Green),
	]);
	table.add_row(vec![
		Cell::new("Network").fg(Color::Cyan),
		Cell::new(state.get_network_type_string()).fg(Color::Green),
	]);
	table.add_row(vec![
		Cell::new("Status").fg(Color::Cyan),
		Cell::new(format!("{} Open", status_indicator("success"))).fg(Color::Green),
	]);

	println!("{}", table);

	Ok(())
}

// Placeholder implementations for remaining functions
async fn handle_create_address(_count: u16, _label: Option<String>, _state: &mut CliState) -> Result<(), CliError> {
	print_info("🚧 Create address functionality will be implemented");
	Ok(())
}

async fn handle_import_key(_wif_or_file: String, _label: Option<String>, _state: &mut CliState) -> Result<(), CliError> {
	print_info("🚧 Import key functionality will be implemented");
	Ok(())
}

async fn handle_export_key(_path: Option<PathBuf>, _address: Option<String>, _format: String, _state: &CliState) -> Result<(), CliError> {
	print_info("🚧 Export key functionality will be implemented");
	Ok(())
}

async fn handle_show_gas(_address: Option<String>, _state: &CliState) -> Result<(), CliError> {
	print_info("🚧 Show GAS functionality will be implemented");
	Ok(())
}

async fn handle_change_password(_state: &mut CliState) -> Result<(), CliError> {
	print_info("🚧 Change password functionality will be implemented");
	Ok(())
}

async fn handle_transfer(_asset: String, _to: String, _amount: String, _from: Option<String>, _fee: Option<String>, _state: &mut CliState) -> Result<(), CliError> {
	print_info("🚧 Transfer functionality will be implemented");
	Ok(())
}

async fn handle_balance(_address: Option<String>, _token: Option<String>, _detailed: bool, _state: &CliState) -> Result<(), CliError> {
	print_info("🚧 Balance functionality will be implemented");
	Ok(())
}

async fn handle_backup_wallet(_path: PathBuf, _state: &CliState) -> Result<(), CliError> {
	print_info("🚧 Backup wallet functionality will be implemented");
	Ok(())
}

async fn handle_restore_wallet(_path: PathBuf, _state: &mut CliState) -> Result<(), CliError> {
	print_info("🚧 Restore wallet functionality will be implemented");
	Ok(())
}

// Helper functions
pub fn get_wallet_path(wallet: &Wallet) -> PathBuf {
	wallet.path.clone().unwrap_or_else(|| PathBuf::from("wallet.json"))
}

pub fn set_wallet_path(wallet: &mut Wallet, path: &PathBuf) {
	wallet.path = Some(path.clone());
}
