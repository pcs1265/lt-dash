use self::index::index_handler;
use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

mod index;
mod services;
mod static_asset;
mod websocket;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .nest_service("/websocket", websocket::get_router())
        .nest_service("/static", axum_static::static_router("static"))
        .nest_service("/services", services::get_router())
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
