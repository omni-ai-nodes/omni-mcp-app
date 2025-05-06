// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;
mod commands;
mod github_handler;

use github_handler::open_github_link;
mod model_config;

// Remove this unused import
// use crate::db::ModelConfig;

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
