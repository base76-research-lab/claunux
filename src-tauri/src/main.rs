#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod mcp;

use std::collections::HashMap;

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

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_message,
            get_mcp_servers,
            save_mcp_servers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
