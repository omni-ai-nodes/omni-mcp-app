use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomConfig {
    pub name: String,
    pub api_key: String,
    pub model: String,
    pub session_key: String,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("config.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS custom_configs (
            name TEXT PRIMARY KEY,
            api_key TEXT,
            model TEXT,
            session_key TEXT
        )",
        [],
    )?;
    Ok(conn)
}

pub fn save_custom_config(config: CustomConfig) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT OR REPLACE INTO custom_configs (name, api_key, model, session_key)
         VALUES (?1, ?2, ?3, ?4)",
        [&config.name, &config.api_key, &config.model, &config.session_key],
    )?;
    Ok(())
}

pub fn get_custom_configs() -> Result<Vec<CustomConfig>> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT name, api_key, model, session_key FROM custom_configs")?;
    let configs = stmt.query_map([], |row| {
        Ok(CustomConfig {
            name: row.get(0)?,
            api_key: row.get(1)?,
            model: row.get(2)?,
            session_key: row.get(3)?,
        })
    })?;
    configs.collect()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelConfig {
    pub api_key: String,
    pub model: String,
    pub session_key: String,
    pub endpoint: Option<String>,
}

pub fn get_model_config(provider: &str) -> Result<ModelConfig> {
    let conn = init_db()?;
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

    let mut stmt = conn.prepare(
        "SELECT api_key, model, session_key, endpoint FROM model_configs WHERE provider = ?1"
    )?;
    
    stmt.query_row([provider], |row| {
        Ok(ModelConfig {
            api_key: row.get(0)?,
            model: row.get(1)?,
            session_key: row.get(2)?,
            endpoint: row.get(3)?,
        })
    })
}

pub fn save_model_config(provider: &str, config: ModelConfig) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT OR REPLACE INTO model_configs (provider, api_key, model, session_key, endpoint)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            provider,
            &config.api_key,
            &config.model,
            &config.session_key,
            &config.endpoint.unwrap_or_default(),
        ],
    )?;
    Ok(())
}