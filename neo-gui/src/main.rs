// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

mod commands;
mod services;
mod utils;

use commands::*;
use services::*;

// Application state
#[derive(Default)]
pub struct AppState {
    pub wallets: Mutex<HashMap<String, wallet::WalletInfo>>,
    pub network_client: Mutex<Option<network::NetworkClient>>,
    pub settings: Mutex<settings::AppSettings>,
    pub transactions: Mutex<Vec<transaction::TransactionInfo>>,
}

// Common types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: Utc::now(),
        }
    }
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // Wallet commands
            wallet::create_wallet,
            wallet::open_wallet,
            wallet::close_wallet,
            wallet::list_wallets,
            wallet::get_wallet_info,
            wallet::create_address,
            wallet::import_private_key,
            wallet::export_private_key,
            wallet::get_balance,
            wallet::send_transaction,
            wallet::get_transaction_history,
            
            // Network commands
            network::connect_to_network,
            network::disconnect_from_network,
            network::get_network_status,
            network::get_block_info,
            network::get_transaction_info,
            network::get_contract_info,
            
            // Contract commands
            contract::deploy_contract,
            contract::invoke_contract,
            contract::test_contract,
            contract::get_contract_state,
            
            // DeFi commands
            defi::get_token_info,
            defi::swap_tokens,
            defi::add_liquidity,
            defi::remove_liquidity,
            defi::stake_tokens,
            defi::unstake_tokens,
            defi::get_pool_info,
            
            // NFT commands
            nft::mint_nft,
            nft::transfer_nft,
            nft::get_nft_info,
            nft::list_nfts,
            nft::get_collection_info,
            
            // Settings commands
            settings::get_settings,
            settings::update_settings,
            settings::reset_settings,
            
            // Utility commands
            utils::encode_data,
            utils::decode_data,
            utils::hash_data,
            utils::validate_address,
            utils::format_amount,
        ])
        .setup(|app| {
            // Initialize application
            log::info!("Neo GUI application starting...");
            
            // Load settings
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = settings::load_initial_settings(&app_handle).await {
                    log::error!("Failed to load initial settings: {}", e);
                }
            });
            
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                log::info!("Application closing...");
                // Perform cleanup here
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 