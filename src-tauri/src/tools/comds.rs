use serde_json::Error;
use std::env;
use std::fs;
use std::io;

// load man.json
pub fn load_man(base_dir: &str) -> Result<String, io::Error> {
    let man_path = format!("{}/config/man.json", base_dir);
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
pub fn get_root_dir() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

#[tauri::command]
pub fn get_man() -> String {
    let man = load_man(&get_root_dir()).unwrap();
    man
}
