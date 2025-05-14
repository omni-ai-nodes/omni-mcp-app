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
        if cfg!(target_os = "windows") {
            match self {
                Tool::Git => {
                    // Git 使用 where 命令检查
                    Command::new("where")
                        .arg(self.name())
                        .output()
                        .map_or(false, |output| output.status.success())
                },
                Tool::Bun => {
                    // 检查 Bun 的安装目录
                    let home = std::env::var("USERPROFILE").unwrap_or_default();
                    let bun_path = format!("{}/.bun/bin/bun.exe", home);
                    std::path::Path::new(&bun_path).exists()
                },
                Tool::Uv => {
                    // 检查 UV 的安装目录
                    let home = std::env::var("USERPROFILE").unwrap_or_default();
                    let uv_path = format!("{}/.local/bin/uv.exe", home);
                    std::path::Path::new(&uv_path).exists()
                },
            }
        } else {
            // Unix 系统保持原样
            Command::new("which")
                .arg(self.name())
                .output()
                .map_or(false, |output| output.status.success())
        }
    }

    // 获取 UV 最新版本号
    fn get_uv_latest_version() -> Result<String, String> {
        println!("正在获取 UV 最新版本号...");
        let mut command = Command::new("curl");
        command.args([
            "-s",
            "-L", // 添加跟随重定向
            "--connect-timeout", "10", // 添加超时设置
            "https://api.github.com/repos/astral-sh/uv/releases/latest"
        ]);

          // 如果存在系统代理，添加代理支持
        if let Ok(proxy) = std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("https_proxy")) {
            command.args(["--proxy", &proxy]);
        }

        let output = command
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
                    match Self::get_uv_latest_version() {
                        Ok(version) => {
                            println!("成功获取 UV 最新版本: {}", version);
                            Ok((
                                "powershell".to_string(),
                                vec![
                                    "-ExecutionPolicy".to_string(),
                                    "Bypass".to_string(),
                                    "-c".to_string(),
                                    format!(
                                        "$ErrorActionPreference = 'Stop';
                                        Write-Host '正在下载 UV 安装脚本...';
                                        $progressPreference = 'silentlyContinue';
                                        curl.exe -L -s -o uv-installer.ps1 https://github.com/astral-sh/uv/releases/download/{}/uv-installer.ps1;
                                        if ($LastExitCode -eq 0) {{
                                            Write-Host '开始安装 UV...';
                                            $env:UV_INSTALL_QUIET = 1;
                                            .\\uv-installer.ps1;
                                            if ($LastExitCode -eq 0) {{
                                                Write-Host '正在更新环境变量...';
                                                $env:Path = [System.Environment]::GetEnvironmentVariable('Path', 'User') + ';' + $env:USERPROFILE + '\\.local\\bin';
                                                [System.Environment]::SetEnvironmentVariable('Path', $env:Path, 'User');
                                                Write-Host 'UV 安装成功！';
                                                Remove-Item uv-installer.ps1;
                                            }} else {{
                                                throw 'UV 安装失败'
                                            }}
                                        }} else {{
                                            throw '下载安装脚本失败'
                                        }}",
                                        version
                                    ),
                                ]
                            ))
                        },
                        Err(e) => Err(format!("获取 UV 版本失败: {}", e))
                    }
                },
                Tool::Bun => Ok((
                    "powershell".to_string(),
                    vec![
                        "-ExecutionPolicy".to_string(),
                        "Bypass".to_string(),
                        "-c".to_string(),
                        "$ErrorActionPreference = 'Stop';
                        Write-Host '正在下载 Bun 安装脚本...';
                        $progressPreference = 'silentlyContinue';
                        curl.exe -L -s -o bun-installer.ps1 https://bun.sh/install.ps1;
                        if ($LastExitCode -eq 0) {
                            Write-Host '开始安装 Bun...';
                            $BunRoot = \"$env:USERPROFILE\\.bun\";
                            $env:BUN_INSTALL = $BunRoot;
                            .\\bun-installer.ps1;
                            if ($LastExitCode -eq 0) {
                                Write-Host '正在更新环境变量...';
                                $BunBinPath = \"$BunRoot\\bin\";
                                $CurrentPath = [System.Environment]::GetEnvironmentVariable('Path', 'User');
                                if (-not ($CurrentPath -like \"*$BunBinPath*\")) {
                                    $NewPath = \"$CurrentPath;$BunBinPath\";
                                    [System.Environment]::SetEnvironmentVariable('Path', $NewPath, 'User');
                                }
                                Write-Host '清理安装文件...';
                                Remove-Item bun-installer.ps1;
                                Write-Host 'Bun 安装成功！'
                            } else {
                                throw 'Bun 安装失败'
                            }
                        } else {
                            throw '下载安装脚本失败'
                        }".to_string(),
                    ]
                )),
                Tool::Git => Ok((
                    "powershell".to_string(),
                    vec![
                        "-ExecutionPolicy".to_string(),
                        "Bypass".to_string(),
                        "-c".to_string(),
                        "$ErrorActionPreference = 'Stop';
                        Write-Host '正在下载 Git 安装程序...';
                        $progressPreference = 'silentlyContinue';
                        curl.exe -L -s -o git-installer.exe https://github.com/git-for-windows/git/releases/download/v2.43.0.windows.1/Git-2.43.0-64-bit.exe;
                        if ($LastExitCode -eq 0) {
                            Write-Host '开始安装 Git...';
                            .\\git-installer.exe /VERYSILENT /NORESTART /COMPONENTS='icons,ext\\reg\\shellhere,assoc,assoc_sh';
                            if ($LastExitCode -eq 0) {
                                Write-Host '正在更新环境变量...';
                                $env:Path = [System.Environment]::GetEnvironmentVariable('Path', 'Machine');
                                [System.Environment]::SetEnvironmentVariable('Path', $env:Path, 'Machine');
                                Write-Host '清理安装文件...';
                                Remove-Item git-installer.exe;
                                Write-Host 'Git 安装成功！'
                            } else {
                                throw 'Git 安装失败'
                            }
                        } else {
                            throw '下载安装程序失败'
                        }".to_string(),
                    ]
                )),
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
            
            // 过滤掉进度条相关的输出
            let filtered_stderr = stderr.lines()
                .filter(|line| !line.contains("##") && !line.contains("%"))
                .collect::<Vec<&str>>()
                .join("\n");
            
            if !filtered_stderr.is_empty() {
                println!("安装错误输出:\n{}", filtered_stderr);
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
    info!("开始安装单个工具: {}", tool);
    
    let tool = match tool {
        "uv" => Tool::Uv,
        "bun" => Tool::Bun,
        "git" => Tool::Git,
        _ => return Err("不支持的工具类型".to_string()),
    };
    
    install_tool(&tool)
}
