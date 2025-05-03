// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod commands;
mod github_handler;
mod db;

use github_handler::open_github_link;
use db::CustomConfig;
mod model_config;

#[tauri::command]
async fn save_custom_config(config: CustomConfig) -> Result<(), String> {
    db::save_custom_config(config).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_custom_configs() -> Result<Vec<CustomConfig>, String> {
    db::get_custom_configs().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tools::check_tools_status,
            tools::install_single_tool,
            tools::check_and_install_tools,
            commands::execute_command,
            open_github_link,
            model_config::save_model_config,
            model_config::get_model_config,
            save_custom_config,
            get_custom_configs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
