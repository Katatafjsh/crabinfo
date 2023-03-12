use tokio::signal;
use std::net::SocketAddr;
use std::env;

use axum::{
    Json, Router, 
    http::HeaderMap,
    routing::get, 
    response::{Response, IntoResponse},
};

use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() {
    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    let app = Router::new()
    .merge(crabinfo::zpages::routes())
    .merge(crabinfo::status::routes())
    .merge(crabinfo::jwt::routes())
    .merge(crabinfo::delay::routes())
    .route("/panic", get(panic))
    .route("/env", get(environment))
    .route("/headers", get(headers));

    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Signal received, starting graceful shutdown...");
}

async fn panic() {
    // TODO: add control flag 
    info!("Panic recieved, exiting...");
    std::process::exit(255);
}

async fn environment() -> Response {
    // TODO: filter out sensitive info
    let mut items: Vec<String> = Vec::new();

    for (k,v) in env::vars() {
        items.push(format!("{}={}", k,v));
    }
    
    Json(items).into_response()
}

async fn headers(_headers: HeaderMap) {
    todo!()
}