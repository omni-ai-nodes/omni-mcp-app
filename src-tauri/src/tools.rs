use std::path::Path;
use std::process::Command;
use std::fs;
use log::{info, error, warn};

// 定义支持的工具
pub enum Tool {
    Uv,
    Bun,
}

impl Tool {
    // 获取工具的名称
    fn name(&self) -> &str {
        match self {
            Tool::Uv => "uv",
            Tool::Bun => "bun",
        }
    }
    
    // 获取工具的下载URL
    fn download_url(&self) -> Result<String, String> {
        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "darwin"  // macOS 在发布版本中通常使用 "darwin" 而不是 "macos"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            return Err(format!("不支持的操作系统"));
        };
        
        let arch = if cfg!(target_arch = "x86_64") {
            "x64"  // 很多项目使用 x64 而不是 x86_64
        } else if cfg!(target_arch = "aarch64") {
            "arm64"  // 通常使用 arm64 而不是 aarch64
        } else {
            return Err(format!("不支持的架构"));
        };
        
        match self {
            Tool::Uv => {
                // uv 的下载 URL 格式
                if cfg!(target_os = "windows") {
                    Ok(format!("https://github.com/astral-sh/uv/releases/latest/download/uv-{}-{}.exe", os, arch))
                } else {
                    Ok(format!("https://github.com/astral-sh/uv/releases/latest/download/uv-{}-{}", os, arch))
                }
            },
            Tool::Bun => {
                // bun 的下载 URL 格式
                if cfg!(target_os = "windows") {
                    Ok(format!("https://bun.sh/download/latest/bun-{}-{}.exe", os, arch))
                } else if cfg!(target_os = "macos") {
                    Ok(format!("https://bun.sh/download/latest/bun-{}-{}.zip", os, arch))
                } else {
                    Ok(format!("https://bun.sh/download/latest/bun-{}-{}.zip", os, arch))
                }
            },
        }
    }
}

// 检查工具是否已安装
pub fn is_tool_installed(tool: &Tool, bin_dir: &Path) -> bool {
    let tool_path = bin_dir.join(tool.name());
    
    if cfg!(target_os = "windows") {
        tool_path.with_extension("exe").exists()
    } else {
        tool_path.exists()
    }
}

// 下载工具
pub fn download_tool(tool: &Tool, bin_dir: &Path) -> Result<(), String> {
    println!("开始下载工具: {}", tool.name());
    info!("开始下载工具: {}", tool.name());
    
    // 确保bin目录存在
    if !bin_dir.exists() {
        fs::create_dir_all(bin_dir).map_err(|e| {
            let err_msg = format!("无法创建bin目录: {}", e);
            error!("{}", err_msg);
            err_msg
        })?;
    }
    
    let download_url = tool.download_url()?;
    let tool_path = bin_dir.join(tool.name());
    
    println!("下载地址: {}", download_url);
    println!("保存路径: {:?}", tool_path);
    info!("从 {} 下载 {} 到 {:?}", download_url, tool.name(), tool_path);
    
    // 使用curl下载工具，添加进度显示
    let output = if cfg!(target_os = "windows") {
        println!("使用 PowerShell 下载...");
        Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "$ProgressPreference = 'Continue'; Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
                    download_url,
                    tool_path.to_string_lossy()
                )
            ])
            .output()
    } else {
        println!("使用 curl 下载...");
        Command::new("curl")
            .args([
                "-L",
                "-#", // 添加进度条
                &download_url,
                "-o",
                &tool_path.to_string_lossy()
            ])
            .output()
    };
    
    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let err_msg = format!("下载 {} 失败: {}", tool.name(), stderr);
                println!("错误: {}", err_msg);
                error!("{}", err_msg);
                return Err(err_msg);
            }
            
            // 在Unix系统上设置可执行权限
            if !cfg!(target_os = "windows") {
                println!("设置可执行权限...");
                let chmod_output = Command::new("chmod")
                    .args(["+x", &tool_path.to_string_lossy()])
                    .output();
                
                if let Err(e) = chmod_output {
                    let err_msg = format!("无法设置 {} 的可执行权限: {}", tool.name(), e);
                    println!("错误: {}", err_msg);
                    error!("{}", err_msg);
                    return Err(err_msg);
                }
            }
            
            println!("{} 下载完成！", tool.name());
            info!("{} 下载完成", tool.name());
            Ok(())
        },
        Err(e) => {
            let err_msg = format!("下载 {} 时出错: {}", tool.name(), e);
            println!("错误: {}", err_msg);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

// 检查并安装所有工具
pub fn check_and_install_tools() -> Result<(), String> {
    info!("检查工具安装状态");
    
    // 获取.omni-ai目录
    let home_dir = dirs::home_dir().ok_or_else(|| "无法获取用户主目录".to_string())?;
    let omni_dir = home_dir.join(".omni-ai");
    let bin_dir = omni_dir.join("bin");
    
    // 确保目录存在
    if !omni_dir.exists() {
        info!("创建.omni-ai目录");
        fs::create_dir_all(&omni_dir).map_err(|e| {
            let err_msg = format!("无法创建.omni-ai目录: {}", e);
            error!("{}", err_msg);
            err_msg
        })?;
    }
    
    if !bin_dir.exists() {
        info!("创建bin目录");
        fs::create_dir_all(&bin_dir).map_err(|e| {
            let err_msg = format!("无法创建bin目录: {}", e);
            error!("{}", err_msg);
            err_msg
        })?;
    }
    
    // 检查并安装uv
    let uv = Tool::Uv;
    if !is_tool_installed(&uv, &bin_dir) {
        info!("uv未安装，开始下载");
        match download_tool(&uv, &bin_dir) {
            Ok(_) => info!("uv安装成功"),
            Err(e) => warn!("uv安装失败: {}", e),
        }
    } else {
        info!("uv已安装");
    }
    
    // 检查并安装bun
    let bun = Tool::Bun;
    if !is_tool_installed(&bun, &bin_dir) {
        info!("bun未安装，开始下载");
        match download_tool(&bun, &bin_dir) {
            Ok(_) => info!("bun安装成功"),
            Err(e) => warn!("bun安装失败: {}", e),
        }
    } else {
        info!("bun已安装");
    }
    
    Ok(())
}