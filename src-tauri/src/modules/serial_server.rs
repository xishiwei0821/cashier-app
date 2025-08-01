use crate::logger;

use tokio_serial::SerialPortBuilderExt;
use tokio::{io::{AsyncReadExt}, sync::Mutex};
use std::sync::Arc;

const MODULE: &str = "Serial";

#[derive(Clone)]
pub struct SerialServer {
    port_name: String,
    baud_rate: u32,
    on_data: Option<Arc<dyn Fn(String) + Send + Sync>>,
    task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

#[allow(dead_code)]
impl SerialServer {
    pub fn new(port_name: &str, baud_rate: &u32) -> Self {
        Self {
            port_name: (*port_name).to_string(),
            baud_rate: *baud_rate,
            on_data: None,
            task: Arc::new(Mutex::new(None)),
        }
    }

    pub fn on_data<F: Fn(String) + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.on_data = Some(Arc::new(callback));
        self
    }

    pub async fn start(&self) -> Result<(), String> {

        logger::info(&format!("尝试连接串口: {}", self.port_name), MODULE);

        let mut task = self.task.lock().await;

        if task.is_some() {
            logger::warning(&format!("串口: {} 监听任务已存在", self.port_name), MODULE);
            return Err(format!("串口: {} 监听任务已存在", self.port_name));
        }

        match tokio_serial::new(self.port_name.clone(), self.baud_rate).open_native_async() {
            Ok(mut stream) => {
                let self_clone = self.clone();

                *task = Some(tokio::spawn(async move {
                    let mut buf = [0, 255];

                    while let Ok(n) = stream.read(&mut buf).await {
                        if let Some(cb) = &self_clone.on_data {
                            let data = String::from_utf8_lossy(&buf[..n]).to_string();
                            cb(data);
                        }
                    }
                }));

                logger::success(&format!("串口: {} 监听任务启动成功", self.port_name), MODULE);
                Ok(())
            },
            Err(e) => {
                logger::error(&format!("连接串口 {} 失败", e), MODULE);
                Err(format!("连接串口 {} 失败", e))
            }
        }
    }

    pub async fn stop(&self) -> Result<(), String> {
        let mut task = self.task.lock().await;

        if task.is_none() {
            logger::warning(&format!("串口: {} 未连接", self.port_name), MODULE);
            return Err(format!("串口: {} 未连接", self.port_name));
        }

        if let Some(task) = task.take() {
            logger::warning(&format!("取消串口: {} 监听任务", self.port_name), MODULE);
            task.abort();
        }

        Ok(())
    }

    // async fn listen(&self, port: &str) {
    //     loop {
    //         logger::info(&format!("尝试连接串口: {}", port), MODULE);
        
    //         match tokio_serial::new(port, self.baud_rate).open_native_async() {
    //             Ok(mut stream) => {
    //                 logger::success(&format!("连接到串口: {} 成功, 正在监听数据....", port), MODULE);

    //                 let mut buf = [0, 255];

    //                 while let Ok(n) = stream.read(&mut buf).await {
    //                     if let Some(cb) = &self.on_data {
    //                         let data = String::from_utf8_lossy(&buf[..n]).to_string();
    //                         cb(data);
    //                     }
    //                 }
    //             },
    //             Err(e) => {
    //                 logger::error(&format!("连接到串口: {} 失败: {}", port, e), MODULE);
    //             }
    //         }

    //         time::sleep(time::Duration::from_secs(10)).await;
    //     }
    // }
}
