use axum::{
    http::{StatusCode, Uri},
    Router,
};
use tokio::net::TcpListener;
use tracing::info;

use crate::config::{get_server_address_config, get_server_port_config};

mod config;
mod route;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!(r#"DASHBD started"#);

    let app = route::get_router();

    let listener = TcpListener::bind(format!(
        "{}:{}",
        get_server_address_config(),
        get_server_port_config()
    ))
    .await
    .unwrap();

    info!("Server starting - {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
