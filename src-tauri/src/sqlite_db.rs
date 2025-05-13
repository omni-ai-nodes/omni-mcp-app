use rusqlite::{Connection, Result};
use dirs;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let app_dir = dirs::data_dir()
            .expect("无法获取应用数据目录")
            .join("omni-mcp-app");
        
        std::fs::create_dir_all(&app_dir).expect("无法创建应用数据目录");
        let db_path = app_dir.join("omni_mcp.db");
        println!("数据库路径: {:?}", db_path);
        
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
    
    // 检查表是否存在
    pub fn table_exists(&self, table_name: &str) -> bool {
        self.conn.query_row(
            &format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", table_name),
            [],
            |_| Ok(true)
        ).unwrap_or(false)
    }
    
    // 检查列是否存在
    pub fn column_exists(&self, table: &str, column: &str) -> bool {
        let query = format!("PRAGMA table_info({})", table);
        let mut stmt = self.conn.prepare(&query).unwrap();
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
    
    // 创建表
    pub fn create_table(&self, sql: &str) -> Result<()> {
        self.conn.execute(sql, [])?;
        Ok(())
    }
    
    // 添加列
    pub fn add_column(&self, table: &str, column: &str, column_type: &str) -> Result<()> {
        let sql = format!("ALTER TABLE {} ADD COLUMN {} {}", table, column, column_type);
        self.conn.execute(&sql, [])?;
        Ok(())
    }
    
    // 执行插入或更新操作
    #[allow(dead_code)]
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        self.conn.execute(sql, params)
    }
    
    // 初始化模型配置表
    pub fn init_model_configs_table(&self) -> Result<()> {
        if self.table_exists("model_configs") {
            // 检查并添加必要的列
            if !self.column_exists("model_configs", "api_url") {
                self.add_column("model_configs", "api_url", "TEXT")?;
            }
            
            if !self.column_exists("model_configs", "endpoint") {
                self.add_column("model_configs", "endpoint", "TEXT")?;
            }
            
            if !self.column_exists("model_configs", "method") {
                self.add_column("model_configs", "method", "TEXT")?;
            }
        } else {
            // 创建新表
            self.create_table(
                "CREATE TABLE model_configs (
                    provider TEXT PRIMARY KEY,
                    api_url TEXT,
                    model TEXT,
                    session_key TEXT,
                    endpoint TEXT,
                    method TEXT
                )"
            )?;
        }
        Ok(())
    }
    
    pub fn init_mcp_servers_table(&self) -> Result<()> {
        if !self.table_exists("mcpServers") {
            self.create_table(
                "CREATE TABLE mcpServers (
                    server_name TEXT PRIMARY KEY,
                    description TEXT,
                    type_ TEXT,
                    base_url TEXT,
                    command TEXT,
                    args TEXT,
                    disabled BOOLEAN,
                    pid INTEGER,
                    install_dir TEXT,
                    env TEXT
                )"
            )?;
        } else {
            // 检查并添加新的列
            let columns = [
                ("install_dir", "TEXT"),
                ("pid", "INTEGER"),
                ("env", "TEXT")
            ];
            
            for (column_name, column_type) in columns.iter() {
                if !self.column_exists("mcpServers", column_name) {
                    self.add_column("mcpServers", column_name, column_type)?;
                }
            }
        }
        Ok(())
    }
}