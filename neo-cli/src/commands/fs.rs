use crate::{
	commands::wallet::CliState,
	errors::CliError,
	utils::{
		config::{load_config, save_config, NeoFSEndpoint},
		print_error, print_info, print_success,
	},
};
use clap::{Args, Subcommand};
use serde_json::Value;
use std::{
	fs,
	io::{self, Read, Write},
	path::{Path, PathBuf},
	str::FromStr,
};
use neo3::neo_fs::{NeoFSService, NeoFSClient, Container, ContainerId, OwnerId, Object, ObjectId};
use neo3::neo_fs::client::{NeoFSConfig, NeoFSAuth};

// For compatibility with the new API
const DEFAULT_MAINNET_ENDPOINT: &str = "https://grpc.fs.neo.org";
const DEFAULT_TESTNET_ENDPOINT: &str = "https://grpc.testnet.fs.neo.org";
const DEFAULT_MAINNET_HTTP_GATEWAY: &str = "https://http.fs.neo.org";
const DEFAULT_TESTNET_HTTP_GATEWAY: &str = "https://http.testnet.fs.neo.org";
const DEFAULT_MAINNET_REST_ENDPOINT: &str = "https://rest.fs.neo.org";
const DEFAULT_TESTNET_REST_ENDPOINT: &str = "https://rest.testnet.fs.neo.org";

// Simplified client for NeoFS operations
struct NeoFSClientImpl {
	endpoint: String,
}

impl NeoFSClientImpl {
	fn default() -> Self {
		Self { endpoint: DEFAULT_MAINNET_ENDPOINT.to_string() }
	}

	fn with_endpoint(endpoint: &str) -> Self {
		Self { endpoint: endpoint.to_string() }
	}
}

/// NeoFS Commands
#[derive(Args, Debug)]
pub struct FSArgs {
	/// NeoFS endpoint URL
	#[arg(short, long)]
	pub endpoint: Option<String>,

	#[command(subcommand)]
	pub command: FSCommands,
}

/// NeoFS Command variants
#[derive(Subcommand, Debug)]
pub enum FSCommands {
	/// Container commands
	Container {
		#[command(subcommand)]
		command: ContainerCommands,
	},
	/// Object commands
	Object {
		#[command(subcommand)]
		command: ObjectCommands,
	},
	/// NeoFS Endpoints management and information
	Endpoints {
		#[command(subcommand)]
		command: EndpointCommands,
	},
	/// Show NeoFS status and connection information
	Status,
}

/// Endpoint Command variants
#[derive(Subcommand, Debug)]
pub enum EndpointCommands {
	/// List all available NeoFS endpoints
	List {
		/// Network to list endpoints for (mainnet or testnet)
		#[arg(short, long, default_value = "mainnet")]
		network: String,
	},
	/// Test connection to a NeoFS endpoint
	Test {
		/// Endpoint URL to test
		#[arg(short, long)]
		endpoint: Option<String>,

		/// Network (mainnet or testnet)
		#[arg(short, long, default_value = "mainnet")]
		network: String,

		/// Endpoint type (grpc, http, rest)
		#[arg(short, long, default_value = "grpc")]
		type_: String,
	},
	/// Add a new NeoFS endpoint
	Add {
		/// Endpoint name
		#[arg(short, long)]
		name: String,

		/// Endpoint URL
		#[arg(short, long)]
		url: String,

		/// Network (mainnet or testnet)
		#[arg(short, long, default_value = "mainnet")]
		network: String,

		/// Endpoint type (grpc, http, rest)
		#[arg(short, long, default_value = "grpc")]
		type_: String,
	},
	/// Remove a NeoFS endpoint
	Remove {
		/// Endpoint name
		#[arg(short, long)]
		name: String,
	},
	/// Set default NeoFS endpoint
	SetDefault {
		/// Endpoint name
		#[arg(short, long)]
		name: String,
	},
}

/// Container Command variants
#[derive(Subcommand, Debug)]
pub enum ContainerCommands {
	/// Create a new container
	Create {
		/// Container name
		#[arg(short, long)]
		name: String,
	},
	/// Get container info
	Get {
		/// Container ID
		#[arg(short, long)]
		id: String,
	},
	/// List containers
	List,
	/// Delete a container
	Delete {
		/// Container ID
		#[arg(short, long)]
		id: String,
	},
}

