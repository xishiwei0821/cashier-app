// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod logger;
mod modules;

use std::sync::Mutex;
use tauri::State;

struct AppData {
    ws_server: Mutex<Option<WsServer>>,
    serial_server: Mutex<Option<SerialServer>>,
}

use modules::{ws_server::WsServer, serial_server::SerialServer};

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

    match ws_server.start().await {
        Ok(_) => {
            *state.ws_server.lock().unwrap() = Some(ws_server);
            Ok(true)
        },
        Err(e) => Err(e),
    }
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

#[tauri::command]
async fn start_serial_server(port_name: &str, baud_rate: u32, state: State<'_, AppData>) -> Result<bool, String> {
    if let Some(_serial_server) = state.serial_server.lock().unwrap().as_mut() {
        return Err("串口服务已经在运行中".to_string());
    }

    let serial_server = SerialServer::new(port_name, &baud_rate);

    match serial_server.start().await {
        Ok(_) => {
            *state.serial_server.lock().unwrap() = Some(serial_server);
            Ok(true)
        },
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn stop_serial_server(state: State<'_, AppData>) -> Result<bool, String> {
    let mut serial_server = state.serial_server.lock().expect("获取串口服务失败").take();

    match serial_server.as_mut() {
        Some(serial_server) => {
            let _ = serial_server.stop().await;
            *state.serial_server.lock().unwrap() = None;
            Ok(true)
        }
        None => {
            Err("串口服务未启动".to_string())
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
            serial_server: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![greet, start_ws_server, stop_ws_server, start_serial_server, stop_serial_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
