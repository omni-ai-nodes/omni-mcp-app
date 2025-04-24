use std::process::Command;
use std::io::{self, Write};
use log::{info, error};

#[tauri::command]
pub fn execute_command(cmd: &str, args: Vec<String>) -> Result<String, String> {
    info!("执行命令: {} {}", cmd, args.join(" "));
    
    // 确保 .omni-ai 目录存在
    let home_dir = dirs::home_dir().ok_or_else(|| "无法获取用户主目录".to_string())?;
    let omni_dir = home_dir.join(".omni-ai");
    
    // 如果目录不存在，则创建它
    if !omni_dir.exists() {
        info!("创建工作目录: {:?}", omni_dir);
        std::fs::create_dir_all(&omni_dir)
            .map_err(|e| {
                let err_msg = format!("无法创建 .omni-ai 目录: {}", e);
                error!("{}", err_msg);
                err_msg
            })?;
    }
    
    info!("在 {:?} 中执行命令", omni_dir);
    
    // 构建完整命令字符串用于日志记录
    let full_cmd = format!("{} {}", cmd, args.join(" "));
    println!("正在执行命令: {}", full_cmd);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .args(&args)
            .current_dir(&omni_dir)
            .output()
    } else {
        Command::new("sh")
            .args(["-c", &format!("{} {}", cmd, args.join(" "))])
            .current_dir(&omni_dir)
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            // 检查命令是否成功执行
            let status = output.status;
            info!("命令执行状态: {}", status);
            
            // 即使stdout为空也打印信息
            if stdout.is_empty() {
                println!("命令 '{}' 没有产生标准输出", full_cmd);
            } else {
                println!("命令 '{}' 输出:\n{}", full_cmd, stdout);
            }
            
            // 检查是否有错误输出
            if !stderr.is_empty() {
                println!("命令 '{}' 错误输出:\n{}", full_cmd, stderr);
                error!("命令执行错误: {}", stderr);
                
                // 如果标准输出为空但有错误输出，返回错误输出
                if stdout.is_empty() {
                    return Err(stderr);
                }
                
                // 如果两者都有，返回组合输出
                let combined = format!("标准输出:\n{}\n\n错误输出:\n{}", stdout, stderr);
                return Ok(combined);
            }
            
            // 如果标准输出为空且没有错误，返回一个提示信息
            if stdout.is_empty() {
                return Ok("命令执行成功，但没有产生输出。".to_string());
            }
            
            // 确保刷新输出
            io::stdout().flush().unwrap();
            
            Ok(stdout)
        }
        Err(e) => {
            let err_msg = format!("命令执行失败: {}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}