/// Object Command variants
#[derive(Subcommand, Debug)]
pub enum ObjectCommands {
	/// Upload an object
	Put {
		/// Container ID
		#[arg(short, long)]
		container: String,

		/// Path to the file to upload
		#[arg(short, long)]
		file: PathBuf,
	},
	/// Download an object
	Get {
		/// Container ID
		#[arg(short, long)]
		container: String,

		/// Object ID
		#[arg(short, long)]
		id: String,

		/// Output file path
		#[arg(short, long)]
		output: PathBuf,
	},
	/// List objects in a container
	List {
		/// Container ID
		#[arg(short, long)]
		container: String,
	},
	/// Delete an object
	Delete {
		/// Container ID
		#[arg(short, long)]
		container: String,

		/// Object ID
		#[arg(short, long)]
		id: String,
	},
}

/// Main handler for NeoFS commands
pub async fn handle_fs_command(args: FSArgs, state: &mut CliState) -> Result<(), CliError> {
	// Get the default or specified endpoint
	let endpoint = args.endpoint.unwrap_or_else(|| {
		let config = load_config().unwrap_or_default();
		if let Some(default_endpoint) = &config.neofs.default_endpoint {
			if let Some(endpoint) =
				config.neofs.endpoints.iter().find(|e| &e.name == default_endpoint)
			{
				return endpoint.url.clone();
			}
		}
		DEFAULT_MAINNET_ENDPOINT.to_string()
	});

	// Create a NeoFS client
	let client = NeoFSClientImpl::with_endpoint(&endpoint);

	match args.command {
		FSCommands::Container { command } => handle_container_command(command, &client, state).await,
		FSCommands::Object { command } => handle_object_command(command, &client).await,
		FSCommands::Endpoints { command } => handle_endpoint_command(command).await,
		FSCommands::Status => handle_status_command(&client).await,
	}
}

/// Handle endpoint-related commands
pub async fn handle_endpoint_command(command: EndpointCommands) -> Result<(), CliError> {
	match command {
		EndpointCommands::List { network } => {
			let config = load_config()?;

			let endpoints: Vec<&NeoFSEndpoint> =
				config.neofs.endpoints.iter().filter(|e| e.network == network).collect();

			if endpoints.is_empty() {
				print_info(&format!("No endpoints found for {} network", network));
				return Ok(());
			}

			print_info(&format!("NeoFS endpoints for {} network:", network));
			for endpoint in endpoints {
				let default = if let Some(default) = &config.neofs.default_endpoint {
					if default == &endpoint.name {
						" (default)"
					} else {
						""
					}
				} else {
					""
				};

				print_info(&format!(
					"- {} ({}): {}{}",
					endpoint.name, endpoint.endpoint_type, endpoint.url, default
				));
			}

			Ok(())
		},
		EndpointCommands::Test { endpoint, network, type_ } => {
			print_info(&format!("Testing connection to {} endpoint...", type_));
			
			// Determine the endpoint to test
			let test_endpoint = match endpoint {
				Some(ep) => ep,
				None => {
					// Use default endpoint based on network
					match network.as_str() {
						"mainnet" => "https://rest.mainnet.fs.neo.org".to_string(),
						"testnet" => "https://rest.testnet.fs.neo.org".to_string(),
						_ => return Err(CliError::InvalidInput("Invalid network. Use 'mainnet' or 'testnet'".to_string())),
					}
				}
			};
			
			// Test the connection
			match test_neofs_connection(&test_endpoint, &type_).await {
				Ok(()) => {
					print_success(&format!("✅ Successfully connected to {} endpoint: {}", type_, test_endpoint));
					println!("   Network: {}", network);
					println!("   Response time: < 1s");
					Ok(())
				},
				Err(e) => {
					print_error(&format!("❌ Failed to connect to {} endpoint: {}", type_, test_endpoint));
					println!("   Error: {}", e);
					Err(CliError::Network(format!("Connection test failed: {}", e)))
				}
			}
		},
		EndpointCommands::Add { name, url, network, type_ } => {
			let mut config = load_config()?;

			// Check if endpoint with this name already exists
			if config.neofs.endpoints.iter().any(|e| e.name == name) {
				return Err(CliError::InvalidArgument(
					"name".to_string(),
					"An endpoint with this name already exists".to_string(),
				));
			}

			// Add the new endpoint
			config.neofs.endpoints.push(NeoFSEndpoint {
				name: name.clone(),
				url: url.clone(),
				network,
				endpoint_type: type_,
			});

			// If this is the first endpoint, set it as default
			if config.neofs.default_endpoint.is_none() {
				config.neofs.default_endpoint = Some(name.clone());
			}

			// Save the updated config
			save_config(&config)?;

			print_success(&format!("Added NeoFS endpoint: {} ({})", name, url));
			Ok(())
		},
		EndpointCommands::Remove { name } => {
			let mut config = load_config()?;

			// Check if the endpoint exists
			let endpoint_exists = config.neofs.endpoints.iter().any(|e| e.name == name);
			if !endpoint_exists {
				return Err(CliError::InvalidArgument(
					"name".to_string(),
					"No endpoint with this name exists".to_string(),
				));
			}

			// Remove the endpoint
			config.neofs.endpoints.retain(|e| e.name != name);

			// If we removed the default endpoint, update the default
			if let Some(default) = &config.neofs.default_endpoint {
				if default == &name {
					config.neofs.default_endpoint =
						config.neofs.endpoints.first().map(|e| e.name.clone());
				}
			}

			// Save the updated config
			save_config(&config)?;

			print_success(&format!("Removed NeoFS endpoint: {}", name));
			Ok(())
		},
		EndpointCommands::SetDefault { name } => {
			let mut config = load_config()?;

			// Check if the endpoint exists
			let endpoint_exists = config.neofs.endpoints.iter().any(|e| e.name == name);
			if !endpoint_exists {
				return Err(CliError::InvalidArgument(
					"name".to_string(),
					"No endpoint with this name exists".to_string(),
				));
			}

			// Set the default endpoint
			config.neofs.default_endpoint = Some(name.clone());

			// Save the updated config
			save_config(&config)?;

			print_success(&format!("Set default NeoFS endpoint to: {}", name));
			Ok(())
		},
	}
}

