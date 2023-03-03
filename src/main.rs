use axum::{
    routing::get,
    Router,
    Json,
};

use serde::Serialize;
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() {
    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = Router::new()
    .route("/healthz", get(liveness_probe));

    info!("Starting service...");
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn liveness_probe() -> Json<Status> {
    Json(Status { status: "OK".to_owned() })
}

#[derive(Serialize)]
struct Status {
    status: String,
}