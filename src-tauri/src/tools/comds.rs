use std::env;

#[tauri::command]
pub fn get_startup_dir() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}