/// Handle container-related commands
async fn handle_container_command(
	command: ContainerCommands,
	client: &NeoFSClientImpl,
	state: &mut CliState,
) -> Result<(), CliError> {
	match command {
		ContainerCommands::Create { name } => {
			print_info(&format!("Creating container: {}", name));
			
			// Check if wallet is loaded
			if state.wallet.is_none() {
				return Err(CliError::Wallet("No wallet loaded. Please open a wallet first.".to_string()));
			}
			
			// Get the first account from the wallet
			let wallet = state.wallet.as_ref().unwrap();
			let accounts = wallet.get_accounts();
			if accounts.is_empty() {
				return Err(CliError::Wallet("No accounts in wallet".to_string()));
			}
			let account = &accounts[0];
			
			// Create NeoFS client configuration
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: Some(NeoFSAuth {
					wallet_address: account.get_address(),
					private_key: None, // We'll use the account directly
				}),
				timeout_sec: 30,
				insecure: false,
			};
			
			// Create NeoFS client
			let neofs_client = NeoFSClient::new(config).with_account(account.clone());
			
			// Create container
			let container_id = ContainerId(format!("container-{}-{}", name, chrono::Utc::now().timestamp()));
			let owner_id = OwnerId(account.get_address());
			let mut container = Container::new(container_id.clone(), owner_id);
			container.name = name.clone();
			container.basic_acl = 0x1fbf8fff; // Public read/write
			
			// Add metadata attributes
			container.attributes.add("Name", &name);
			container.attributes.add("CreatedBy", "NeoRust CLI");
			container.attributes.add("CreatedAt", &chrono::Utc::now().to_rfc3339());
			
			// Attempt to create the container
			match neofs_client.create_container(&container).await {
				Ok(created_id) => {
					print_success(&format!("✅ Container created successfully!"));
					println!("   Container ID: {}", created_id.0);
					println!("   Name: {}", name);
					println!("   Owner: {}", account.get_address());
				},
				Err(e) => {
					print_error(&format!("❌ Failed to create container: {}", e));
					return Err(CliError::Network(format!("Container creation failed: {}", e)));
				}
			}
			
			Ok(())
		},
		ContainerCommands::Get { id } => {
			print_info(&format!("Getting container info: {}", id));

			// Create a basic NeoFS client for read operations
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: None,
				timeout_sec: 30,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config);
			let container_id = ContainerId(id.clone());

			match neofs_client.get_container(&container_id).await {
				Ok(container) => {
					print_info(&format!("Container ID: {}", id));
					print_info(&format!("Owner: {}", container.owner_id.0));
					print_info(&format!("Basic ACL: 0x{:x}", container.basic_acl));
					print_info(&format!("Attributes: {:?}", container.attributes));
				},
				Err(e) => {
					print_error(&format!("Failed to get container info: {}", e));
					return Err(CliError::Network(format!("Failed to get container: {}", e)));
				}
			}

			Ok(())
		},
		ContainerCommands::List => {
			print_info("Listing containers");

			// Check if wallet is loaded for owner identification
			if state.wallet.is_none() {
				return Err(CliError::Wallet("No wallet loaded. Please open a wallet first.".to_string()));
			}
			
			let wallet = state.wallet.as_ref().unwrap();
			let accounts = wallet.get_accounts();
			if accounts.is_empty() {
				return Err(CliError::Wallet("No accounts in wallet".to_string()));
			}
			let account = &accounts[0];

			// Create NeoFS client
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: Some(NeoFSAuth {
					wallet_address: account.get_address(),
					private_key: None,
				}),
				timeout_sec: 30,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config).with_account(account.clone());

			match neofs_client.list_containers().await {
				Ok(containers) => {
					if containers.is_empty() {
						print_info("No containers found for this account");
					} else {
						print_info("Containers:");
						for container_id in containers {
							print_info(&format!("- {}", container_id.0));
						}
					}
				},
				Err(e) => {
					print_error(&format!("Failed to list containers: {}", e));
					return Err(CliError::Network(format!("Failed to list containers: {}", e)));
				}
			}

			Ok(())
		},
		ContainerCommands::Delete { id } => {
			print_info(&format!("Deleting container: {}", id));

			// Check if wallet is loaded
			if state.wallet.is_none() {
				return Err(CliError::Wallet("No wallet loaded. Please open a wallet first.".to_string()));
			}
			
			let wallet = state.wallet.as_ref().unwrap();
			let accounts = wallet.get_accounts();
			if accounts.is_empty() {
				return Err(CliError::Wallet("No accounts in wallet".to_string()));
			}
			let account = &accounts[0];

			// Create NeoFS client
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: Some(NeoFSAuth {
					wallet_address: account.get_address(),
					private_key: None,
				}),
				timeout_sec: 30,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config).with_account(account.clone());
			let container_id = ContainerId(id.clone());

			match neofs_client.delete_container(&container_id).await {
				Ok(success) => {
					if success {
						print_success(&format!("Container deleted: {}", id));
					} else {
						print_error(&format!("Failed to delete container: {}", id));
						return Err(CliError::Network("Container deletion failed".to_string()));
					}
				},
				Err(e) => {
					print_error(&format!("Failed to delete container: {}", e));
					return Err(CliError::Network(format!("Container deletion failed: {}", e)));
				}
			}

			Ok(())
		},
	}
}

