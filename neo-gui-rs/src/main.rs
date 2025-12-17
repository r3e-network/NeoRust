use eframe::{egui, egui::RichText};
use egui_plot::{Line, Plot, PlotPoints};
use neo3::neo_builder::ScriptBuilder;
use neo3::neo_clients::{APITrait, HttpProvider, RpcClient};
use neo3::neo_protocol::{Account, AccountTrait};
use neo3::neo_types::{ContractParameter, ScriptHash, ScriptHashExtension};
use neo3::sdk::hd_wallet::{HDWallet, HDWalletBuilder};
use neo3::sdk::websocket::{SubscriptionType, WebSocketClient};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rust_decimal::prelude::{FromStr, ToPrimitive};
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::{sleep, Duration};

static VERSION: Lazy<String> = Lazy::new(|| neo3::VERSION.to_string());
const GAS_HASH: &str = "d2a4cff31913016155e38e474a2c06d08be276cf";
const NEO_HASH: &str = "c56f33fc6ecfcd0c225c4ab356fee59390af8560";

fn main() -> eframe::Result<()> {
	let native_options = eframe::NativeOptions {
		viewport: eframe::egui::ViewportBuilder::default()
			.with_inner_size([1024.0, 768.0])
			.with_min_inner_size([800.0, 600.0]),
		follow_system_theme: true,
		..Default::default()
	};

	eframe::run_native(
		"NeoRust GUI (Native)",
		native_options,
		Box::new(|_cc| {
			// Customize fonts or style here if needed
			let rt = Runtime::new().expect("Failed to build tokio runtime");
			let state = Arc::new(Mutex::new(AppState::default()));
			let (tx, rx) = unbounded_channel();
			spawn_background(rx, state.clone(), rt.handle().clone());
			Box::new(NeoGuiApp { state, rt, tx, simulator_script: String::new() })
		}),
	)
}

#[derive(Copy, Clone, Default, PartialEq)]
enum Tab {
	#[default]
	Dashboard,
	Wallet,
	HdWallet,
	Simulator,
	WebSocket,
	Analytics,
	Settings,
}

struct AppState {
	current_tab: Tab,
	network: NetworkInfo,
	wallet_status: String,
	logs: Vec<String>,
	max_logs: usize,
	height_history: Vec<(f64, f64)>,
	client: Option<Arc<RpcClient<HttpProvider>>>,
	poller_running: bool,
	poll_interval_secs: u64,
	last_height: Option<u32>,
	peer_count: Option<usize>,
	version: Option<String>,
	dark_mode: bool,
	accounts: Vec<AccountInfo>,
	wif_input: String,
	hd_wallet: Option<HDWallet>,
	hd_mnemonic: String,
	hd_mnemonic_input: String,
	hd_passphrase: String,
	hd_word_count: usize,
	hd_derivation_path: String,
	hd_accounts: Vec<AccountInfo>,
	ws_url: String,
	ws_status: String,
	ws_connected: bool,
	ws_events: Vec<String>,
	ws_client: Option<WebSocketClient>,
	simulator_result: String,
	ws_subscription: String,
	transfer_to: String,
	transfer_amount: String,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			current_tab: Tab::Dashboard,
			network: NetworkInfo::default(),
			wallet_status: String::new(),
			logs: Vec::new(),
			max_logs: 250,
			height_history: Vec::new(),
			client: None,
			poller_running: false,
			poll_interval_secs: 5,
			last_height: None,
			peer_count: None,
			version: None,
			dark_mode: true,
			accounts: Vec::new(),
			wif_input: String::new(),
			hd_wallet: None,
			hd_mnemonic: String::new(),
			hd_mnemonic_input: String::new(),
			hd_passphrase: String::new(),
			hd_word_count: 12,
			hd_derivation_path: "m/44'/888'/0'/0/0".to_string(),
			hd_accounts: Vec::new(),
			ws_url: "wss://testnet1.neo.org:7443/ws".to_string(),
			ws_status: "WebSocket idle".to_string(),
			ws_connected: false,
			ws_events: Vec::new(),
			ws_client: None,
			simulator_result: "Enter a script and run to simulate".to_string(),
			ws_subscription: "NewBlocks".to_string(),
			transfer_to: String::new(),
			transfer_amount: "1".to_string(),
		}
	}
}

impl AppState {
	fn push_log(&mut self, msg: impl Into<String>) {
		self.logs.push(msg.into());
		if self.logs.len() > self.max_logs {
			let overflow = self.logs.len().saturating_sub(self.max_logs);
			if overflow > 0 {
				self.logs.drain(0..overflow);
			}
		}
	}

	fn record_height(&mut self, height: u32) {
		let x = self.height_history.len() as f64;
		self.height_history.push((x, height as f64));
		if self.height_history.len() > 64 {
			let overflow = self.height_history.len() - 64;
			self.height_history.drain(0..overflow);
		}
	}
}

struct NeoGuiApp {
	state: Arc<Mutex<AppState>>,
	#[allow(dead_code)]
	rt: Runtime,
	tx: UnboundedSender<Action>,
	simulator_script: String,
}

#[derive(Clone)]
struct NetworkInfo {
	endpoint: String,
	network_type: String,
	connected: bool,
	status: String,
}

