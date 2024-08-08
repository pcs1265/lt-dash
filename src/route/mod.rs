use axum::{
    http::{StatusCode, Uri},
    Router,
};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod monitoring;
mod websocket;

pub fn get_router() -> Router {
    Router::new()
        .nest_service(
            "/",
            ServeDir::new("app").not_found_service(ServeFile::new("assets/index.html")),
        )
        .nest_service("/websocket", websocket::get_router())
        .nest_service("/monitoring", monitoring::get_router())
        .fallback(not_found)
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
