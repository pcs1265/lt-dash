use std::{thread::sleep, time::Duration};

use axum::{
    extract::{ws::WebSocket, Request, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::Local;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use sysinfo::{RefreshKind, System};
use tracing::info;

pub fn get_router() -> Router {
    Router::new().route("/", get(monitoring_connect_handler))
}

async fn monitoring_connect_handler(ws: WebSocketUpgrade, req: Request) -> impl IntoResponse {
    info!("{:?}", req);
    ws.on_upgrade(|socket| monitoring_websocket(socket))
}

async fn monitoring_websocket(stream: WebSocket) {
    let (mut sender, mut receiver): (
        futures::prelude::stream::SplitSink<WebSocket, axum::extract::ws::Message>,
        futures::prelude::stream::SplitStream<WebSocket>,
    ) = stream.split();

    let mut system = System::new_all();

    let disks = sysinfo::Disks::new_with_refreshed_list();

    while let Some(Ok(msg)) = receiver.next().await {
        system.refresh_cpu_usage();
        system.refresh_cpu_frequency();
        system.refresh_memory();

        if sender
            .send(format!("{}", json!(system)).into())
            .await
            .is_err()
        {
            break;
        }
    }
}
