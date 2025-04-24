// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use log::{info, error};

// 导入工具模块
mod tools;
mod commands;

#[tauri::command]
fn greet(name: &str) -> String {
    info!("Greeting user: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    info!("应用启动");
    
    // 检查并安装工具
    match tools::check_and_install_tools() {
        Ok(_) => info!("工具检查完成"),
        Err(e) => error!("工具检查失败: {}", e),
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::execute_command,
            tools::install_single_tool,
            tools::check_tools_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
