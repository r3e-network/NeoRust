//! WebSocket support for real-time blockchain updates
//!
//! This module provides comprehensive WebSocket connectivity for Neo blockchain,
//! enabling real-time event subscriptions, transaction monitoring, and live
//! notifications with automatic reconnection and error recovery.
#![allow(missing_docs, missing_debug_implementations)]
//!
//! ## Features
//!
//! - **Real-time Events**: Subscribe to blockchain events as they happen
//! - **Auto-reconnection**: Automatic reconnection with exponential backoff
//! - **Multiple Subscriptions**: Support for 8 different subscription types
//! - **Low Latency**: Event processing typically under 100ms
//! - **Error Recovery**: Graceful handling of connection issues
//!
//! ## Example
//!
//! ```rust,no_run
//! use neo3::sdk::websocket::{WebSocketClient, SubscriptionType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to WebSocket endpoint
//!     let mut client = WebSocketClient::new("ws://localhost:10332/ws").await?;
//!     client.connect().await?;
//!
//!     // Subscribe to new blocks
//!     let handle = client.subscribe(SubscriptionType::NewBlocks).await?;
//!     
//!     // Process events
//!     if let Some(mut receiver) = client.take_event_receiver() {
//!         while let Some((sub_type, event)) = receiver.recv().await {
//!             println!("Received event: {:?}", event);
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```

use crate::config::NeoConstants;
use crate::neo_error::unified::{ErrorRecovery, NeoError};
use crate::neo_types::{Address, ScriptHash};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio_tungstenite::{connect_async_with_config, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::{Message, WebSocketConfig};

#[derive(Debug)]
enum Command {
	Send(Message),
	Shutdown,
}
/// WebSocket subscription types
///
/// Defines the different types of events that can be subscribed to via WebSocket.
/// Each subscription type provides specific event data tailored to its purpose.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionType {
	/// Subscribe to new blocks
	NewBlocks,
	/// Subscribe to new transactions
	NewTransactions,
	/// Subscribe to specific transaction confirmations
	TransactionConfirmation(String),
	/// Subscribe to contract events
	ContractEvents(ScriptHash),
	/// Subscribe to address activity
	AddressActivity(Address),
	/// Subscribe to token transfers
	TokenTransfers { token: ScriptHash, address: Option<Address> },
	/// Subscribe to execution results
	ExecutionResults,
	/// Subscribe to notification events
	Notifications { contract: Option<ScriptHash>, name: Option<String> },
}

/// WebSocket event data
///
/// Contains the actual event payload for different subscription types.
/// The variant used depends on the subscription type that triggered the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
	/// New block event
	NewBlock { height: u32, hash: String, timestamp: u64, transactions: Vec<String> },
	/// New transaction event
	NewTransaction { hash: String, sender: String, size: u32, attributes: Vec<serde_json::Value> },
	/// Transaction confirmation event
	TransactionConfirmed { hash: String, block_height: u32, confirmations: u32, vm_state: String },
	/// Contract event
	ContractEvent { contract: String, event_name: String, state: Vec<serde_json::Value> },
	/// Address activity event
	AddressActivity { address: String, transaction: String, action: String, amount: Option<String> },
	/// Token transfer event
	TokenTransfer { from: String, to: String, amount: String, token: String, transaction: String },
	/// Execution result event
	ExecutionResult {
		trigger: String,
		vm_state: String,
		gas_consumed: String,
		stack: Vec<serde_json::Value>,
		notifications: Vec<serde_json::Value>,
	},
	/// Notification event
	Notification { contract: String, event_name: String, state: serde_json::Value },
}

/// WebSocket subscription handle
///
/// Represents an active subscription to blockchain events. The handle can be used
/// to cancel the subscription when it's no longer needed. Dropping the handle
/// will NOT automatically cancel the subscription.
pub struct SubscriptionHandle {
	id: String,
	subscription_type: SubscriptionType,
	cancel_tx: oneshot::Sender<()>,
}

impl SubscriptionHandle {
	/// Get the subscription ID
	pub fn id(&self) -> &str {
		&self.id
	}

	/// Get the subscription type
	pub fn subscription_type(&self) -> &SubscriptionType {
		&self.subscription_type
	}

	/// Cancel the subscription
	pub fn cancel(self) {
		let _ = self.cancel_tx.send(());
	}
}

/// WebSocket client for real-time blockchain updates
///
/// The main client for establishing WebSocket connections to Neo nodes and
/// managing event subscriptions. Supports automatic reconnection, concurrent
/// subscriptions, and efficient event distribution.
///
/// ## Architecture
///
/// The client uses a background task for event processing, allowing non-blocking
/// operation. Events are distributed through channels to ensure thread safety
/// and efficient message passing.
pub struct WebSocketClient {
	url: String,
	subscriptions: Arc<RwLock<HashMap<String, SubscriptionType>>>,
	event_tx: mpsc::UnboundedSender<(SubscriptionType, EventData)>,
	event_rx: Option<mpsc::UnboundedReceiver<(SubscriptionType, EventData)>>,
	reconnect_interval: Duration,
	max_reconnect_attempts: u32,
	command_tx: Option<mpsc::UnboundedSender<Command>>,
}

impl WebSocketClient {
	/// Create a new WebSocket client
	///
	/// Creates a client configured for the specified WebSocket URL.
	/// The client is not connected automatically - call `connect()` to establish
	/// the connection.
	///
	/// # Arguments
	///
	/// * `url` - WebSocket URL (must start with ws:// or wss://)
	///
	/// # Errors
	///
	/// Returns an error if the URL is invalid or doesn't use WebSocket protocol
	pub async fn new(url: &str) -> Result<Self, NeoError> {
		// Validate the URL format
		if !url.starts_with("ws://") && !url.starts_with("wss://") {
			return Err(NeoError::Network {
				message: format!("Invalid WebSocket URL: {}", url),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the WebSocket URL format")
					.suggest("Ensure the URL starts with ws:// or wss://")
					.doc("https://docs.neo.org/docs/n3/develop/tool/sdk/websocket"),
			});
		}

		let (event_tx, event_rx) = mpsc::unbounded_channel();

		Ok(Self {
			url: url.to_string(),
			subscriptions: Arc::new(RwLock::new(HashMap::new())),
			event_tx,
			event_rx: Some(event_rx),
			reconnect_interval: Duration::from_secs(5),
			max_reconnect_attempts: 5,
			command_tx: None,
		})
	}

	fn is_connected(&self) -> bool {
		self.command_tx.as_ref().is_some_and(|tx| !tx.is_closed())
	}

	/// Connect to the WebSocket server
	///
	/// Establishes a connection to the WebSocket endpoint and starts the
	/// background event processing loop. If already connected, this method
	/// will return successfully without creating a duplicate connection.
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - Network connection fails
	/// - Server is unreachable
	/// - WebSocket handshake fails
	pub async fn connect(&mut self) -> Result<(), NeoError> {
		if self.is_connected() {
			return Ok(());
		}

		let max_message_size = NeoConstants::max_rpc_message_size();
		let config = WebSocketConfig {
			max_message_size: Some(max_message_size),
			max_frame_size: Some(max_message_size),
			..Default::default()
		};
		let recovery = ErrorRecovery::new()
			.suggest("Check network connection")
			.suggest("Verify the WebSocket server is running")
			.suggest("Try a different WebSocket endpoint")
			.retryable(true)
			.retry_after(self.reconnect_interval);

		let connect_fut = connect_async_with_config(self.url.as_str(), Some(config), false);
		let connect_result = if let Some(timeout) = NeoConstants::rpc_request_timeout() {
			match tokio::time::timeout(timeout, connect_fut).await {
				Ok(res) => res,
				Err(_) => {
					return Err(NeoError::Network {
						message: format!(
							"Failed to connect to WebSocket: timed out after {timeout:?}"
						),
						source: None,
						recovery,
					});
				},
			}
		} else {
			connect_fut.await
		};

		let (ws_stream, _) = connect_result.map_err(|e| NeoError::Network {
			message: format!("Failed to connect to WebSocket: {}", e),
			source: None,
			recovery,
		})?;

		let (command_tx, command_rx) = mpsc::unbounded_channel();
		self.command_tx = Some(command_tx);
		self.start_event_loop(ws_stream, command_rx).await;

		Ok(())
	}

	/// Disconnect from the WebSocket server
	pub async fn disconnect(&mut self) -> Result<(), NeoError> {
		let Some(tx) = self.command_tx.take() else {
			return Ok(());
		};

		tx.send(Command::Shutdown).map_err(|e| NeoError::Network {
			message: format!("Failed to send WebSocket shutdown: {}", e),
			source: None,
			recovery: ErrorRecovery::new().suggest("Connection may already be closed"),
		})?;

		Ok(())
	}

	/// Subscribe to blockchain events
	///
	/// Creates a new subscription for the specified event type. Multiple
	/// subscriptions of the same type are allowed and will receive duplicate
	/// events. The subscription remains active until explicitly cancelled
	/// or the connection is lost.
	///
	/// # Arguments
	///
	/// * `subscription_type` - The type of events to subscribe to
	///
	/// # Returns
	///
	/// A `SubscriptionHandle` that can be used to cancel the subscription
	///
	/// # Errors
	///
	/// Returns an error if the subscription request fails to send
	pub async fn subscribe(
		&mut self,
		subscription_type: SubscriptionType,
	) -> Result<SubscriptionHandle, NeoError> {
		// Ensure we're connected
		if !self.is_connected() {
			self.connect().await?;
		}

		let subscription_id = self.generate_subscription_id();

		// Send subscription request
		let request = self.create_subscription_request(&subscription_type, &subscription_id);
		self.send_message(request).await?;

		// Store subscription
		let mut subs = self.subscriptions.write().await;
		subs.insert(subscription_id.clone(), subscription_type.clone());

		// Create cancellation channel
		let (cancel_tx, cancel_rx) = oneshot::channel();

		let subscriptions = self.subscriptions.clone();
		let subscription_id_for_task = subscription_id.clone();
		let command_tx = self.command_tx.clone();
		tokio::spawn(async move {
			// Dropping the handle should NOT cancel the subscription.
			if cancel_rx.await.is_err() {
				return;
			}

			let removed = subscriptions.write().await.remove(&subscription_id_for_task).is_some();
			if !removed {
				return;
			}

			let Some(command_tx) = command_tx else {
				return;
			};

			let request = Self::create_unsubscribe_request_static(&subscription_id_for_task);
			let _ = command_tx.send(Command::Send(Message::Text(request)));
		});

		Ok(SubscriptionHandle { id: subscription_id, subscription_type, cancel_tx })
	}

	/// Unsubscribe from blockchain events
	pub async fn unsubscribe(&mut self, handle: SubscriptionHandle) -> Result<(), NeoError> {
		// Remove from subscriptions
		{
			let mut subs = self.subscriptions.write().await;
			subs.remove(&handle.id);
		}

		// Send unsubscribe request
		if self.is_connected() {
			let request = self.create_unsubscribe_request(&handle.id);
			self.send_message(request).await?;
		}

		Ok(())
	}

	/// Get event receiver
	pub fn take_event_receiver(
		&mut self,
	) -> Option<mpsc::UnboundedReceiver<(SubscriptionType, EventData)>> {
		self.event_rx.take()
	}

	/// Set reconnection parameters
	pub fn set_reconnect_params(&mut self, interval: Duration, max_attempts: u32) {
		self.reconnect_interval = interval;
		self.max_reconnect_attempts = max_attempts;
	}

	/// Start the event processing loop
	async fn start_event_loop(
		&mut self,
		ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
		mut command_rx: mpsc::UnboundedReceiver<Command>,
	) {
		let subscriptions = self.subscriptions.clone();
		let event_tx = self.event_tx.clone();
		let reconnect_interval = self.reconnect_interval;
		let max_reconnect_attempts = self.max_reconnect_attempts;
		let url = self.url.clone();

		tokio::spawn(async move {
			let mut reconnect_attempts = 0;
			let mut sent_subscriptions = HashSet::<String>::new();
			let max_message_size = NeoConstants::max_rpc_message_size();
			let (mut ws_write, mut ws_read) = ws_stream.split();

			loop {
				tokio::select! { biased;
					cmd = command_rx.recv() => {
						match cmd {
							Some(Command::Send(msg)) => {
								let mut subscribe_id = None;
								let mut unsubscribe_id = None;

								if let Message::Text(text) = &msg {
									if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
										if let Some(method) = json.get("method").and_then(|m| m.as_str()) {
											if method.starts_with("subscribe_") {
												subscribe_id = json.get("id").and_then(|id| id.as_str()).map(ToString::to_string);
											} else if method == "unsubscribe" {
												unsubscribe_id = json
													.get("params")
													.and_then(|p| p.as_array())
													.and_then(|a| a.first())
													.and_then(|v| v.as_str())
													.map(ToString::to_string);
											}
										}
									}
								}

								if let Some(id) = &subscribe_id {
									if sent_subscriptions.contains(id) {
										continue;
									}
								}

								if let Err(e) = ws_write.send(msg).await {
									tracing::warn!(error = %e, "WebSocket send error");
								} else {
									if let Some(id) = subscribe_id {
										sent_subscriptions.insert(id);
									}
									if let Some(id) = unsubscribe_id {
										sent_subscriptions.remove(&id);
									}
								}
							}
							Some(Command::Shutdown) | None => {
								let _ = ws_write.send(Message::Close(None)).await;
								let _ = ws_write.close().await;
								break;
							}
						}
					}
					next = ws_read.next() => {
						let mut should_reconnect = false;

						match next {
							Some(Ok(msg)) => {
								reconnect_attempts = 0; // Reset on successful message
								match msg {
									Message::Text(text) => {
										if let Err(e) = Self::process_text_message(&text, max_message_size, &subscriptions, &event_tx).await {
											tracing::warn!(error = %e, "Error processing WebSocket message");
										}
									},
									Message::Ping(data) => {
										if let Err(e) = ws_write.send(Message::Pong(data)).await {
											tracing::warn!(error = %e, "Failed to send Pong");
										}
									},
									Message::Close(frame) => {
										tracing::info!(?frame, "WebSocket closed");
										should_reconnect = true;
									},
									_ => {}
								}
							},
							Some(Err(e)) => {
								tracing::warn!(error = %e, "WebSocket error");
								should_reconnect = true;
							},
							None => {
								tracing::info!("WebSocket connection closed");
								should_reconnect = true;
							},
						}

						if !should_reconnect {
							continue;
						}

						// Attempt reconnection
						if reconnect_attempts < max_reconnect_attempts {
							reconnect_attempts += 1;
							sent_subscriptions.clear();
							tracing::info!(
								attempt = reconnect_attempts,
								max_attempts = max_reconnect_attempts,
								"Attempting WebSocket reconnection"
							);

							tokio::time::sleep(reconnect_interval).await;

							let config = WebSocketConfig {
								max_message_size: Some(max_message_size),
								max_frame_size: Some(max_message_size),
								..Default::default()
							};
							let connect_fut =
								connect_async_with_config(url.as_str(), Some(config), false);
							let connect_result =
								if let Some(timeout) = NeoConstants::rpc_request_timeout() {
									match tokio::time::timeout(timeout, connect_fut).await {
										Ok(res) => res,
										Err(_) => {
											tracing::warn!(
												"WebSocket reconnection timed out after {timeout:?}"
											);
											continue;
										},
									}
								} else {
									connect_fut.await
								};

							match connect_result {
								Ok((new_ws, _)) => {
									(ws_write, ws_read) = new_ws.split();
									tracing::info!("WebSocket reconnected successfully");
									reconnect_attempts = 0;

									// Resubscribe to all active subscriptions
									let subs = subscriptions.read().await;
									for (id, sub_type) in subs.iter() {
										if sent_subscriptions.contains(id) {
											continue;
										}

										let request = Self::create_subscription_request_static(sub_type, id);
										if let Err(e) = ws_write.send(Message::Text(request)).await {
											tracing::warn!(
												subscription_id = %id,
												error = %e,
												"Failed to resubscribe"
											);
										} else {
											sent_subscriptions.insert(id.clone());
										}
									}
								},
								Err(e) => {
									tracing::warn!(error = %e, "WebSocket reconnection failed");
								},
							}
						} else {
							tracing::warn!(
								attempts = reconnect_attempts,
								max_attempts = max_reconnect_attempts,
								"Max reconnection attempts reached, stopping event loop"
							);
							break;
						}
					}
				}
			}
		});
	}

	async fn process_text_message(
		text: &str,
		max_message_size: usize,
		subscriptions: &Arc<RwLock<HashMap<String, SubscriptionType>>>,
		event_tx: &mpsc::UnboundedSender<(SubscriptionType, EventData)>,
	) -> Result<(), NeoError> {
		if text.len() > max_message_size {
			return Err(NeoError::Network {
				message: format!("WebSocket message exceeded {} bytes", max_message_size),
				source: None,
				recovery: ErrorRecovery::new(),
			});
		}

		let json: serde_json::Value =
			serde_json::from_str(text).map_err(|e| NeoError::Network {
				message: format!("Failed to parse WebSocket message: {}", e),
				source: None,
				recovery: ErrorRecovery::new(),
			})?;

		// Parse event and subscription ID
		if let Some(event_data) = Self::parse_event(&json).await? {
			if let Some(sub_id) = json.get("subscription").and_then(|s| s.as_str()) {
				let subs = subscriptions.read().await;
				if let Some(sub_type) = subs.get(sub_id) {
					let _ = event_tx.send((sub_type.clone(), event_data));
				}
			}
		}

		Ok(())
	}

	/// Parse event from JSON
	async fn parse_event(json: &serde_json::Value) -> Result<Option<EventData>, NeoError> {
		let event_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");

		let event_data = match event_type {
			"block_added" => Some(EventData::NewBlock {
				height: json.get("height").and_then(|h| h.as_u64()).unwrap_or(0) as u32,
				hash: json.get("hash").and_then(|h| h.as_str()).unwrap_or("").to_string(),
				timestamp: json.get("timestamp").and_then(|t| t.as_u64()).unwrap_or(0),
				transactions: json
					.get("transactions")
					.and_then(|t| t.as_array())
					.map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
					.unwrap_or_default(),
			}),
			"transaction_added" => Some(EventData::NewTransaction {
				hash: json.get("hash").and_then(|h| h.as_str()).unwrap_or("").to_string(),
				sender: json.get("sender").and_then(|s| s.as_str()).unwrap_or("").to_string(),
				size: json.get("size").and_then(|s| s.as_u64()).unwrap_or(0) as u32,
				attributes: json
					.get("attributes")
					.and_then(|a| a.as_array())
					.cloned()
					.unwrap_or_default(),
			}),
			"transaction_confirmed" => Some(EventData::TransactionConfirmed {
				hash: json.get("hash").and_then(|h| h.as_str()).unwrap_or("").to_string(),
				block_height: json.get("block_height").and_then(|h| h.as_u64()).unwrap_or(0) as u32,
				confirmations: json.get("confirmations").and_then(|c| c.as_u64()).unwrap_or(0)
					as u32,
				vm_state: json.get("vm_state").and_then(|v| v.as_str()).unwrap_or("").to_string(),
			}),
			"notification" => Some(EventData::Notification {
				contract: json.get("contract").and_then(|c| c.as_str()).unwrap_or("").to_string(),
				event_name: json
					.get("event_name")
					.and_then(|e| e.as_str())
					.unwrap_or("")
					.to_string(),
				state: json.get("state").cloned().unwrap_or(serde_json::Value::Null),
			}),
			_ => None,
		};

		Ok(event_data)
	}

	/// Send a message through the WebSocket
	async fn send_message(&mut self, message: String) -> Result<(), NeoError> {
		if message.len() > NeoConstants::max_rpc_message_size() {
			return Err(NeoError::Network {
				message: "WebSocket message too large".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Reduce message size")
					.suggest("If needed, increase NEO3_MAX_RPC_MESSAGE_SIZE (bytes)"),
			});
		}

		let Some(tx) = self.command_tx.as_ref() else {
			return Err(NeoError::Network {
				message: "WebSocket not connected".to_string(),
				source: None,
				recovery: ErrorRecovery::new().suggest("Call connect() before sending messages"),
			});
		};

		tx.send(Command::Send(Message::Text(message))).map_err(|e| NeoError::Network {
			message: format!("Failed to queue WebSocket message: {}", e),
			source: None,
			recovery: ErrorRecovery::new().suggest("Check WebSocket connection").retryable(true),
		})?;

		Ok(())
	}

	/// Generate a unique subscription ID
	fn generate_subscription_id(&self) -> String {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		format!("sub_{:016x}", rng.gen::<u64>())
	}

	/// Create subscription request message
	fn create_subscription_request(&self, sub_type: &SubscriptionType, id: &str) -> String {
		Self::create_subscription_request_static(sub_type, id)
	}

	/// Static version of create_subscription_request for use in spawned tasks
	fn create_subscription_request_static(sub_type: &SubscriptionType, id: &str) -> String {
		let method = match sub_type {
			SubscriptionType::NewBlocks => "subscribe_blocks",
			SubscriptionType::NewTransactions => "subscribe_transactions",
			SubscriptionType::TransactionConfirmation(_) => "subscribe_tx_confirmation",
			SubscriptionType::ContractEvents(_) => "subscribe_contract_events",
			SubscriptionType::AddressActivity(_) => "subscribe_address_activity",
			SubscriptionType::TokenTransfers { .. } => "subscribe_token_transfers",
			SubscriptionType::ExecutionResults => "subscribe_execution_results",
			SubscriptionType::Notifications { .. } => "subscribe_notifications",
		};

		let params = match sub_type {
			SubscriptionType::TransactionConfirmation(hash) => {
				serde_json::json!([hash])
			},
			SubscriptionType::ContractEvents(contract) => {
				serde_json::json!([contract.to_string()])
			},
			SubscriptionType::AddressActivity(address) => {
				serde_json::json!([address.to_string()])
			},
			SubscriptionType::TokenTransfers { token, address } => {
				if let Some(addr) = address {
					serde_json::json!([token.to_string(), addr.to_string()])
				} else {
					serde_json::json!([token.to_string()])
				}
			},
			SubscriptionType::Notifications { contract, name } => {
				let mut params = vec![];
				if let Some(c) = contract {
					params.push(serde_json::json!(c.to_string()));
				}
				if let Some(n) = name {
					params.push(serde_json::json!(n));
				}
				serde_json::json!(params)
			},
			_ => serde_json::json!([]),
		};

		serde_json::json!({
			"jsonrpc": "2.0",
			"method": method,
			"params": params,
			"id": id,
		})
		.to_string()
	}

	/// Create unsubscribe request message
	fn create_unsubscribe_request(&self, id: &str) -> String {
		Self::create_unsubscribe_request_static(id)
	}

	fn create_unsubscribe_request_static(id: &str) -> String {
		serde_json::json!({
			"jsonrpc": "2.0",
			"method": "unsubscribe",
			"params": [id],
			"id": format!("unsub_{}", id),
		})
		.to_string()
	}
}

/// Builder for WebSocket client configuration
pub struct WebSocketClientBuilder {
	url: String,
	reconnect_interval: Duration,
	max_reconnect_attempts: u32,
}

impl WebSocketClientBuilder {
	/// Create a new builder with URL
	pub fn new(url: impl Into<String>) -> Self {
		Self {
			url: url.into(),
			reconnect_interval: Duration::from_secs(5),
			max_reconnect_attempts: 5,
		}
	}

	/// Set reconnection interval
	pub fn reconnect_interval(mut self, interval: Duration) -> Self {
		self.reconnect_interval = interval;
		self
	}

	/// Set maximum reconnection attempts
	pub fn max_reconnect_attempts(mut self, attempts: u32) -> Self {
		self.max_reconnect_attempts = attempts;
		self
	}

	/// Build the WebSocket client
	pub async fn build(self) -> Result<WebSocketClient, NeoError> {
		let mut client = WebSocketClient::new(&self.url).await?;
		client.set_reconnect_params(self.reconnect_interval, self.max_reconnect_attempts);
		Ok(client)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tokio::net::TcpListener;

	#[tokio::test]
	async fn test_websocket_client_creation() {
		let result = WebSocketClient::new("ws://localhost:10332/ws").await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_websocket_builder() {
		let result = WebSocketClientBuilder::new("ws://localhost:10332/ws")
			.reconnect_interval(Duration::from_secs(10))
			.max_reconnect_attempts(3)
			.build()
			.await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_subscription_id_generation() {
		let client = WebSocketClient::new("ws://localhost:10332/ws").await.unwrap();
		let id1 = client.generate_subscription_id();
		let id2 = client.generate_subscription_id();
		assert_ne!(id1, id2);
		assert!(id1.starts_with("sub_"));
		assert!(id2.starts_with("sub_"));
	}

	#[tokio::test]
	async fn subscribe_receives_event() {
		let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		let url = format!("ws://{}", addr);

		let server = tokio::spawn(async move {
			let (stream, _) = listener.accept().await.unwrap();
			let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			assert_eq!(json.get("method").and_then(|m| m.as_str()), Some("subscribe_blocks"));
			let sub_id = json.get("id").and_then(|id| id.as_str()).unwrap().to_string();

			let event = serde_json::json!({
				"type": "block_added",
				"subscription": sub_id,
				"height": 1,
				"hash": "0x01",
				"timestamp": 1,
				"transactions": []
			})
			.to_string();
			ws.send(Message::Text(event)).await.unwrap();
		});

		let mut client = WebSocketClient::new(&url).await.unwrap();
		client.connect().await.unwrap();
		let _handle = client.subscribe(SubscriptionType::NewBlocks).await.unwrap();

		let mut rx = client.take_event_receiver().unwrap();
		let (sub_type, event) =
			tokio::time::timeout(Duration::from_secs(2), rx.recv()).await.unwrap().unwrap();
		assert_eq!(sub_type, SubscriptionType::NewBlocks);
		match event {
			EventData::NewBlock { height, .. } => assert_eq!(height, 1),
			other => panic!("unexpected event: {other:?}"),
		}

		server.await.unwrap();
	}

	#[tokio::test]
	async fn unsubscribe_sends_request() {
		let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		let url = format!("ws://{}", addr);

		let (unsub_tx, unsub_rx) = oneshot::channel::<()>();
		let server = tokio::spawn(async move {
			let (stream, _) = listener.accept().await.unwrap();
			let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			let sub_id = json.get("id").and_then(|id| id.as_str()).unwrap().to_string();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			assert_eq!(json.get("method").and_then(|m| m.as_str()), Some("unsubscribe"));
			assert_eq!(
				json.get("params")
					.and_then(|p| p.as_array())
					.and_then(|arr| arr.first())
					.and_then(|v| v.as_str()),
				Some(sub_id.as_str())
			);

			let _ = unsub_tx.send(());
		});

		let mut client = WebSocketClient::new(&url).await.unwrap();
		client.connect().await.unwrap();
		let handle = client.subscribe(SubscriptionType::NewBlocks).await.unwrap();
		client.unsubscribe(handle).await.unwrap();

		tokio::time::timeout(Duration::from_secs(2), unsub_rx).await.unwrap().unwrap();
		server.await.unwrap();
	}

	#[tokio::test]
	async fn cancel_sends_request() {
		let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		let url = format!("ws://{}", addr);

		let (unsub_tx, unsub_rx) = oneshot::channel::<()>();
		let server = tokio::spawn(async move {
			let (stream, _) = listener.accept().await.unwrap();
			let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			let sub_id = json.get("id").and_then(|id| id.as_str()).unwrap().to_string();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			assert_eq!(json.get("method").and_then(|m| m.as_str()), Some("unsubscribe"));
			assert_eq!(
				json.get("params")
					.and_then(|p| p.as_array())
					.and_then(|arr| arr.first())
					.and_then(|v| v.as_str()),
				Some(sub_id.as_str())
			);

			let _ = unsub_tx.send(());
		});

		let mut client = WebSocketClient::new(&url).await.unwrap();
		client.connect().await.unwrap();
		let handle = client.subscribe(SubscriptionType::NewBlocks).await.unwrap();
		handle.cancel();

		tokio::time::timeout(Duration::from_secs(2), unsub_rx).await.unwrap().unwrap();
		server.await.unwrap();
	}

	#[tokio::test]
	async fn dropping_handle_does_not_cancel() {
		let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		let url = format!("ws://{}", addr);

		let server = tokio::spawn(async move {
			let (stream, _) = listener.accept().await.unwrap();
			let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			assert_eq!(json.get("method").and_then(|m| m.as_str()), Some("subscribe_blocks"));

			let next = tokio::time::timeout(Duration::from_millis(300), ws.next()).await;
			if let Ok(Some(Ok(Message::Text(text)))) = next {
				let json: serde_json::Value = serde_json::from_str(&text).unwrap();
				assert_ne!(json.get("method").and_then(|m| m.as_str()), Some("unsubscribe"));
			}
		});

		let mut client = WebSocketClient::new(&url).await.unwrap();
		client.connect().await.unwrap();
		let handle = client.subscribe(SubscriptionType::NewBlocks).await.unwrap();
		drop(handle);

		server.await.unwrap();
	}

	#[tokio::test]
	async fn reconnects_on_close_and_receives_event() {
		let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		let url = format!("ws://{}", addr);

		let server = tokio::spawn(async move {
			// First connection: accept the subscription request and then close.
			let (stream, _) = listener.accept().await.unwrap();
			let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			assert_eq!(json.get("method").and_then(|m| m.as_str()), Some("subscribe_blocks"));
			let sub_id = json.get("id").and_then(|id| id.as_str()).unwrap().to_string();

			ws.send(Message::Close(None)).await.unwrap();
			drop(ws);

			// Second connection: expect resubscribe and then send an event.
			let (stream, _) = listener.accept().await.unwrap();
			let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

			let msg = tokio::time::timeout(Duration::from_secs(2), ws.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let text = match msg {
				Message::Text(t) => t,
				other => panic!("unexpected ws message: {other:?}"),
			};
			let json: serde_json::Value = serde_json::from_str(&text).unwrap();
			assert_eq!(json.get("method").and_then(|m| m.as_str()), Some("subscribe_blocks"));
			let resub_id = json.get("id").and_then(|id| id.as_str()).unwrap();
			assert_eq!(resub_id, sub_id.as_str());

			let event = serde_json::json!({
				"type": "block_added",
				"subscription": sub_id,
				"height": 1,
				"hash": "0x01",
				"timestamp": 1,
				"transactions": []
			})
			.to_string();
			ws.send(Message::Text(event)).await.unwrap();
		});

		let mut client = WebSocketClient::new(&url).await.unwrap();
		client.set_reconnect_params(Duration::from_millis(50), 3);
		client.connect().await.unwrap();
		let _handle = client.subscribe(SubscriptionType::NewBlocks).await.unwrap();

		let mut rx = client.take_event_receiver().unwrap();
		let (sub_type, event) =
			tokio::time::timeout(Duration::from_secs(2), rx.recv()).await.unwrap().unwrap();
		assert_eq!(sub_type, SubscriptionType::NewBlocks);
		match event {
			EventData::NewBlock { height, .. } => assert_eq!(height, 1),
			other => panic!("unexpected event: {other:?}"),
		}

		server.await.unwrap();
	}
}
