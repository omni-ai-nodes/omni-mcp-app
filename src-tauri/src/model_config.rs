use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use dirs;

#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    pub api_key: String,
    pub model: String,
    pub session_key: String,
    pub endpoint: Option<String>,
}

pub fn init_db() -> Result<Connection> {
    let app_dir = dirs::data_dir()
        .expect("无法获取应用数据目录")
        .join("omni-mcp-app");
    
    std::fs::create_dir_all(&app_dir).expect("无法创建应用数据目录");
    let db_path = app_dir.join("model_config.db");
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS model_configs (
            provider TEXT PRIMARY KEY,
            api_key TEXT,
            model TEXT,
            session_key TEXT,
            endpoint TEXT
        )",
        [],
    )?;
    
    Ok(conn)
}

#[tauri::command]
pub async fn save_model_config(provider: String, config: ModelConfig) -> Result<(), String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT OR REPLACE INTO model_configs (provider, api_key, model, session_key, endpoint) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            &provider,
            &config.api_key,
            &config.model,
            &config.session_key,
            &config.endpoint.unwrap_or_default(),
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_model_config(provider: String) -> Result<Option<ModelConfig>, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT api_key, model, session_key, endpoint 
         FROM model_configs 
         WHERE provider = ?1"
    ).map_err(|e| e.to_string())?;
    
    let result = stmt.query_row([&provider], |row| {
        Ok(ModelConfig {
            api_key: row.get(0)?,
            model: row.get(1)?,
            session_key: row.get(2)?,
            endpoint: row.get(3)?,
        })
    }).ok();
    
    Ok(result)
}