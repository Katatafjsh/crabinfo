use crabinfo::status::routes;
use tokio::signal;
use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
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
    .route("/panic", get(panic));

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
    info!("Panic recieved, exiting...");
    std::process::exit(255);
}