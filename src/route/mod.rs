use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};

use self::index::index_handler;

mod index;
mod services;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .fallback(not_found)
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