#[derive(Clone)]
struct AccountInfo {
	address: String,
	scripthash: String,
	wif: Option<String>,
	unclaimed_gas: Option<String>,
	neo_balance: Option<String>,
	gas_balance: Option<String>,
}

impl Default for NetworkInfo {
	fn default() -> Self {
		Self {
			endpoint: "https://testnet1.neo.org:443".to_string(),
			network_type: "testnet".to_string(),
			connected: false,
			status: "Not connected".to_string(),
		}
	}
}

impl NeoGuiApp {
	fn render_sidebar(&mut self, ui: &mut egui::Ui) {
		ui.add_space(10.0);
		ui.heading(RichText::new("NeoRust SDK").size(20.0).strong().color(egui::Color32::from_rgb(0, 229, 153)));
		ui.label(RichText::new("Native GUI · Beta").size(12.0).weak());
		ui.add_space(20.0);

		// Navigation
		self.tab_button(ui, Tab::Dashboard, "📊 Dashboard");
		self.tab_button(ui, Tab::Wallet, "👛 Wallet");
		self.tab_button(ui, Tab::HdWallet, "🔐 HD Wallet");
		self.tab_button(ui, Tab::Simulator, "⚡ Simulator");
		self.tab_button(ui, Tab::WebSocket, "🔌 WebSocket");
		self.tab_button(ui, Tab::Analytics, "📈 Analytics");
		self.tab_button(ui, Tab::Settings, "⚙ Settings");

		ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
			ui.add_space(10.0);
			let v = format!("v{}", *VERSION);
			ui.label(RichText::new(v).weak().size(10.0));
			ui.separator();
			let status = {
				let s = self.state.lock();
				if s.network.connected { "🟢 Online" } else { "🔴 Offline" }
			};
			ui.label(status);
		});
	}

	fn tab_button(&mut self, ui: &mut egui::Ui, tab: Tab, label: &str) {
		let selected = {
			let state = self.state.lock();
			state.current_tab == tab
		};
		
		let color = if selected {
			ui.visuals().widgets.active.bg_fill
		} else {
			egui::Color32::TRANSPARENT
		};

		let text_color = if selected {
			egui::Color32::WHITE
		} else {
			ui.visuals().text_color()
		};

		let btn = egui::Button::new(RichText::new(label).color(text_color).size(14.0))
			.fill(color)
			.frame(false)
			.rounding(4.0)
			.min_size(egui::vec2(ui.available_width(), 32.0));

		if ui.add(btn).clicked() {
			self.state.lock().current_tab = tab;
		}
		ui.add_space(4.0);
	}

	fn render_content(&mut self, ui: &mut egui::Ui) {
		let tab = self.state.lock().current_tab;
		egui::ScrollArea::vertical().show(ui, |ui| {
			ui.add_space(20.0);
			match tab {
				Tab::Dashboard => self.render_dashboard(ui),
				Tab::Wallet => self.render_wallet(ui),
				Tab::HdWallet => self.render_hd_wallet(ui),
				Tab::Simulator => self.render_simulator(ui),
				Tab::WebSocket => self.render_websocket(ui),
				Tab::Analytics => self.render_analytics(ui),
				Tab::Settings => self.render_settings(ui),
			}
			ui.add_space(20.0);
		});
	}

	fn render_dashboard(&mut self, ui: &mut egui::Ui) {
		ui.heading("Dashboard");
		ui.add_space(10.0);

		// Network Connection Card
		egui::Frame::group(ui.style()).rounding(8.0).show(ui, |ui| {
			ui.set_width(ui.available_width());
			ui.heading("Network Connection");
			ui.add_space(10.0);
			
			let (mut endpoint, mut network_type, connected, status) = {
				let state = self.state.lock();
				(
					state.network.endpoint.clone(),
					state.network.network_type.clone(),
					state.network.connected,
					state.network.status.clone(),
				)
			};

			ui.horizontal(|ui| {
				ui.label("Endpoint:");
				ui.add(egui::TextEdit::singleline(&mut endpoint).desired_width(250.0));
			});
			
			ui.horizontal(|ui| {
				ui.label("Network:");
				egui::ComboBox::from_id_source("net_type")
					.selected_text(&network_type)
					.show_ui(ui, |ui| {
						ui.selectable_value(&mut network_type, "mainnet".to_string(), "MainNet");
						ui.selectable_value(&mut network_type, "testnet".to_string(), "TestNet");
						ui.selectable_value(&mut network_type, "custom".to_string(), "Custom");
					});
			});

			ui.add_space(10.0);
			ui.horizontal(|ui| {
				if ui.button(if connected { "Reconnect" } else { "Connect" }).clicked() {
					// Update state locally first so UI reflects change
					{
						let mut s = self.state.lock();
						s.network.endpoint = endpoint.clone();
						s.network.network_type = network_type.clone();
					}
					self.queue_action(Action::Connect {
						endpoint: endpoint.clone(),
						network_type: network_type.clone(),
					});
				}
				if connected && ui.button("Disconnect").clicked() {
					self.queue_action(Action::Disconnect);
				}
			});
			ui.add_space(5.0);
			ui.label(RichText::new(status).italics());
		});

		ui.add_space(20.0);

		// Stats Grid
		let (height, peers, version) = {
			let s = self.state.lock();
			(
				s.last_height.map(|h| h.to_string()).unwrap_or_else(|| "-".into()),
				s.peer_count.map(|p| p.to_string()).unwrap_or_else(|| "-".into()),
				s.version.clone().unwrap_or_else(|| "-".into()),
			)
		};

		egui::Grid::new("dash_stats").spacing([20.0, 20.0]).show(ui, |ui| {
			self.stat_card(ui, "Block Height", &height, "🧱");
			self.stat_card(ui, "Connected Peers", &peers, "🔗");
			self.stat_card(ui, "Node Version", &version, "ℹ");
			ui.end_row();
		});

		ui.add_space(20.0);
		
		// Activity Log
		ui.heading("Recent Activity");
		egui::Frame::window(ui.style()).fill(egui::Color32::from_rgb(10, 10, 15)).show(ui, |ui| {
			egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
				ui.set_min_width(ui.available_width());
				let logs = self.state.lock().logs.clone();
				for entry in logs.iter().rev() {
					ui.monospace(entry);
				}
			});
		});
	}

	fn stat_card(&self, ui: &mut egui::Ui, title: &str, value: &str, icon: &str) {
		egui::Frame::group(ui.style()).rounding(8.0).show(ui, |ui| {
			ui.set_min_width(150.0);
			ui.set_min_height(80.0);
			ui.vertical_centered(|ui| {
				ui.add_space(10.0);
				ui.label(RichText::new(icon).size(24.0));
				ui.add_space(5.0);
				ui.label(RichText::new(value).size(20.0).strong());
				ui.label(RichText::new(title).size(12.0).weak());
				ui.add_space(10.0);
			});
		});
	}

	fn render_wallet(&mut self, ui: &mut egui::Ui) {
		ui.heading("Wallet Management");
		ui.label("Manage your Neo N3 accounts.");
		ui.add_space(20.0);

		// Actions Toolbar
		egui::Frame::none().show(ui, |ui| {
			ui.horizontal(|ui| {
				if ui.button("➕ Create New").clicked() {
					match Account::create() {
						Ok(acc) => {
							let info = AccountInfo {
								address: acc.get_address(),
								scripthash: format!("{}", acc.get_script_hash()),
								wif: acc.key_pair().as_ref().map(|kp| kp.export_as_wif()),
								unclaimed_gas: None,
								neo_balance: None,
								gas_balance: None,
							};
							let mut s = self.state.lock();
							s.accounts.push(info.clone());
							s.push_log(format!("Created account {}", info.address));
						},
						Err(e) => {
							let mut s = self.state.lock();
							s.push_log(format!("Account creation failed: {}", e));
						},
					}
				}
				
				ui.separator();
				
				let mut wif_input = self.state.lock().wif_input.clone();
				ui.add(egui::TextEdit::singleline(&mut wif_input).hint_text("Import WIF...").desired_width(300.0));
				if ui.button("📥 Import").clicked() {
					let wif = wif_input.trim().to_string();
					if !wif.is_empty() {
						match Account::from_wif(&wif) {
							Ok(acc) => {
								let info = AccountInfo {
									address: acc.get_address(),
									scripthash: format!("{}", acc.get_script_hash()),
									wif: Some(wif.clone()),
									unclaimed_gas: None,
									neo_balance: None,
									gas_balance: None,
								};
								let mut s = self.state.lock();
								s.accounts.push(info.clone());
								s.push_log(format!("Imported account {}", info.address));
								wif_input.clear();
							},
							Err(e) => {
								self.state.lock().push_log(format!("WIF import failed: {}", e));
							},
						}
					}
				}
				self.state.lock().wif_input = wif_input;

				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					if ui.button("🔄 Refresh Balances").clicked() {
						self.queue_action(Action::RefreshBalances);
						self.queue_action(Action::FetchBalances);
					}
				});
			});
		});

		ui.add_space(20.0);

		// Accounts Table
		egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
			egui::Grid::new("accounts_grid")
				.striped(true)
				.spacing([20.0, 10.0])
				.min_col_width(100.0)
				.show(ui, |ui| {
					ui.label(RichText::new("Address").strong());
					ui.label(RichText::new("NEO").strong());
					ui.label(RichText::new("GAS").strong());
					ui.label(RichText::new("Unclaimed GAS").strong());
					ui.end_row();

					let accounts = self.state.lock().accounts.clone();
					if accounts.is_empty() {
						ui.label("No accounts");
						ui.end_row();
					} else {
						for acc in accounts {
							ui.monospace(&acc.address);
							ui.label(acc.neo_balance.as_deref().unwrap_or("-"));
							ui.label(acc.gas_balance.as_deref().unwrap_or("-"));
							ui.label(acc.unclaimed_gas.as_deref().unwrap_or("-"));
							ui.end_row();
						}
					}
				});
		});

		ui.add_space(30.0);
		ui.separator();
		ui.add_space(10.0);

		// Transfer Section
		ui.heading("Quick Transfer (GAS)");
		egui::Frame::group(ui.style()).rounding(8.0).show(ui, |ui| {
			ui.set_width(ui.available_width());
			let (mut to, mut amount) = {
				let s = self.state.lock();
				(s.transfer_to.clone(), s.transfer_amount.clone())
			};

			ui.horizontal(|ui| {
				ui.label("Recipient:");
				ui.add(egui::TextEdit::singleline(&mut to).desired_width(300.0));
			});
			ui.add_space(5.0);
			ui.horizontal(|ui| {
				ui.label("Amount:");
				ui.add(egui::TextEdit::singleline(&mut amount).desired_width(100.0));
				ui.label("GAS");
			});
			ui.add_space(10.0);
			if ui.button("📝 Draft & Simulate Transfer").clicked() {
				self.queue_action(Action::DraftTransfer {
					to: to.clone(),
					amount: amount.clone(),
				});
			}

			// Sync back
			{
				let mut s = self.state.lock();
				s.transfer_to = to;
				s.transfer_amount = amount;
			}
		});
	}

	fn render_hd_wallet(&mut self, ui: &mut egui::Ui) {
		ui.heading("HD Wallet Generator");
		ui.label("BIP-39/BIP-44 compliant hierarchical deterministic wallet.");
		ui.add_space(20.0);

		egui::Grid::new("hd_setup_grid").spacing([20.0, 10.0]).show(ui, |ui| {
			ui.label("Mnemonic Setup");
			ui.horizontal(|ui| {
				let mut s = self.state.lock();
				egui::ComboBox::from_id_source("hd_words")
					.selected_text(format!("{} Words", s.hd_word_count))
					.show_ui(ui, |ui| {
						ui.selectable_value(&mut s.hd_word_count, 12, "12 Words");
						ui.selectable_value(&mut s.hd_word_count, 24, "24 Words");
					});
				
				if ui.button("🎲 Generate New").clicked() {
					drop(s); // release lock
					self.generate_hd_wallet();
				}
			});
			ui.end_row();

			ui.label("Passphrase");
			let mut s = self.state.lock();
			ui.add(egui::TextEdit::singleline(&mut s.hd_passphrase).password(true));
			ui.end_row();

			ui.label("Import Existing");
			ui.horizontal(|ui| {
				ui.add(egui::TextEdit::multiline(&mut s.hd_mnemonic_input).desired_rows(2).desired_width(400.0));
				if ui.button("📥 Import").clicked() {
					drop(s);
					self.import_hd_wallet();
				}
			});
			ui.end_row();
		});

		ui.add_space(20.0);

		// Active Wallet Display
		let mnemonic = self.state.lock().hd_mnemonic.clone();
		if !mnemonic.is_empty() {
			egui::Frame::group(ui.style()).rounding(8.0).fill(egui::Color32::from_rgb(20, 20, 25)).show(ui, |ui| {
				ui.set_width(ui.available_width());
				ui.label(RichText::new("Active Mnemonic Phrase").strong().color(egui::Color32::YELLOW));
				ui.add_space(5.0);
				ui.monospace(&mnemonic);
			});

			ui.add_space(20.0);
			ui.heading("Derive Accounts");
			ui.horizontal(|ui| {
				let mut s = self.state.lock();
				ui.label("Path:");
				ui.add(egui::TextEdit::singleline(&mut s.hd_derivation_path).desired_width(200.0));
				if ui.button("⚡ Derive").clicked() {
					drop(s);
					self.derive_hd_account();
				}
			});

			ui.add_space(10.0);
			// Derived List
			let derived = self.state.lock().hd_accounts.clone();
			if !derived.is_empty() {
				egui::Grid::new("derived_grid").striped(true).spacing([20.0, 5.0]).show(ui, |ui| {
					ui.label(RichText::new("Address").strong());
					ui.label(RichText::new("WIF").strong());
					ui.end_row();
					for acc in derived {
						ui.monospace(acc.address);
						ui.monospace(acc.wif.unwrap_or_default());
						ui.end_row();
					}
				});
			}
		}
	}

	fn generate_hd_wallet(&self) {
		let (word_count, passphrase) = {
			let s = self.state.lock();
			(s.hd_word_count, s.hd_passphrase.clone())
		};
		let passphrase_opt = if passphrase.is_empty() { None } else { Some(passphrase.as_str()) };
		
		match HDWallet::generate(word_count, passphrase_opt) {
			Ok(wallet) => {
				let phrase = wallet.mnemonic_phrase().to_string();
				let mut s = self.state.lock();
				s.hd_wallet = Some(wallet);
				s.hd_mnemonic = phrase.clone();
				s.hd_accounts.clear();
				s.push_log(format!("Generated HD wallet ({} words)", word_count));
			},
			Err(e) => {
				self.state.lock().push_log(format!("HD wallet generation failed: {}", e));
			},
		}
	}

	fn import_hd_wallet(&self) {
		let (mnemonic, passphrase) = {
			let s = self.state.lock();
			(s.hd_mnemonic_input.trim().to_string(), s.hd_passphrase.clone())
		};
		
		if mnemonic.is_empty() { return; }
		
		let mut builder = HDWalletBuilder::new().mnemonic(mnemonic.clone());
		if !passphrase.is_empty() {
			builder = builder.passphrase(passphrase);
		}
		
		match builder.build() {
			Ok(wallet) => {
				let phrase = wallet.mnemonic_phrase().to_string();
				let mut s = self.state.lock();
				s.hd_wallet = Some(wallet);
				s.hd_mnemonic = phrase.clone();
				s.hd_accounts.clear();
				s.push_log("Imported HD wallet mnemonic".to_string());
			},
			Err(e) => {
				self.state.lock().push_log(format!("HD wallet import failed: {}", e));
			},
		}
	}

	fn derive_hd_account(&self) {
		let path = { self.state.lock().hd_derivation_path.clone() };
		let mut s = self.state.lock();
		
		if let Some(wallet) = s.hd_wallet.as_mut() {
			match wallet.derive_account(&path) {
				Ok(acc) => {
					let info = AccountInfo {
						address: acc.get_address(),
						scripthash: format!("{}", acc.get_script_hash()),
						wif: acc.key_pair().as_ref().map(|kp| kp.export_as_wif()),
						unclaimed_gas: None,
						neo_balance: None,
						gas_balance: None,
					};
					// Add to HD list if unique
					if !s.hd_accounts.iter().any(|a| a.address == info.address) {
						s.hd_accounts.push(info.clone());
					}
					// Add to main list if unique
					if !s.accounts.iter().any(|a| a.address == info.address) {
						s.accounts.push(info.clone());
					}
					s.push_log(format!("Derived account {} from {}", info.address, path));
				},
				Err(e) => {
					s.push_log(format!("Derivation failed: {}", e));
				},
			}
		}
	}

	fn render_simulator(&mut self, ui: &mut egui::Ui) {
		ui.heading("Transaction Simulator");
		ui.label("Run dry-run simulations of arbitrary scripts.");
		ui.add_space(20.0);

		ui.label(RichText::new("Script (Hex)").strong());
		ui.add(
			egui::TextEdit::multiline(&mut self.simulator_script)
				.code_editor()
				.desired_rows(6)
				.desired_width(f32::INFINITY),
		);

		ui.add_space(10.0);
		if ui.button("▶ Run Simulation").clicked() {
			let script_hex = self.simulator_script.trim().to_string();
			self.queue_action(Action::Simulate { script_hex });
		}

		ui.add_space(20.0);
		ui.label(RichText::new("Simulation Result").strong());
		let result = self.state.lock().simulator_result.clone();
		egui::Frame::window(ui.style()).fill(egui::Color32::from_rgb(10, 10, 15)).show(ui, |ui| {
			ui.set_width(ui.available_width());
			ui.set_min_height(100.0);
			ui.monospace(result);
		});
	}

	fn render_websocket(&mut self, ui: &mut egui::Ui) {
		ui.heading("WebSocket Monitor");
		ui.add_space(20.0);

		let (mut url, mut subscription, connected, status) = {
			let s = self.state.lock();
			(s.ws_url.clone(), s.ws_subscription.clone(), s.ws_connected, s.ws_status.clone())
		};

		egui::Grid::new("ws_grid").spacing([10.0, 10.0]).show(ui, |ui| {
			ui.label("URL");
			ui.add(egui::TextEdit::singleline(&mut url).desired_width(300.0));
			ui.end_row();

			ui.label("Subscription");
			egui::ComboBox::from_id_source("ws_sub")
				.selected_text(&subscription)
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut subscription, "NewBlocks".to_string(), "NewBlocks");
					ui.selectable_value(&mut subscription, "NewTransactions".to_string(), "NewTransactions");
					ui.selectable_value(&mut subscription, "ExecutionResults".to_string(), "ExecutionResults");
				});
			ui.end_row();
		});

		ui.add_space(10.0);
		ui.horizontal(|ui| {
			if ui.button(if connected { "Disconnect" } else { "Connect" }).clicked() {
				if connected {
					self.queue_action(Action::WsDisconnect);
				} else {
					self.queue_action(Action::WsConnect { url: url.clone(), subscription: subscription.clone() });
				}
			}
			ui.label(if connected { "🟢 Connected" } else { "🔴 Disconnected" });
			ui.label(RichText::new(status).italics().weak());
		});

		// Sync back inputs
		{
			let mut s = self.state.lock();
			s.ws_url = url;
			s.ws_subscription = subscription;
		}

		ui.add_space(20.0);
		ui.label(RichText::new("Event Stream").strong());
		egui::Frame::window(ui.style()).fill(egui::Color32::from_rgb(10, 10, 15)).show(ui, |ui| {
			egui::ScrollArea::vertical().max_height(300.0).stick_to_bottom(true).show(ui, |ui| {
				ui.set_width(ui.available_width());
				ui.set_min_height(200.0);
				let events = self.state.lock().ws_events.clone();
				if events.is_empty() {
					ui.label(RichText::new("No events received yet.").weak());
				} else {
					for evt in events {
						ui.monospace(evt);
					}
				}
			});
		});
	}

	fn render_analytics(&mut self, ui: &mut egui::Ui) {
		ui.heading("Analytics");
		ui.label("Real-time blockchain metrics.");
		ui.add_space(20.0);

		let history = self.state.lock().height_history.clone();

		if history.is_empty() {
			ui.centered_and_justified(|ui| {
				ui.label("Connect to a node to visualize data.");
			});
		} else {
			let points = PlotPoints::from_iter(history.iter().map(|(x, y)| [*x, *y]));
			Plot::new("height_plot")
				.height(300.0)
				.show_axes([false, true])
				.show_grid([false, true])
				.allow_zoom(false)
				.allow_drag(false)
				.show(ui, |plot_ui| {
					plot_ui.line(Line::new(points).color(egui::Color32::from_rgb(0, 229, 153)).width(2.0).name("Block Height"));
				});
		}
	}

	fn render_settings(&mut self, ui: &mut egui::Ui) {
		ui.heading("Settings");
		ui.add_space(20.0);

		let mut state = self.state.lock();
		
		egui::Frame::group(ui.style()).show(ui, |ui| {
			ui.set_width(ui.available_width());
			ui.heading("Appearance");
			ui.checkbox(&mut state.dark_mode, "Use Dark Mode");
		});

		ui.add_space(10.0);

		egui::Frame::group(ui.style()).show(ui, |ui| {
			ui.set_width(ui.available_width());
			ui.heading("Performance");
			ui.add(egui::Slider::new(&mut state.poll_interval_secs, 1..=60).text("Poll Interval (s)"));
			ui.add(egui::Slider::new(&mut state.max_logs, 100..=1000).text("Max Log Entries"));
		});

		ui.add_space(10.0);

		egui::Frame::group(ui.style()).show(ui, |ui| {
			ui.set_width(ui.available_width());
			ui.heading("Data Management");
			ui.horizontal(|ui| {
				if ui.button("🗑 Clear Logs").clicked() {
					state.logs.clear();
				}
				if ui.button("📊 Reset Analytics").clicked() {
					state.height_history.clear();
					state.last_height = None;
				}
			});
		});
	}

	fn queue_action(&self, action: Action) {
		let _ = self.tx.send(action);
	}
}

