/// 打开 GitHub 仓库链接
/// 
/// 使用系统默认浏览器打开指定的 GitHub 仓库链接
#[tauri::command]
pub async fn open_github_link() -> Result<(), String> {
    const GITHUB_URL: &str = "https://github.com/omni-ai-nodes/omni-mcp-app.git";
    
    match open::that(GITHUB_URL) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}