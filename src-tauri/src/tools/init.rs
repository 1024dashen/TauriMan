use super::model::Man;
use crate::tools::comds::{get_exe_dir, get_www_dir, load_man};
use serde_json::{json, Error};
use tauri::{utils::config::WindowConfig, App, AppHandle, Manager, Url, WebviewUrl, WindowEvent};
use tauri_plugin_store::StoreExt;

pub fn show_window(app: &AppHandle) {
    let main = app.get_webview_window("main");
    if let Some(main) = main {
        main.unminimize().expect("Sorry, can't unminimize window");
        main.set_focus().expect("Sorry, can't focus window");
    } else {
        app.webview_windows()
            .values()
            .next()
            .expect("Sorry, no window found")
            .set_focus()
            .expect("Can't Bring Window to Focus");
    }
}

// handle something when start app
pub async fn resolve_setup(app: &mut App) -> Result<(), Error> {
    // get startup dir
    let startup_dir = get_exe_dir();
    println!("startup_dir: {}", startup_dir);

    let app_handle = app.handle();

    let window_json = r#"
        {
            "title": "TauriMan",
            "url": "http://localhost:5173/"
        }
    "#;
    let mut config: WindowConfig = serde_json::from_str(window_json).unwrap();

    let man = load_man(&startup_dir);
    let man_content = man.unwrap();
    if man_content.len() > 0 {
        let mut man_config: Man = serde_json::from_str(&man_content).unwrap();
        println!("man: {:?}", man_config);
        let www_dir = get_www_dir(&startup_dir);
        let www_dir_str = www_dir.unwrap();
        if www_dir_str.len() > 0 {
            println!("www_dir: {}", www_dir_str);
            man_config.window.url = WebviewUrl::External(Url::parse(&www_dir_str).unwrap());
        }
        config = man_config.window;
    }

    let window: tauri::WebviewWindow =
        tauri::WebviewWindowBuilder::from_config(app_handle, &config)
            .unwrap()
            .build()
            .unwrap();

    let store = app.store("app_data.json").unwrap();

    let window_fullscreen: Option<serde_json::Value> = store.get("window_fullscreen");
    // println!("windows_fullscreen: {:?}", window_fullscreen);

    let window_size: Option<serde_json::Value> = store.get("window_size");
    // println!("windows_size: {:?}", window_size);
    let mut width = 960.0;
    let mut height = 720.0;
    if let Some(window_size) = window_size {
        let size = window_size.as_object().unwrap();
        width = size["width"].as_f64().unwrap();
        height = size["height"].as_f64().unwrap();
        // println!("width: {:?}", width);
        // println!("height: {:?}", height);
    }

    let window_position: Option<serde_json::Value> = store.get("window_position");
    let mut x = 0.0;
    let mut y = 0.0;
    // println!("windows_position: {:?}", window_position);
    if let Some(window_position) = window_position {
        let position = window_position.as_object().unwrap();
        x = position["x"].as_f64().unwrap();
        y = position["y"].as_f64().unwrap();
        // println!("x: {:?}", x);
        // println!("y: {:?}", y);
    }

    if let Some(window_fullscreen) = window_fullscreen {
        let fullscreen = window_fullscreen.as_object().unwrap();
        // println!("fullscreen: {:?}", fullscreen);
        if config.fullscreen || fullscreen["fullscreen"].as_bool().unwrap() {
            window.set_fullscreen(true).unwrap();
            // println!("window fullscreen");
        } else {
            window
                .set_size(tauri::PhysicalSize::new(width, height))
                .unwrap();
            if config.center || (x == 0.0 && y == 0.0) {
                window.center().unwrap();
            } else {
                window
                    .set_position(tauri::PhysicalPosition::new(x, y))
                    .unwrap();
            }
        }
    }

    let window_clone = window.clone();

    window.on_window_event(move |event| {
        if let WindowEvent::Resized(size) = event {
            // println!("window_size: {:?}", size);
            if size.width > 0 && size.height > 0 {
                let _ = store.set(
                    "window_size",
                    json!({
                        "width": size.width,
                        "height": size.height
                    }),
                );
            }
            if window_clone.is_fullscreen().unwrap_or(false) {
                // println!("Window entered fullscreen mode.");
                let _ = store.set(
                    "window_fullscreen",
                    json!({
                        "fullscreen": true
                    }),
                );
            } else {
                let _ = store.set(
                    "window_fullscreen",
                    json!({
                        "fullscreen": false
                    }),
                );
            }
        }
        if let WindowEvent::Moved(position) = event {
            // println!("window_position: {:?}", position);
            if position.x > 0 && position.y > 0 {
                let _ = store.set(
                    "window_position",
                    json!({ "x": position.x, "y": position.y }),
                );
            }
        }
    });

    window.show().unwrap();

    window.set_focus().unwrap();

    Ok(())
}
