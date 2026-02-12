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
    let cfg = ProxyConfig::load().expect("Failed to load configuration");
    let crate_name = env!("CARGO_CRATE_NAME");
    let log_level = &cfg.server.log_level;

    let filter_str = format!("{},tower_http={}", crate_name, log_level);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(filter_str))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = reqwest::Client::new();
    let listen_address = cfg.server.listen_address.clone();

    let shared_state = Arc::new(AppState {
        client,
        config: cfg,
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/v1/query", get(handlers::query).post(handlers::query_post))
        .route("/api/v1/query_range", get(handlers::query_range).post(handlers::query_range_post))
        .route("/api/v1/test", get(handlers::test))
        .with_state(shared_state)
        .layer(middleware::from_fn(logging::print_request_response));

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}

async fn health_handler() -> &'static str {
    "OK"
}
