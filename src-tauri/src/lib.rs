// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod logger;
mod modules;

use std::sync::Mutex;
use tauri::{AppHandle, State, Emitter};
use modules::{ws_server::WsServer, serial_server::SerialServer};

const MODULE: &str = "MAIN";

struct AppData {
    // app_handle: AppHandle,
    ws_server: Mutex<Option<WsServer>>,
    serial_server: Mutex<Option<SerialServer>>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            // app_handle: AppHandle::default(),
            ws_server: Mutex::new(None),
            serial_server: Mutex::new(None),
        }
    }
}

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
fn read_serial_port() -> Result<Vec<String>, String> {
    match tokio_serial::available_ports() {
        Ok(ports) => {
            let mut port_list = Vec::new();
            for port in ports {
                port_list.push(port.port_name);
            }
            Ok(port_list)
        },
        Err(e) => {
            let message = format!("未发现可用串口: {}", e);
            logger::error(&message, MODULE);
            Err(message)
        }
    }
}

// fn serial_on_data(message: String) {
//     println!("读取到重量: {}", message);
// }

#[tauri::command]
async fn start_serial_server(app_handle: AppHandle, port_name: &str, baud_rate: u32, state: State<'_, AppData>) -> Result<bool, String> {
    if let Some(_serial_server) = state.serial_server.lock().unwrap().as_mut() {
        return Err("串口服务已经在运行中".to_string());
    }

    let app_handle_clone = app_handle.clone();
    let serial_server = SerialServer::new(port_name, &baud_rate).on_data(move |data| {
        app_handle_clone.emit("serial_data", data).unwrap();
    });

    match serial_server.start().await {
        Ok(_) => {
            app_handle.emit("serial_data", "10.345").unwrap();
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
        .manage(AppData::default())
        // .setup(|app| {
        //     let app_handle = app.handle();
        //     let mut app_data = app.state::<AppData>();
        //     app_data.app_handle = app_handle.clone();
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![greet, start_ws_server, stop_ws_server, read_serial_port, start_serial_server, stop_serial_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
