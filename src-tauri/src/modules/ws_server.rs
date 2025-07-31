use crate::logger;

use tokio::sync::Mutex;
use std::{collections::HashMap, sync::{Arc}, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use futures::{stream::SplitSink, StreamExt, SinkExt};

const MODULE: &str = "WebSocket";

type Sender = SplitSink<WebSocketStream<TcpStream>, Message>;
type Clients = Arc<Mutex<HashMap<String, Sender>>>;
type OnMessage = Arc<dyn Fn(&WsServer, String, String) + Send + Sync>;

#[derive(Clone)]
pub struct WsServer {
    port: u32,
    clients: Clients,
    message_handler: OnMessage,
    ws_task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

#[allow(dead_code)]
impl WsServer {
    pub fn new(port: &u32) -> Self {
        Self {
            port: *port,
            clients: Arc::new(Mutex::new(HashMap::new())),
            message_handler: Arc::new(|_, _, _| {}),
            ws_task: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        logger::debug(&format!("正在启动WebSocket服务, 使用端口: {}...", self.port), MODULE);

        let mut ws_task = self.ws_task.lock().await;

        let self_clone = self.clone();

        *ws_task = Some(tokio::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", self_clone.port)).await.expect("绑定本地端口失败");

            while let Ok((stream, addr)) = listener.accept().await {
                self_clone.handle_connection(stream, addr).await;
            }
        }));

        logger::success(&format!("websocket服务启动成功, 使用端口: {}", self.port), MODULE);
        Ok(())
    }

    pub async fn stop(&mut self) {  
        logger::debug(&format!("正在停止WebSocket服务, 使用端口: {}...", self.port), MODULE);

        let mut clients = self.clients.lock().await;

        for (uuid, sender) in clients.iter_mut() {
            // let _ = sender.send(Message::Close(None)).await;
            match sender.close().await {
                Ok(_) => logger::info(&format!("关闭客户端 {} 连接成功", uuid), MODULE),
                Err(e) => logger::warning(&format!("关闭客户端: {} 连接失败, 失败原因: {}", uuid, e), MODULE),
            }
        }

        clients.clear();

        let handle = {
            let mut ws_task = self.ws_task.lock().await;
            ws_task.take()
        };

        // 关闭端口
        if let Some(handle) = handle {
            handle.abort();
        }

        logger::success(&format!("服务停止, 端口: {}", self.port), MODULE);
    }

    pub fn on_message<F>(mut self, handler: F) -> Self
    where
        F: Fn(&Self, String, String) + Send + Sync + 'static
    {
        self.message_handler = Arc::new(handler);
        self
    }

    async fn handle_connection(&self, stream: TcpStream, addr: SocketAddr) {
        let clients = self.clients.clone();

        let ws_stream = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                logger::error(&format!("握手失败, 失败原因: {}", e), MODULE);
                return;
            }
        };

        let (sender, mut receiver) = ws_stream.split();

        let uuid = uuid::Uuid::new_v4().to_string();

        clients.lock().await.insert(uuid.clone(), sender);

        logger::success(&format!("客户端 {} 连接成功, 分配ID: {}", addr, uuid), MODULE);

        while let Some(message) = receiver.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    logger::info(&format!("收到客户端: {} 消息: {}", uuid, text), MODULE);
                    (self.message_handler)(self, uuid.clone(), text);
                },
                Ok(Message::Close(_)) => {
                    clients.lock().await.remove(&uuid);
                    logger::warning(&format!("客户端 {} 断开连接", uuid), MODULE);
                    break;
                },
                Err(e) => {
                    logger::error(&format!("处理消息失败, 客户端: {}, 失败原因: {:?}", uuid, e), MODULE);
                },
                _ => {
                    logger::warning(&format!("未定义的消息类型"), MODULE);
                },
            }
        }
    }

    pub async fn send_to_client(&self, uuid: &str, message: &str) {
        if let Some(s) = self.clients.lock().await.get_mut(uuid) {
            let _ = s.send(Message::Text(message.to_string())).await;
            logger::success(&format!("给客户端: {} 发送消息: {} 成功", uuid, message), MODULE);
        }
    }

    pub async fn broadcast(&self, message: &str) {
        let mut clients = self.clients.lock().await;

        let uuids: Vec<_> = clients.keys().cloned().collect();

        for uuid in uuids {
            if let Some(sender) = clients.get_mut(&uuid) {
                if let Err(e) = sender.send(Message::Text(message.to_string())).await {
                    logger::error(&format!("给客户端: {} 发送消息失败, 失败原因: {}", uuid, e), MODULE);
                    clients.remove(&uuid);
                } else {
                    logger::success(&format!("给客户端: {} 发送消息: {} 成功", uuid, message), MODULE);
                }
            }
        }
    }
}
