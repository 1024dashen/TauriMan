use super::model::ServerState;
use std::env;
use std::fs;
use std::io;
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use warp::Filter;
use machine_uid;


// load man.json
pub fn load_man(base_dir: &str) -> Result<String, io::Error> {
    let mut man_path = PathBuf::from(base_dir);
    man_path.push("config");
    man_path.push("man.json");
    match fs::read_to_string(&man_path) {
        Ok(man_json) => Ok(man_json),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // 文件不存在时返回空字符串
            Ok(String::new())
        }
        Err(e) => {
            // 其他类型的错误仍然返回
            Err(e)
        }
    }
}

#[tauri::command]
pub fn find_port() -> Result<u16, String> {
    // 绑定到端口 0，让系统自动分配可用端口
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    Ok(port)
}

// get machine uid
#[tauri::command]
pub fn get_machine_uid() -> String {
    let uid: String = machine_uid::get().unwrap();
    println!("{}", uid);
    uid
}

// server config www dir
#[tauri::command]
pub fn get_www_dir(base_dir: &str) -> Result<String, io::Error> {
    let mut www_dir = PathBuf::from(base_dir);
    www_dir.push("config");
    www_dir.push("www");
    // 判断文件夹是否存在并是否为空
    if fs::metadata(&www_dir).is_ok() {
        let files = fs::read_dir(&www_dir)?;
        if files.count() > 0 {
            let port = find_port().unwrap();
            let route = warp::fs::dir(www_dir);
            tokio::spawn(async move {
                warp::serve(route).run(([127, 0, 0, 1], port)).await;
            });
            return Ok(format!("http://127.0.0.1:{}", port));
        } else {
            return Ok(String::new());
        }
    }
    Ok(String::new())
}

#[tauri::command]
pub async fn start_server(
    state: tauri::State<'_, Arc<Mutex<ServerState>>>,
    path: String,
) -> Result<u16, String> {
    println!("start_server: {}", path);
    let mut state = state.lock().unwrap();
    if state.server_handle.is_some() {
        return Err("Server is already running".into());
    }
    let path_clone = path.clone();
    let port = find_port().unwrap();
    println!("port: {}", port);
    let server_handle = tokio::spawn(async move {
        let route = warp::fs::dir(path_clone)
            .map(|reply| {
                warp::reply::with_header(
                    reply,
                    "Cache-Control",
                    "no-store, no-cache, must-revalidate, max-age=0",
                )
            })
            .map(|reply| warp::reply::with_header(reply, "Vary", "*"))
            .map(|reply| warp::reply::with_header(reply, "Surrogate-Control", "no-store"))
            .map(|reply| warp::reply::with_header(reply, "Pragma", "no-cache"))
            .map(|reply| warp::reply::with_header(reply, "Expires", "0"));
        warp::serve(route).run(([127, 0, 0, 1], port)).await;
    });
    state.server_handle = Some(server_handle);
    Ok(port)
}

#[tauri::command]
pub async fn stop_server(state: tauri::State<'_, Arc<Mutex<ServerState>>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if let Some(handle) = state.server_handle.take() {
        handle.abort();
        Ok(())
    } else {
        Err("Server is not running".into())
    }
}

#[tauri::command]
pub fn get_exe_dir() -> String {
    let exe_path = env::current_exe().unwrap(); // 获取当前可执行文件路径
    let exe_dir = exe_path.parent().unwrap();
    exe_dir.to_str().unwrap().to_string()
}

#[tauri::command]
pub fn get_man() -> String {
    let man = load_man(&get_exe_dir()).unwrap();
    man
}
