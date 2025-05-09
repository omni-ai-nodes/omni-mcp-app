use std::path;

use rusqlite::{Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::sqlite_db::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct McpServerConfig {
    pub server_name: String,
    pub command: String,
    pub args: Vec<String>,
    pub disabled: bool,
}

fn get_db() -> Result<Database, String> {
    Database::new().map_err(|e| format!("初始化数据库失败: {}", e))
}

const TABLE_NAME: &str = "mcpServers";

pub fn save_mcp_server_config(config: McpServerConfig) -> Result<(), String> {
    println!("保存配置: config={:?}", config);
    
    let db = get_db()?;
    db.init_mcp_servers_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    
    // 将args转换为JSON字符串
    let args_json = serde_json::to_string(&config.args)
        .map_err(|e| format!("序列化args失败: {}", e))?;
    
    let insert_sql = format!(
        "INSERT OR REPLACE INTO {} (server_name, command, args, disabled) VALUES (?1, ?2, ?3, ?4)",
        TABLE_NAME
    );
    println!("执行SQL: {}", insert_sql);
    
    let result = conn.execute(
        &insert_sql,
        [
            &config.server_name,
            &config.command,
            &args_json,
            &config.disabled.to_string(),
        ],
    );

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
pub async fn get_mcp_server_config(server_name: String) -> Result<Option<McpServerConfig>, String> {
    let db = get_db()?;
    db.init_mcp_servers_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    
    let mut stmt = match conn.prepare("SELECT command, args, disabled FROM mcpServers WHERE server_name = ?") {
        Ok(stmt) => stmt,
        Err(e) => {
            let error_msg = format!("准备查询语句失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    let result = stmt.query_row([&server_name], |row| {
        let command: String = row.get(0)?;
        let args_json: String = row.get(1)?;
        let disabled: String = row.get(2)?;
        
        let args: Vec<String> = serde_json::from_str(&args_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;
        
        Ok(McpServerConfig {
            server_name: server_name.clone(),
            command,
            args,
            disabled: disabled.parse().unwrap_or(false),
        })
    }).ok();
    
    Ok(result)
}

#[tauri::command]
pub async fn get_all_mcp_servers() -> Result<Vec<McpServerConfig>, String> {
    let db = get_db()?;
    db.init_mcp_servers_table().map_err(|e| format!("初始化表失败: {}", e))?;
    
    let conn = db.get_connection();
    
    let mut stmt = match conn.prepare("SELECT server_name, command, args, disabled FROM mcpServers") {
        Ok(stmt) => stmt,
        Err(e) => {
            let error_msg = format!("准备查询语句失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    let rows = stmt.query_map([], |row| {
        let server_name: String = row.get(0)?;
        let command: String = row.get(1)?;
        let args_json: String = row.get(2)?;
        let disabled: String = row.get(3)?;
        
        let args: Vec<String> = serde_json::from_str(&args_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;
        
        Ok(McpServerConfig {
            server_name,
            command,
            args,
            disabled: disabled.parse().unwrap_or(false),
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
pub fn parse_mcp_config(config: &str) -> Result<String, String> {
    // 首先尝试解析 JSON
    let parsed_result: Result<Value, serde_json::Error> = serde_json::from_str(config);
    println!("开始解析 MCP 配置");
    
    match parsed_result {
        Ok(json_value) => {
            // 获取 mcpServers 对象
            let mcp_servers = json_value.get("mcpServers")
                .ok_or_else(|| "缺少 mcpServers 字段".to_string())?;
                
            // 创建 .env 文件内容
            let mut env_content = String::new();
            let mut success_count = 0;
            let mut error_messages = Vec::new();
            
            // 遍历所有服务器配置
            if let Some(servers) = mcp_servers.as_object() {
                for (server_name, server_config) in servers {
                    // 获取环境变量配置
                    if let Some(env) = server_config.get("env") {
                        if let Some(env_map) = env.as_object() {
                            // 添加服务器名称注释
                            env_content.push_str(&format!("\n# {} 服务器配置\n", server_name));
                            
                            // 添加环境变量
                            for (key, value) in env_map {
                                if let Some(value_str) = value.as_str() {
                                    env_content.push_str(&format!("{}={}\n", key, value_str));
                                }
                            }
                        }
                    }
                    
                    // 提取服务器配置并保存到数据库
                    if let Some(command) = server_config.get("command").and_then(|v| v.as_str()) {
                        let args = server_config.get("args")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(String::from)
                                .collect::<Vec<String>>())
                            .unwrap_or_default();
                            
                        let disabled = server_config.get("disabled")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                            
                        let config = McpServerConfig {
                            server_name: server_name.clone(),
                            command: command.to_string(),
                            args,
                            disabled,
                        };
                        
                        match save_mcp_server_config(config) {
                            Ok(_) => {
                                success_count += 1;
                                println!("成功保存服务器配置: {}", server_name);
                                
                                // 创建服务器目录
                                if let Err(e) = create_mcp_server_dir(&server_name) {
                                    error_messages.push(format!("创建服务器 {} 目录失败: {}", server_name, e));
                                }
                            },
                            Err(e) => {
                                error_messages.push(format!("保存服务器 {} 配置失败: {}", server_name, e));
                            }
                        }
                    }
                    
                    // 创建 .env 文件 - 为每个服务器创建目录和环境变量文件
                    let app_dir = get_mcp_server_dir(&server_name)?;
                    std::fs::create_dir_all(&app_dir)
                        .map_err(|e| format!("创建应用目录失败: {}", e))?;
                    
                    // 创建 .env 文件
                    let env_file_path = app_dir.join(".env");
                    println!("mcpServer .env save: {}", env_file_path.to_str().unwrap());
                    // 写入 .env 文件
                    std::fs::write(&env_file_path, env_content.trim())
                        .map_err(|e| format!("写入 .env 文件失败: {}", e))?;
                }
            }
            
            // 获取用户主目录
            // let home_dir = get_user_home_dir().map_err(|e| format!("获取用户主目录失败: {}", e))?;
            
            // 生成结果消息
            let mut result_message = format!("环境变量已成功写入\n");
            result_message.push_str(&format!("成功保存 {} 个服务器配置\n", success_count));
            
            if !error_messages.is_empty() {
                result_message.push_str("发生以下错误：\n");
                for error in error_messages {
                    result_message.push_str(&format!("- {}\n", error));
                }
            }
            
            Ok(result_message)
        },
        Err(e) => Err(format!("JSON 解析失败: {}", e))
    }
}

// 获取用户主目录，根据不同操作系统返回相应的路径
fn get_user_home_dir() -> Result<path::PathBuf, String> {
    if let Some(home_dir) = dirs::home_dir() {
        Ok(home_dir)
    } else {
        Err("无法获取用户主目录".to_string())
    }
}

fn get_mcp_server_dir(server_name: &str) -> Result<path::PathBuf, String> {
    let home_dir = get_user_home_dir()?;
    let server_dir = home_dir.join(".omni-mcp").join("mcpServer").join(server_name);
    Ok(server_dir)
}
// 创建MCP服务器目录
#[allow(dead_code)]
pub fn create_mcp_server_dir(server_name: &str) -> Result<path::PathBuf, String> {
    let server_dir = get_mcp_server_dir(server_name)?;
    std::fs::create_dir_all(&server_dir)
        .map_err(|e| format!("创建服务器目录失败: {}", e))?;
    
    Ok(server_dir)
}