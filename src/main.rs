mod config;
mod handlers;
mod logging;
mod models;
mod promql;

use axum::{Router, middleware, routing::get};
use config::ProxyConfig;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    client: reqwest::Client,
    config: ProxyConfig,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = ProxyConfig::load().expect("Failed to load configuration");
    let client = reqwest::Client::new();
    let listen_address = cfg.server.listen_address.clone();

    // Оборачиваем в Arc для совместного использования между потоками
    let shared_state = Arc::new(AppState {
        client,
        config: cfg,
    });

    let app = Router::new()
        .route("/api/v1/query", get(handlers::query))
        .with_state(shared_state)
        .layer(middleware::from_fn(logging::print_request_response));

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
