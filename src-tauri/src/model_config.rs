use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use dirs;

#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    provider: String,
    config: String,
}

pub fn init_db() -> Result<Connection> {
    let app_dir = dirs::data_dir()
        .expect("无法获取应用数据目录")
        .join("omni-mcp-app");
    
    // 确保目录存在
    std::fs::create_dir_all(&app_dir).expect("无法创建应用数据目录");
    
    let db_path = app_dir.join("model_config.db");
    
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS model_configs (
            provider TEXT PRIMARY KEY,
            config TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

#[tauri::command]
pub async fn save_model_config(provider: String, config: String) -> Result<(), String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT OR REPLACE INTO model_configs (provider, config) VALUES (?1, ?2)",
        [&provider, &config],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_model_config(provider: String) -> Result<Option<String>, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT config FROM model_configs WHERE provider = ?1")
        .map_err(|e| e.to_string())?;
        
    let config = stmt.query_row([&provider], |row| row.get(0))
        .ok();
        
    Ok(config)
}