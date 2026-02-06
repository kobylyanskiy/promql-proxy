use crate::{AppState, models::PromQuery, promql::parse_promql};
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;

pub async fn test(
    State(state): State<Arc<AppState>>,
    query: Query<PromQuery>,
) -> impl IntoResponse {
    let prom_query_struct = query.0;
    let (env, modified_query) = parse_promql(prom_query_struct.query.as_str());
    tracing::info!("parsed env: {:#?}", env);

    let target_url = state
        .config
        .tenants
        .get(&env)
        .map(|url| url.as_str())
        .unwrap_or(state.config.routing.fallback_url.as_str());

    Json(json!({
        "env": env,
        "target_url": target_url,
        "query": modified_query
    }))
}

pub async fn query(
    State(state): State<Arc<AppState>>,
    query: Query<PromQuery>,
) -> impl IntoResponse {
    let prom_query_struct = query.0;
    let (env, modified_query) = parse_promql(prom_query_struct.query.as_str());

    let target_url = state
        .config
        .tenants
        .get(&env)
        .map(|url| url.as_str())
        .unwrap_or(state.config.routing.fallback_url.as_str());

    tracing::info!("query: {}", &prom_query_struct.query);
    tracing::info!("Routing query to: {}", target_url);

    let response = state
        .client
        .get(format!("{}/api/v1/query", target_url))
        .query(&[("query", &modified_query)])
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
