use serde::Serialize;

use axum::{
    Json, Router,
    extract::Path,
    routing::get,
    http::StatusCode,
    response::{Response, IntoResponse},
};

pub fn routes() -> Router {
    Router::new()
        .route("/status/:status_code", get(status_code))
}

async fn status_code(Path(status_code): Path<u16>) -> Response {
    // TODO: add control flag
    match StatusCode::from_u16(status_code) {
        Ok(s) => (s, Json(Status { status: s.to_string() })).into_response(),
        _ => StatusCode::BAD_REQUEST.into_response(),
    }
 }

 #[derive(Serialize)]
 struct Status {
     status: String,
 }