use serde::{Deserialize, Serialize};
use tauri::utils::config::WindowConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Man {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub window: WindowConfig,
}

pub struct ServerState {
    pub server_handle: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub modified: u64, // 时间戳
    pub is_dir: bool,
}
