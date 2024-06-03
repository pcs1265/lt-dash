use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{get_server_address_config, get_server_cors_config, get_server_port_config};

mod config;
mod route;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!(r#"LT-DASH started"#);

    let mut app = route::get_router();

    if get_server_cors_config() {
        info!("CORS Policy : permissive");
        app = app.layer(CorsLayer::permissive()); //Develop purpose !!! DELETE THIS ON!!!
    }

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
