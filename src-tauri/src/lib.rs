// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod logger;
mod modules;

use std::sync::Mutex;
use tauri::State;

struct AppData {
    ws_server: Mutex<Option<WsServer>>,
}

use modules::{ws_server::WsServer};

const MODULE: &str = "MAIN";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_ws_server(port: u32, state: State<'_, AppData>) -> Result<bool, String> {
    if let Some(_ws_server) = state.ws_server.lock().unwrap().as_mut() {
        return Err("WebSocket服务已经在运行中".to_string());
    }

    let mut ws_server = WsServer::new(&port);

    ws_server.start().await.expect("启动WebSocket服务失败");
    *state.ws_server.lock().unwrap() = Some(ws_server);

    Ok(true)
}

#[tauri::command]
async fn stop_ws_server(state: State<'_, AppData>) -> Result<bool, String> {
    let mut ws_server = state.ws_server.lock().expect("获取WebSocket服务失败").take();

    match ws_server.as_mut() {
        Some(ws_server) => {
            ws_server.stop().await;
            *state.ws_server.lock().unwrap() = None;
            Ok(true)
        }
        None => {
            Err("WebSocket服务未启动".to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::Logger::init(&None).unwrap();
    logger::info("Logger 初始化成功", MODULE);
    logger::Logger::set_level("DEBUG");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppData {
            ws_server: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![greet, start_ws_server, stop_ws_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
