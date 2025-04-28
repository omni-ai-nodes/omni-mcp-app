use std::process::Command;
use log::{info, error, warn};

// 定义支持的工具
pub enum Tool {
    Uv,
    Bun,
    Git,
}

impl Tool {
    // 获取工具的名称
    fn name(&self) -> &str {
        match self {
            Tool::Uv => "uv",
            Tool::Bun => "bun",
            Tool::Git => "git",
        }
    }
    
    // 检查工具是否已安装
    fn check_installed(&self) -> bool {
        let output = if cfg!(target_os = "windows") {
            Command::new("where")
                .arg(self.name())
                .output()
        } else {
            Command::new("which")
                .arg(self.name())
                .output()
        };

        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    // 获取 UV 最新版本号
    fn get_uv_latest_version() -> Result<String, String> {
        println!("正在获取 UV 最新版本号...");
        let output = Command::new("curl")
            .args([
                "-s",
                "https://api.github.com/repos/astral-sh/uv/releases/latest"
            ])
            .output()
            .map_err(|e| format!("获取UV版本失败: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        // println!("GitHub API 响应:\n{}", stdout);
        
        // 直接查找 "tag_name": "x.x.x" 格式
        if let Some(tag_pos) = stdout.find("\"tag_name\": \"") {
            let start = tag_pos + "\"tag_name\": \"".len();
            if let Some(end) = stdout[start..].find('"') {
                let version = &stdout[start..start + end];
                println!("解析到的版本号: {}", version);
                return Ok(version.to_string());
            }
        }
        
        let err_msg = "无法解析UV版本信息".to_string();
        println!("错误: {}", err_msg);
        Err(err_msg)
    }
    
    // 获取安装命令
    fn install_command(&self) -> Result<(String, Vec<String>), String> {
        if self.check_installed() {
            return Err(format!("{} 已经安装", self.name()));
        }
    
        if cfg!(target_os = "windows") {
            match self {
                Tool::Uv => {
                    // 先获取版本号
                    match Self::get_uv_latest_version() {
                        Ok(version) => {
                            println!("成功获取 UV 最新版本: {}", version);
                            Ok((
                                "powershell".to_string(),
                                vec![
                                    "-ExecutionPolicy".to_string(),
                                    "Bypass".to_string(),
                                    "-c".to_string(),
                                    format!("irm https://github.com/astral-sh/uv/releases/download/{}/uv-installer.ps1 | iex",
                                        version),
                                ]
                            ))
                        },
                        Err(e) => Err(format!("获取 UV 版本失败: {}", e))
                    }
                },
                Tool::Bun => Ok((
                    "powershell".to_string(),
                    vec![
                        "-c".to_string(),
                        "irm bun.sh/install.ps1 | iex".to_string()
                    ]
                )),
                Tool::Git => {
                    if cfg!(target_os = "macos") {
                        Ok((
                            "brew".to_string(),
                            vec![
                                "install".to_string(),
                                "git".to_string()
                            ]
                        ))
                    } else {
                        // Linux
                        Ok((
                            "apt-get".to_string(),
                            vec![
                                "install".to_string(),
                                "-y".to_string(),
                                "git".to_string()
                            ]
                        ))
                    }
                },
            }
        } else {
            // Unix-like systems (macOS, Linux)
            match self {
                Tool::Uv => {
                    // 先获取版本号
                    match Self::get_uv_latest_version() {
                        Ok(version) => {
                            println!("成功获取 UV 最新版本: {}", version);
                            Ok((
                                "curl".to_string(),
                                vec![
                                    "--proto".to_string(),
                                    "=https".to_string(),
                                    "--tlsv1.2".to_string(),
                                    "-LsSf".to_string(),
                                    format!("https://github.com/astral-sh/uv/releases/download/{}/uv-installer.sh",
                                        version),
                                    "|".to_string(),
                                    "bash".to_string()
                                ]
                            ))
                        },
                        Err(e) => Err(format!("获取 UV 版本失败: {}", e))
                    }
                },
                Tool::Bun => Ok((
                    "curl".to_string(),
                    vec![
                        "-fsSL".to_string(),
                        "https://bun.sh/install".to_string(),
                        "|".to_string(),
                        "bash".to_string()
                    ]
                )),
                Tool::Git => {
                    if cfg!(target_os = "macos") {
                        Ok((
                            "brew".to_string(),
                            vec![
                                "install".to_string(),
                                "git".to_string()
                            ]
                        ))
                    } else {
                        // Linux
                        Ok((
                            "apt-get".to_string(),
                            vec![
                                "install".to_string(),
                                "-y".to_string(),
                                "git".to_string()
                            ]
                        ))
                    }
                },
            }
        }
    }
}

// 安装工具
pub fn install_tool(tool: &Tool) -> Result<(), String> {
    println!("开始安装 {}", tool.name());
    info!("开始安装 {}", tool.name());
    
    let (cmd, args) = tool.install_command()?;
    println!("执行命令: {} {}", cmd, args.join(" "));
    
    let output = if cfg!(target_os = "windows") {
        Command::new(&cmd)
            .args(&args)
            .output()
    } else {
        // 对于 Unix 系统，使用 sh -c 来执行完整命令
        let full_command = format!("{} {}", cmd, args.join(" "));
        Command::new("sh")
            .arg("-c")
            .arg(&full_command)
            .output()
    };
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // 打印安装过程输出
            if !stdout.is_empty() {
                println!("安装输出:\n{}", stdout);
            }
            
            if !stderr.is_empty() {
                println!("安装错误输出:\n{}", stderr);
            }
            
            if !output.status.success() {
                let err_msg = format!("安装 {} 失败", tool.name());
                error!("{}", err_msg);
                return Err(err_msg);
            }
            
            println!("{} 安装完成！", tool.name());
            info!("{} 安装完成", tool.name());
            Ok(())
        },
        Err(e) => {
            let err_msg = format!("安装 {} 时出错: {}", tool.name(), e);
            println!("错误: {}", err_msg);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

// 检查并安装所有工具
#[tauri::command] // <-- Add this attribute
pub fn check_and_install_tools() -> Result<(), String> {
    info!("开始安装工具");

    // 安装 uv
    println!("准备安装 uv...");
    match install_tool(&Tool::Uv) {
        Ok(_) => info!("uv 安装成功"),
        Err(e) => warn!("uv 安装失败: {}", e),
    }

    // 安装 bun
    println!("准备安装 bun...");
    match install_tool(&Tool::Bun) {
        Ok(_) => info!("bun 安装成功"),
        Err(e) => warn!("bun 安装失败: {}", e),
    }
    
    // 安装 git
    println!("准备安装 git...");
    match install_tool(&Tool::Git) {
        Ok(_) => info!("git 安装成功"),
        Err(e) => warn!("git 安装失败: {}", e),
    }

    Ok(())
}

// 检查工具状态
#[tauri::command]
pub fn check_tools_status() -> Result<serde_json::Value, String> {
    let uv_status = if Tool::Uv.check_installed() {
        "已安装"
    } else {
        "未安装"
    };
    
    let bun_status = if Tool::Bun.check_installed() {
        "已安装"
    } else {
        "未安装"
    };
    
    let git_status = if Tool::Git.check_installed() {
        "已安装"
    } else {
        "未安装"
    };
    
    let status = serde_json::json!({
        "uv": uv_status,
        "bun": bun_status,
        "git": git_status
    });
    
    Ok(status)
}

// 安装单个工具的命令
#[tauri::command]
pub fn install_single_tool(tool: &str) -> Result<(), String> {
    let tool = match tool {
        "uv" => Tool::Uv,
        "bun" => Tool::Bun,
        "git" => Tool::Git,
        _ => return Err("不支持的工具类型".to_string()),
    };
    
    install_tool(&tool)
}