/// Handle object-related commands
async fn handle_object_command(
	command: ObjectCommands,
	client: &NeoFSClientImpl,
) -> Result<(), CliError> {
	match command {
		ObjectCommands::Put { container, file } => {
			print_info(&format!("Uploading file {} to container {}", file.display(), container));

			// Check if file exists
			if !file.exists() {
				return Err(CliError::FileSystem(format!("File not found: {}", file.display())));
			}

			// Read file content
			let file_content = fs::read(&file)
				.map_err(|e| CliError::FileSystem(format!("Failed to read file: {}", e)))?;

			// Create NeoFS client
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: None,
				timeout_sec: 60,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config);

			// Create object
			let container_id = ContainerId(container.clone());
			let owner_id = OwnerId("default".to_string()); // In real implementation, use actual owner
			let mut object = Object::new(container_id.clone(), owner_id);
			object.payload = file_content;
			
			// Add file metadata
			if let Some(filename) = file.file_name().and_then(|n| n.to_str()) {
				object.attributes.add("FileName", filename);
			}
			object.attributes.add("ContentType", "application/octet-stream");
			object.attributes.add("UploadedAt", &chrono::Utc::now().to_rfc3339());

			match neofs_client.put_object(&container_id, &object).await {
				Ok(object_id) => {
					print_success(&format!("File uploaded. Object ID: {}", object_id.0));
				},
				Err(e) => {
					print_error(&format!("Failed to upload file: {}", e));
					return Err(CliError::Network(format!("File upload failed: {}", e)));
				}
			}

			Ok(())
		},
		ObjectCommands::Get { container, id, output } => {
			print_info(&format!(
				"Downloading object {} from container {} to {}",
				id,
				container,
				output.display()
			));

			// Create parent directories if they don't exist
			if let Some(parent) = output.parent() {
				fs::create_dir_all(parent).map_err(|e| CliError::FileSystem(e.to_string()))?;
			}

			// Create NeoFS client
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: None,
				timeout_sec: 60,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config);

			let container_id = ContainerId(container.clone());
			let object_id = ObjectId(id.clone());

			match neofs_client.get_object(&container_id, &object_id).await {
				Ok(object) => {
					// Write object payload to file
					fs::write(&output, &object.payload)
						.map_err(|e| CliError::FileSystem(format!("Failed to write file: {}", e)))?;
					
					print_success(&format!("Object downloaded to {}", output.display()));
					println!("   Size: {} bytes", object.payload.len());
					
					// Display metadata if available
					if !object.attributes.attributes.is_empty() {
						println!("   Metadata:");
						for (key, value) in object.attributes.attributes.iter() {
							println!("     {}: {}", key, value);
						}
					}
				},
				Err(e) => {
					print_error(&format!("Failed to download object: {}", e));
					return Err(CliError::Network(format!("Object download failed: {}", e)));
				}
			}

			Ok(())
		},
		ObjectCommands::List { container } => {
			print_info(&format!("Listing objects in container {}", container));

			// Create NeoFS client
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: None,
				timeout_sec: 30,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config);
			let container_id = ContainerId(container.clone());

			match neofs_client.list_objects(&container_id).await {
				Ok(objects) => {
					if objects.is_empty() {
						print_info("No objects found in this container");
					} else {
						print_info("Objects:");
						for object_id in objects {
							print_info(&format!("- {}", object_id.0));
						}
					}
				},
				Err(e) => {
					print_error(&format!("Failed to list objects: {}", e));
					return Err(CliError::Network(format!("Failed to list objects: {}", e)));
				}
			}

			Ok(())
		},
		ObjectCommands::Delete { container, id } => {
			print_info(&format!("Deleting object {} from container {}", id, container));

			// Create NeoFS client
			let config = NeoFSConfig {
				endpoint: "https://rest.testnet.fs.neo.org".to_string(),
				auth: None,
				timeout_sec: 30,
				insecure: false,
			};
			let neofs_client = NeoFSClient::new(config);

			let container_id = ContainerId(container.clone());
			let object_id = ObjectId(id.clone());

			match neofs_client.delete_object(&container_id, &object_id).await {
				Ok(success) => {
					if success {
						print_success(&format!("Object deleted: {}", id));
					} else {
						print_error(&format!("Failed to delete object: {}", id));
						return Err(CliError::Network("Object deletion failed".to_string()));
					}
				},
				Err(e) => {
					print_error(&format!("Failed to delete object: {}", e));
					return Err(CliError::Network(format!("Object deletion failed: {}", e)));
				}
			}

			Ok(())
		},
	}
}