#[derive(Clone)]
enum Action {
	Connect { endpoint: String, network_type: String },
	Disconnect,
	RefreshBalances,
	FetchBalances,
	WsConnect { url: String, subscription: String },
	WsDisconnect,
	Simulate { script_hex: String },
	DraftTransfer { to: String, amount: String },
}

fn spawn_background(
	mut rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
	state: Arc<Mutex<AppState>>,
	handle: tokio::runtime::Handle,
) {
	let background_handle = handle.clone();
	let poll_handle = handle.clone();
	background_handle.spawn(async move {
		while let Some(msg) = rx.recv().await {
			match msg {
				Action::Connect { endpoint, network_type } => {
					{
						let mut s = state.lock();
						s.network.status = "Connecting...".to_string();
						s.push_log(format!("Connecting to {} [{}]", endpoint, network_type));
					}
					let provider = match HttpProvider::new(endpoint.as_str()) {
						Ok(p) => p,
						Err(e) => {
							let mut s = state.lock();
							s.network.status = format!("Error: {}", e);
							s.push_log(format!("Connection failed: {}", e));
							continue;
						},
					};
					let client = RpcClient::new(provider);
					// Probe the node
					let status_result = client.get_block_count().await;
					let mut s = state.lock();
					match status_result {
						Ok(height) => {
							s.network.connected = true;
							s.network.endpoint = endpoint;
							s.network.network_type = network_type;
							s.network.status = format!("Connected · height {}", height);
							s.push_log(format!("Connected. Height: {}", height));
							s.client = Some(Arc::new(client));
							s.last_height = Some(height);
							s.peer_count = None;
							s.version = None;
						},
						Err(e) => {
							s.network.connected = false;
							s.network.status = format!("Error: {}", e);
							s.push_log(format!("Connection failed: {}", e));
							s.client = None;
						},
					}
					// start status poller if not running
					let should_spawn = {
						let s = state.lock();
						s.network.connected && !s.poller_running
					};
					if should_spawn {
						if let Some(c) = state.lock().client.clone() {
							let state_clone = state.clone();
							let poll_handle = poll_handle.clone();
							poll_handle.spawn(status_poller(c, state_clone));
							let mut s = state.lock();
							s.poller_running = true;
						}
					}
				},
				Action::WsConnect { url, subscription } => {
					{
						let mut s = state.lock();
						s.ws_status = "Connecting...".to_string();
						s.ws_events.clear();
						s.push_log(format!("WS connecting to {} ({})", url, subscription));
					}
					match WebSocketClient::new(&url).await {
						Ok(mut client) => {
							let connect_result = client.connect().await;
							if let Err(e) = connect_result {
								let mut s = state.lock();
								s.ws_status = format!("WS error: {}", e);
								s.push_log(format!("WS connect failed: {}", e));
								continue;
							}

							// Subscribe to chosen type
							let sub_type = match subscription.as_str() {
								"NewTransactions" => SubscriptionType::NewTransactions,
								"ExecutionResults" => SubscriptionType::ExecutionResults,
								_ => SubscriptionType::NewBlocks,
							};
							match client.subscribe(sub_type.clone()).await {
								Ok(_) => {
									let mut s = state.lock();
									s.ws_status = format!("WS connected ({})", subscription);
									s.ws_connected = true;
									s.ws_client = Some(client);
									s.push_log(format!("WS subscribed to {}", subscription));
								},
								Err(e) => {
									let mut s = state.lock();
									s.ws_status = format!("WS subscribe failed: {}", e);
									s.push_log(format!("WS subscribe failed: {}", e));
									continue;
								},
							}

							// Spawn event reader
							let mut rx_opt = {
								let mut s = state.lock();
								s.ws_client.as_mut().and_then(|c| c.take_event_receiver())
							};
							if let Some(mut rx) = rx_opt.take() {
								let event_state = state.clone();
								let event_handle = handle.clone();
								event_handle.spawn(async move {
									while let Some((typ, evt)) = rx.recv().await {
										let mut s = event_state.lock();
										let line = format!("{:?}: {:?}", typ, evt);
										s.ws_events.push(line);
										if s.ws_events.len() > 200 {
											let drain = s.ws_events.len() - 200;
											s.ws_events.drain(0..drain);
										}
									}
								});
							}
						},
						Err(e) => {
							let mut s = state.lock();
							s.ws_status = format!("WS error: {}", e);
							s.push_log(format!("WS client init failed: {}", e));
						},
					}
				},
				Action::WsDisconnect => {
					let mut client = {
						let mut s = state.lock();
						s.push_log("WS disconnecting...".to_string());
						s.ws_status = "Disconnecting WS...".to_string();
						s.ws_client.take()
					};
					if let Some(ref mut c) = client {
						let _ = c.disconnect().await;
					}
					let mut s = state.lock();
					s.ws_connected = false;
					s.ws_status = "WebSocket disconnected".to_string();
					s.push_log("WS disconnected".to_string());
					s.ws_subscription = "NewBlocks".to_string();
				},
				Action::Disconnect => {
					{
						let mut s = state.lock();
						s.network.status = "Disconnecting...".to_string();
						s.push_log("Disconnecting...".to_string());
					}
					sleep(Duration::from_millis(300)).await;
					let mut s = state.lock();
					s.network.connected = false;
					s.network.status = "Disconnected".to_string();
					s.push_log("Disconnected.".to_string());
					s.client = None;
					s.poller_running = false;
					s.last_height = None;
				},
				Action::RefreshBalances => {
					let client = { state.lock().client.clone() };
					let accounts = { state.lock().accounts.clone() };
					if let Some(client) = client {
						for acc in accounts {
							let script_hash = match acc.address.parse::<ScriptHash>() {
								Ok(hash) => hash,
								Err(e) => {
									state
										.lock()
										.logs
										.push(format!("Invalid address {}: {}", acc.address, e));
									continue;
								},
							};
							match client.get_unclaimed_gas(script_hash).await {
								Ok(gas) => {
									let mut s = state.lock();
									if let Some(existing) =
										s.accounts.iter_mut().find(|a| a.address == acc.address)
									{
										existing.unclaimed_gas = Some(gas.unclaimed.clone());
									}
									s.push_log(format!(
										"Unclaimed GAS for {}: {}",
										acc.address, gas.unclaimed
									));
								},
								Err(e) => {
									state.lock().push_log(format!(
										"Failed to fetch GAS for {}: {}",
										acc.address, e
									));
								},
							}
						}
					} else {
						state.lock().push_log("Refresh failed: not connected".to_string());
					}
				},
				Action::FetchBalances => {
					let client = { state.lock().client.clone() };
					let accounts = { state.lock().accounts.clone() };
					if let Some(client) = client {
						for acc in accounts {
							let script_hash = match acc.address.parse::<ScriptHash>() {
								Ok(h) => h,
								Err(e) => {
									state
										.lock()
										.logs
										.push(format!("Invalid address {}: {}", acc.address, e));
									continue;
								},
							};
							match client.get_nep17_balances(script_hash).await {
								Ok(balances) => {
									let mut s = state.lock();
									if let Some(existing) =
										s.accounts.iter_mut().find(|a| a.address == acc.address)
									{
										existing.neo_balance = None;
										existing.gas_balance = None;
										for bal in &balances.balances {
											let hash = bal.asset_hash.to_string().to_lowercase();
											let normalized = hash.trim_start_matches("0x");
											if normalized == GAS_HASH {
												existing.gas_balance = Some(bal.amount.clone());
											} else if normalized == NEO_HASH {
												existing.neo_balance = Some(bal.amount.clone());
											}
										}
									}
									s.push_log(format!(
										"Fetched NEP-17 balances for {}",
										acc.address
									));
								},
								Err(e) => {
									state.lock().push_log(format!(
										"Balance fetch failed for {}: {}",
										acc.address, e
									));
								},
							}
						}
					} else {
						state.lock().push_log("Balance fetch failed: not connected".to_string());
					}
				},
				Action::Simulate { script_hex } => {
					let client = { state.lock().client.clone() };
					if client.is_none() {
						let mut s = state.lock();
						s.push_log("Simulation failed: not connected".to_string());
						continue;
					}
					let script_bytes = match hex::decode(script_hex.trim_start_matches("0x")) {
						Ok(b) => b,
						Err(e) => {
							let mut s = state.lock();
							s.push_log(format!("Simulation failed: invalid hex - {}", e));
							continue;
						},
					};
					let response = client
						.as_ref()
						.unwrap()
						.invoke_script(hex::encode(script_bytes), vec![])
						.await;
					let mut s = state.lock();
					match response {
						Ok(result) => {
							s.push_log("Simulation success".to_string());
							let summary = format!(
								"State: {:?} · Gas: {} · Stack items: {}",
								result.state,
								result.gas_consumed,
								result.stack.len()
							);
							s.push_log(summary.clone());
							s.simulator_result = summary;
						},
						Err(e) => {
							let msg = format!("Simulation error: {}", e);
							s.push_log(msg.clone());
							s.simulator_result = msg;
						},
					}
				},
				Action::DraftTransfer { to, amount } => {
					let sender = state.lock().accounts.first().cloned();
					let sender = match sender {
						Some(acc) => acc,
						None => {
							state.lock().push_log(
								"Transfer draft failed: create or import an account first"
									.to_string(),
							);
							continue;
						},
					};
					let value = match Decimal::from_str(&amount) {
						Ok(v) => v,
						Err(e) => {
							state.lock().push_log(format!("Invalid amount {}: {}", amount, e));
							continue;
						},
					};
					let _to_hash = match to.parse::<ScriptHash>() {
						Ok(h) => h,
						Err(e) => {
							state.lock().push_log(format!("Invalid recipient {}: {}", to, e));
							continue;
						},
					};

					let _from_hash = match ScriptHash::from_address(&sender.address) {
						Ok(h) => h,
						Err(e) => {
							state
								.lock()
								.logs
								.push(format!("Invalid sender address {}: {}", sender.address, e));
							continue;
						},
					};

					// Build GAS transfer script
					let _amount_i32 = value.try_into().unwrap_or(0);

					// Best-effort invoke_script estimation
					let from_hash = match ScriptHash::from_address(&sender.address) {
						Ok(h) => h,
						Err(e) => {
							state
								.lock()
								.logs
								.push(format!("Invalid sender address {}: {}", sender.address, e));
							continue;
						},
					};
					let to_hash = match to.parse::<ScriptHash>() {
						Ok(h) => h,
						Err(e) => {
							state.lock().push_log(format!("Invalid recipient {}: {}", to, e));
							continue;
						},
					};
					let scaled = (value * Decimal::from(100_000_000u64)).to_i64().unwrap_or(0);
					let mut script_builder = ScriptBuilder::new();
					let gas_hash =
						ScriptHash::from_hex(GAS_HASH).unwrap_or_else(|_| ScriptHash::zero());
					let params = vec![
						ContractParameter::h160(&from_hash),
						ContractParameter::h160(&to_hash),
						ContractParameter::integer(scaled),
						ContractParameter::any(),
					];
					let script =
						match script_builder.contract_call(&gas_hash, "transfer", &params, None) {
							Ok(_) => script_builder.to_bytes(),
							Err(e) => {
								state
									.lock()
									.logs
									.push(format!("Failed to build transfer script: {}", e));
								continue;
							},
						};
					let client = { state.lock().client.clone() };
					if let Some(client) = client {
						match client.invoke_script(hex::encode(script), vec![]).await {
							Ok(result) => {
								let mut s = state.lock();
								s.push_log(format!(
									"Draft transfer {} GAS -> {} | state={:?} gas={} stack={}",
									amount,
									to,
									result.state,
									result.gas_consumed,
									result.stack.len()
								));
							},
							Err(e) => {
								state.lock().push_log(format!("Draft invoke failed: {}", e));
							},
						}
					} else {
						state
							.lock()
							.logs
							.push("Draft transfer: connect to RPC for estimation".to_string());
					}
				},
			}
		}
	});
}

