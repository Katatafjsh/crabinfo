use serde::Serialize;

use axum::{
    Json, Router,
    extract::Path,
    routing::get,
    response::{Response, IntoResponse},
};

use tokio::time::{sleep, Duration};

pub fn routes() -> Router {
    Router::new()
        .route("/deplay/:seconds", get(delay))
}

async fn delay(Path(seconds): Path<u16>) -> Response {
    sleep(Duration::from_secs(seconds.into())).await;
    Json(Delay { delay: seconds }).into_response()
}

#[derive(Serialize)]
struct Delay {
    delay: u16
}