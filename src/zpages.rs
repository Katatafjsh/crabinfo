use std::sync::Arc;
use tokio::sync::RwLock;
use serde::Serialize;

use axum::{
    Json, Router, Extension,
    http::StatusCode, 
    routing::{get, post},
    response::{Response, IntoResponse}
};

pub fn routes() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/readyz/enable", post(enable_readyz))
        .route("/readyz/disable", post(disable_readyz))
        .layer(Extension(SharedReadyzState::default()))
}

async fn healthz() -> Response {
    (StatusCode::OK, Json(Status { status: "OK".to_owned() })).into_response()
}

async fn readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> Response {
    if readyz_state.read().await.enabled {
        (StatusCode::OK, Json(Status { status: "OK".to_owned() })).into_response()
    }
    else {
        StatusCode::SERVICE_UNAVAILABLE.into_response()
    }
}

async fn enable_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    readyz_state.write().await.enabled = true;
    StatusCode::ACCEPTED
}

async fn disable_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    readyz_state.write().await.enabled = false;
    StatusCode::ACCEPTED
}

#[derive(Serialize)]
struct Status {
    status: String,
}

struct ReadyzState {
    enabled: bool,
}

type SharedReadyzState = Arc<RwLock<ReadyzState>>;

impl Default for ReadyzState {
    fn default() -> Self {
        Self { enabled: true }
    }
}