use std::{thread::sleep, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Request, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::Local;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde_json::json;
use sysinfo::System;
use tracing::info;

pub fn get_router() -> Router {
    Router::new().route("/", get(websocket_handler))
}

async fn websocket_handler(ws: WebSocketUpgrade, req: Request) -> impl IntoResponse {
    info!("{:?}", req);
    ws.on_upgrade(|socket| websocket(socket))
}

async fn websocket(stream: WebSocket) {
    let (sender, receiver): (
        futures::prelude::stream::SplitSink<WebSocket, axum::extract::ws::Message>,
        futures::prelude::stream::SplitStream<WebSocket>,
    ) = stream.split();

    let mut send_task = tokio::spawn(ws_sender(sender));
    let mut recv_task = tokio::spawn(ws_receiver(receiver));

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };
}

async fn ws_sender(mut sender: SplitSink<WebSocket, Message>) {
    let mut system = System::new_all();
    loop {
        sleep(Duration::from_millis(100));

        system.refresh_cpu();
        let date = Local::now();
        let v = date.format("%Y-%m-%d %H:%M:%S.%f");

        if sender
            .send(format!("{}", json!(system)).into())
            .await
            .is_err()
        {
            break;
        }
    }
}

async fn ws_receiver(mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(msg)) = receiver.next().await {
        // let text = msg.into_text().unwrap();
        info!("websocket msg received : {msg:?}");
    }
}