async fn status_poller(client: Arc<RpcClient<HttpProvider>>, state: Arc<Mutex<AppState>>) {
	loop {
		let wait_secs = {
			let s = state.lock();
			s.poll_interval_secs.max(1)
		};
		sleep(Duration::from_secs(wait_secs)).await;
		let connected = {
			let s = state.lock();
			s.network.connected
		};
		if !connected {
			break;
		}

		match client.get_block_count().await {
			Ok(height) => {
				let mut s = state.lock();
				let changed = s.last_height.map(|h| h != height).unwrap_or(true);
				s.last_height = Some(height);
				s.network.status = format!("Connected · height {}", height);
				s.record_height(height);
				if changed {
					s.push_log(format!("Height: {}", height));
				}
			},
			Err(e) => {
				let mut s = state.lock();
				s.network.status = format!("Error: {}", e);
				s.push_log(format!("Status poll failed: {}", e));
			},
		}

		// version/peers best-effort
		match client.get_version().await {
			Ok(v) => {
				let mut s = state.lock();
				s.version = Some(v.user_agent.clone());
			},
			Err(e) => {
				let mut s = state.lock();
				s.push_log(format!("Version fetch failed: {}", e));
			},
		}

		match client.get_peers().await {
			Ok(peers) => {
				let mut s = state.lock();
				let count = peers.connected.len() + peers.unconnected.len() + peers.bad.len();
				s.peer_count = Some(count);
			},
			Err(e) => {
				let mut s = state.lock();
				s.push_log(format!("Peers fetch failed: {}", e));
			},
		}
	}
	let mut s = state.lock();
	s.poller_running = false;
}

impl eframe::App for NeoGuiApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let dark_mode = { self.state.lock().dark_mode };
		if dark_mode {
			ctx.set_visuals(egui::Visuals::dark());
		} else {
			ctx.set_visuals(egui::Visuals::light());
		}
		
		// Apply a global style tweak
		let mut style = (*ctx.style()).clone();
		style.spacing.button_padding = egui::vec2(10.0, 6.0);
		style.visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
		ctx.set_style(style);

		egui::SidePanel::left("sidebar")
			.resizable(false)
			.default_width(200.0)
			.show(ctx, |ui| {
				self.render_sidebar(ui);
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			self.render_content(ui);
		});
	}
}