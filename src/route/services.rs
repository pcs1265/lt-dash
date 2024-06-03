use std::fs::File;

use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(get_service_list))
        .route("/pinned", get(get_pinned_list))
}

const SERVICES_INFO_PATH: &str = "services.json";

async fn get_service_list() -> Json<Services> {
    let services_raw = File::open(SERVICES_INFO_PATH).unwrap();

    let services: Services = serde_json::from_reader(services_raw).unwrap();

    return Json(services);
}

async fn get_pinned_list() -> Json<Vec<Service>> {
    let services_raw = File::open(SERVICES_INFO_PATH).unwrap();

    let services: Services = serde_json::from_reader(services_raw).unwrap();

    return Json(services.pinned);
}

#[derive(Serialize, Deserialize, Debug)]
struct Services {
    #[serde(default)]
    pinned: Vec<Service>,

    #[serde(default)]
    normal: Vec<Service>,

    #[serde(default)]
    etc: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
    title: String,
    subtitle: String,
    link: String,
    icon: String,
}
