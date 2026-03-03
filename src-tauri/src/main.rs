#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod mcp;

use std::collections::HashMap;
use std::fs;

#[tauri::command]
async fn send_message(prompt: String) -> Result<String, String> {
    api::chat(prompt).await
}

#[tauri::command]
fn get_mcp_servers() -> Result<HashMap<String, mcp::McpServer>, String> {
    mcp::get_servers()
}

#[tauri::command]
fn save_mcp_servers(servers: HashMap<String, mcp::McpServer>) -> Result<(), String> {
    mcp::save_servers(servers)
}

#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
    // Cap at 100KB to avoid flooding the context
    if meta.len() > 100_000 {
        return Err(format!(
            "File too large ({} KB). Max 100 KB for context.",
            meta.len() / 1024
        ));
    }
    fs::read_to_string(&path).map_err(|e| format!("Cannot read file: {}", e))
}

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_message,
            get_mcp_servers,
            save_mcp_servers,
            read_file_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
