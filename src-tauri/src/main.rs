// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tools::check_tools_status,
            tools::install_single_tool,
            tools::check_and_install_tools // <-- Add this line
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
