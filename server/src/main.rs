use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::fmt::format::FmtSpan;

mod config;
mod device;
mod manager;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        .init();

    // Create the router
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/devices", get(routes::list::list))
        .route("/devices/:port/logs", get(routes::logs::logs));

    // Serve the server
    let listener = tokio::net::TcpListener::bind(config::CONFIG.bind_address.clone())
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    tracing::info!("Server listening on {}", config::CONFIG.bind_address);
}
