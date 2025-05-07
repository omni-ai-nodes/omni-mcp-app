use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use dirs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelConfig {
    pub provider: String,  // 添加 provider 字段
    pub api_url: String,
    pub model: String,
    pub session_key: String,
    pub endpoint: Option<String>,
}

// 检查列是否存在的函数
fn column_exists(conn: &Connection, table: &str, column: &str) -> bool {
    let query = format!("PRAGMA table_info({})", table);
    let mut stmt = conn.prepare(&query).unwrap();
    let columns = stmt.query_map([], |row| {
        let name: String = row.get(1)?;
        Ok(name)
    }).unwrap();
    
    for col_result in columns {
        if let Ok(name) = col_result {
            if name == column {
                return true;
            }
        }
    }
    false
}

pub fn init_db() -> Result<Connection> {
    let app_dir = dirs::data_dir()
        .expect("无法获取应用数据目录")
        .join("omni-mcp-app");
    
    std::fs::create_dir_all(&app_dir).expect("无法创建应用数据目录");
    let db_path = app_dir.join("model_config.db");
    println!("数据库路径: {:?}", db_path);
    
    let conn = Connection::open(db_path)?;
    
    // 检查表是否存在
    let table_exists = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='model_configs'",
        [],
        |_| Ok(true)
    ).unwrap_or(false);
    
    println!("表是否存在: {}", table_exists);
    
    if table_exists {
        // 获取并打印表结构信息
        let mut stmt = conn.prepare("PRAGMA table_info(model_configs)").unwrap();
        let columns = stmt.query_map([], |row| {
            let column_name: String = row.get(1)?;
            let column_type: String = row.get(2)?;
            Ok((column_name, column_type))
        }).unwrap();
        
        println!("表结构信息:");
        for column in columns {
            if let Ok((name, type_)) = column {
                println!("列名: {}, 类型: {}", name, type_);
            }
        }
        
        // 使用新的函数检查列是否存在
        let api_url_exists = column_exists(&conn, "model_configs", "api_url");
        println!("api_url列是否存在: {}", api_url_exists);
        
        if !api_url_exists {
            match conn.execute("ALTER TABLE model_configs ADD COLUMN api_url TEXT", []) {
                Ok(_) => println!("成功添加api_url列"),
                Err(e) => println!("添加api_url列失败: {}", e),
            }
        }
        
        let endpoint_exists = column_exists(&conn, "model_configs", "endpoint");
        println!("endpoint列是否存在: {}", endpoint_exists);
        
        if !endpoint_exists {
            match conn.execute("ALTER TABLE model_configs ADD COLUMN endpoint TEXT", []) {
                Ok(_) => println!("成功添加endpoint列"),
                Err(e) => println!("添加endpoint列失败: {}", e),
            }
        }
    } else {
        // 创建新表
        match conn.execute(
            "CREATE TABLE model_configs (
                provider TEXT PRIMARY KEY,
                api_url TEXT,
                model TEXT,
                session_key TEXT,
                endpoint TEXT
            )",
            [],
        ) {
            Ok(_) => println!("成功创建model_configs表"),
            Err(e) => println!("创建表失败: {}", e),
        }
    }
    
    Ok(conn)
}

#[tauri::command]
pub async fn get_model_config(provider: String) -> Result<Option<ModelConfig>, String> {
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            let error_msg = format!("初始化数据库失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    let query = format!("SELECT api_url, model, session_key, endpoint FROM model_configs WHERE provider = '{}'", provider);
    println!("执行查询: {}", query);
    
    let mut stmt = match conn.prepare(&query) {
        Ok(stmt) => stmt,
        Err(e) => {
            let error_msg = format!("准备查询语句失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    let result = stmt.query_row([/*&provider*/], |row| {
        let api_url: Result<String> = row.get(0);
        let model: Result<String> = row.get(1);
        let session_key: Result<String> = row.get(2);
        let endpoint: Result<Option<String>> = row.get(3);
        
        println!("查询结果: api_url={:?}, model={:?}, session_key={:?}, endpoint={:?}", 
                 api_url, model, session_key, endpoint);
        
        Ok(ModelConfig {
            provider: provider.clone(), // Add this line to include the provider field
            api_url: api_url?,
            model: model?,
            session_key: session_key?,
            endpoint: endpoint?,
        })
    }).ok();
    
    Ok(result)
}

#[tauri::command]
pub async fn save_model_config(provider: String, config: ModelConfig) -> Result<(), String> {
    println!("保存配置: provider={}, config={:?}", provider, config);
    
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            let error_msg = format!("初始化数据库失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    // 首先检查是否存在相同的 provider
    let exists = conn.query_row(
        "SELECT COUNT(*) FROM model_configs WHERE provider = ?",
        [&provider],
        |row| row.get::<_, i32>(0)
    ).unwrap_or(0) > 0;

    // Use as_ref() to borrow the Option's contents instead of moving it
    let endpoint_value = config.endpoint.as_ref().map_or_else(String::new, |s| s.clone());
    
    let result = if exists {
        // 如果存在，执行更新操作
        println!("更新已存在的配置: {}", provider);
        conn.execute(
            "UPDATE model_configs SET api_url = ?1, model = ?2, session_key = ?3, endpoint = ?4 WHERE provider = ?5",
            [
                &config.api_url,
                &config.model,
                &config.session_key,
                &endpoint_value,
                &provider,
            ],
        )
    } else {
        // 如果不存在，执行插入操作
        println!("插入新配置: {}", provider);
        conn.execute(
            "INSERT INTO model_configs (provider, api_url, model, session_key, endpoint) VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                &provider,
                &config.api_url,
                &config.model,
                &config.session_key,
                &endpoint_value,
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
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            let error_msg = format!("初始化数据库失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    // 根据 filter_type 参数决定是否添加 WHERE 条件
    let query = match filter_type {
        Some(filter) if filter == "ALL" => {
            "SELECT provider, api_url, model, session_key, endpoint FROM model_configs"
        },
        _ => {
            "SELECT provider, api_url, model, session_key, endpoint FROM model_configs WHERE provider != 'openai' AND provider != 'ollama'"
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
        
        Ok(ModelConfig {
            provider,  // 直接使用 provider
            api_url,
            model,
            session_key,
            endpoint,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut configs = Vec::new();
    for row in rows {  // Change to '_row' if you want to explicitly ignore it
        if let Ok(config) = row {
            configs.push(config);
        }
    }
    
    Ok(configs)
}

#[tauri::command]
pub async fn delete_model_config(provider: String) -> Result<(), String> {
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            let error_msg = format!("初始化数据库失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
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