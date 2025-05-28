mod tools;
use std::sync::{Arc, Mutex};
use tools::model::ServerState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(ServerState {
            server_handle: None,
        })))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            tools::comds::get_exe_dir,
            tools::comds::get_man,
            tools::comds::get_machine_uid,
            tools::comds::find_port,
            tools::comds::open_folder,
            tools::comds::read_dir,
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let _ = tools::init::resolve_setup(app).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
