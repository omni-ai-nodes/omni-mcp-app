use rusqlite::{Result};
use serde::{Deserialize, Serialize};
use crate::sqlite_db::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelConfig {
    pub provider: String,
    pub api_url: String,
    pub model: String,
    pub session_key: String,
    pub endpoint: Option<String>,
    pub method: Option<String>,
}

fn get_db() -> Result<Database, String> {
    Database::new().map_err(|e| format!("初始化数据库失败: {}", e))
}

#[tauri::command]
pub async fn get_model_config(provider: String) -> Result<Option<ModelConfig>, String> {
    let db = get_db()?;
    db.init_model_configs_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    let query = "SELECT api_url, model, session_key, endpoint, method FROM model_configs WHERE provider = ?";
    println!("执行查询: {}", query);
    
    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            let error_msg = format!("准备查询语句失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    let result = stmt.query_row([&provider], |row| {
        let api_url: Result<String> = row.get(0);
        let model: Result<String> = row.get(1);
        let session_key: Result<String> = row.get(2);
        let endpoint: Result<Option<String>> = row.get(3);
        let method: Result<Option<String>> = row.get(4);
        
        println!("查询结果: api_url={:?}, model={:?}, session_key={:?}, endpoint={:?}, method={:?}", 
                 api_url, model, session_key, endpoint, method);
        
        Ok(ModelConfig {
            provider: provider.clone(),
            api_url: api_url?,
            model: model?,
            session_key: session_key?,
            endpoint: endpoint?,
            method: method?,
        })
    }).ok();
    
    Ok(result)
}

#[tauri::command]
pub async fn save_model_config(provider: String, config: ModelConfig) -> Result<(), String> {
    println!("保存配置: provider={}, config={:?}", provider, config);
    
    let db = get_db()?;
    db.init_model_configs_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    
    // 首先检查是否存在相同的 provider
    let exists = conn.query_row(
        "SELECT COUNT(*) FROM model_configs WHERE provider = ?",
        [&provider],
        |row| row.get::<_, i32>(0)
    ).unwrap_or(0) > 0;

    // Use as_ref() to borrow the Option's contents instead of moving it
    let endpoint_value = config.endpoint.as_ref().map_or_else(String::new, |s| s.clone());
    let method_value = config.method.as_ref().map_or_else(|| "/v1/chat/completions".to_string(), |s| s.clone());
    
    let result = if exists {
        // 如果存在，执行更新操作
        println!("更新已存在的配置: {}", provider);
        conn.execute(
            "UPDATE model_configs SET api_url = ?1, model = ?2, session_key = ?3, endpoint = ?4, method = ?5 WHERE provider = ?6",
            [
                &config.api_url,
                &config.model,
                &config.session_key,
                &endpoint_value,
                &method_value,
                &provider,
            ],
        )
    } else {
        // 如果不存在，执行插入操作
        println!("插入新配置: {}", provider);
        conn.execute(
            "INSERT INTO model_configs (provider, api_url, model, session_key, endpoint, method) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &provider,
                &config.api_url,
                &config.model,
                &config.session_key,
                &endpoint_value,
                &method_value,
            ],
        )
    };

    match result {
        Ok(_) => {
            println!("保存配置成功");
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("保存配置失败: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
pub async fn get_custom_configs(filter_type: Option<String>) -> Result<Vec<ModelConfig>, String> {
    let db = get_db()?;
    db.init_model_configs_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    
    // 根据 filter_type 参数决定是否添加 WHERE 条件
    let query = match filter_type {
        Some(filter) if filter == "ALL" => {
            "SELECT provider, api_url, model, session_key, endpoint, method FROM model_configs"
        },
        _ => {
            "SELECT provider, api_url, model, session_key, endpoint, method FROM model_configs WHERE provider != 'openai' AND provider != 'ollama'"
        }
    };
    
    println!("执行查询: {}", query);
    
    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            let error_msg = format!("准备查询语句失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    let rows = stmt.query_map([], |row| {
        let provider: String = row.get(0)?;
        let api_url: String = row.get(1)?;
        let model: String = row.get(2)?;
        let session_key: String = row.get(3)?;
        let endpoint: Option<String> = row.get(4)?;
        let method: Option<String> = row.get(5)?;
        
        Ok(ModelConfig {
            provider,
            api_url,
            model,
            session_key,
            endpoint,
            method,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut configs = Vec::new();
    for row in rows {
        if let Ok(config) = row {
            configs.push(config);
        }
    }
    
    Ok(configs)
}

#[tauri::command]
pub async fn delete_model_config(provider: String) -> Result<(), String> {
    let db = get_db()?;
    db.init_model_configs_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    
    match conn.execute(
        "DELETE FROM model_configs WHERE provider = ?",
        [&provider],
    ) {
        Ok(_) => {
            println!("删除配置成功: {}", provider);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("删除配置失败: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}