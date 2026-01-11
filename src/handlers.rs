use crate::{AppState, models::PromQuery, promql::parse_promql};
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn query(
    State(state): State<Arc<AppState>>,
    query: Query<PromQuery>,
) -> impl IntoResponse {
    let prom_query_struct = query.0;
    let labels = parse_promql(prom_query_struct.query.as_str());

    let target_url = match labels.get(0).map(|s| s.as_str()) {
        Some("dev") => "http://localhost:9091",
        Some("production") => "http://localhost:9092",
        _ => &state.config.server.upstream_url, // fallback
    };

    tracing::info!("query: {}", &prom_query_struct.query);
    tracing::info!("Routing query to: {}", target_url);

    let response = state
        .client
        .get(format!("{}/api/v1/query", target_url))
        .query(&[("query", &prom_query_struct.query)])
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            match res.json::<serde_json::Value>().await {
                Ok(json) => (status, Json(json)).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid JSON from upstream",
                )
                    .into_response(),
            }
        }
        Err(e) => {
            tracing::error!("Proxy error: {}", e);
            (StatusCode::BAD_GATEWAY, "Target server unreachable").into_response()
        }
    }
}
