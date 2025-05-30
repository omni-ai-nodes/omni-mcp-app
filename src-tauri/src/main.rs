// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod commands;
mod github_handler;

use github_handler::open_github_link;
mod model_config;
mod save_mcp_config;
mod sqlite_db;
// Remove this unused import
// use crate::db::ModelConfig;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            tools::check_tools_status,
            tools::install_single_tool,
            tools::check_and_install_tools,
            commands::execute_command,
            open_github_link,
            model_config::get_model_config,
            model_config::save_model_config,
            model_config::get_custom_configs,
            model_config::delete_model_config,  // Add this line
            save_mcp_config::parse_mcp_config,
            save_mcp_config::get_all_mcp_servers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
