
use std::{sync::{Arc, Mutex}, collections::HashSet};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    }, response::IntoResponse
};
use tokio::sync::{broadcast};
use futures::{SinkExt, StreamExt};

#[derive(Debug)]
pub struct WsState {
    pub sender:  broadcast::Sender<String>
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<WsState>>,
) -> impl IntoResponse {
    // let (tx, _rx) = broadcast::channel(100);
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<WsState>) {


    let (mut sender, mut receiver) = stream.split();

    let _ = sender.send(Message::Text(String::from("已经链接到服务器"))).await;

    let mut _rx = state.sender.subscribe();

    println!("新用户链接: {:#?}",state.sender);

    let _ = state.sender.send(format!("测试channel"));

    let cn = state.sender.clone();

    let _recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let msg = format!("{}", text);
            println!("收到消息: {} ", msg);
            // let _ = sender.send(Message::Text(String::from(msg))).await;
            let _ = cn.send(msg);

            
            // let _ = tx.send(Message::Text(msg));
        }
    });

    //这里需要获取所有的rx，用于广播发送信息。
    let _send_task = tokio::spawn(async move {
        while let Ok(msg) = _rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}