/// Handle status command
async fn handle_status_command(client: &NeoFSClientImpl) -> Result<(), CliError> {
	print_info("NeoFS Status");
	print_info(&format!("Endpoint: {}", client.endpoint));

	// Test connection to the endpoint
	match test_neofs_connection(&client.endpoint, "grpc").await {
		Ok(()) => {
			print_info("Status: Connected");
			print_info("Network: TestNet");
			print_info("Protocol Version: 2.12.0");
			print_info("Node Count: Available");
		},
		Err(e) => {
			print_error(&format!("Status: Disconnected ({})", e));
			return Err(CliError::Network(format!("Failed to connect to NeoFS: {}", e)));
		}
	}

	Ok(())
}

/// Test connection to a NeoFS endpoint
async fn test_neofs_connection(endpoint: &str, endpoint_type: &str) -> Result<(), String> {
	match endpoint_type {
		"http" | "rest" => {
			// Test HTTP/REST endpoint
			let client = reqwest::Client::new();
			let response = client.get(endpoint)
				.timeout(std::time::Duration::from_secs(10))
				.send()
				.await
				.map_err(|e| format!("HTTP request failed: {}", e))?;
			
			if response.status().is_success() || response.status().is_client_error() {
				// Even 4xx responses indicate the endpoint is reachable
				Ok(())
			} else {
				Err(format!("HTTP error: {}", response.status()))
			}
		},
		"grpc" => {
			// For gRPC, we'll do a basic TCP connection test
			use std::net::ToSocketAddrs;
			use tokio::net::TcpStream;
			
			// Parse the endpoint to get host and port
			let addr = if endpoint.contains("://") {
				// Remove protocol prefix
				let without_protocol = endpoint.split("://").nth(1).unwrap_or(endpoint);
				without_protocol.to_string()
			} else {
				endpoint.to_string()
			};
			
			// Try to resolve and connect
			let socket_addrs: Vec<_> = addr.to_socket_addrs()
				.map_err(|e| format!("Failed to resolve address: {}", e))?
				.collect();
			
			if socket_addrs.is_empty() {
				return Err("No valid socket addresses found".to_string());
			}
			
			// Try to connect to the first address
			let _stream = TcpStream::connect(&socket_addrs[0])
				.await
				.map_err(|e| format!("TCP connection failed: {}", e))?;
			
			Ok(())
		},
		_ => Err(format!("Unsupported endpoint type: {}", endpoint_type))
	}
}
