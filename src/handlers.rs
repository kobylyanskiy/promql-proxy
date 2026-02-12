use crate::{AppState, models::{PromQuery, PromQueryRange}, promql::parse_promql};
use axum::{
    Json,
    extract::{Query, State, Form},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

pub async fn test(
    State(state): State<Arc<AppState>>,
    query: Query<PromQuery>,
) -> impl IntoResponse {
    let prom_query_struct = query.0;
    let (env, modified_query) = parse_promql(
        state.config.routing.target_label.clone(),
        prom_query_struct.query.as_str(),
    );
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

async fn query_impl(
    state: Arc<AppState>,
    prom_query_struct: PromQuery,
) -> impl IntoResponse {
    let (env, modified_query) = parse_promql(
        state.config.routing.target_label.clone(),
        prom_query_struct.query.as_str(),
    );

    let target_url = state
        .config
        // TODO tenants/prometheuses
        .tenants
        .get(&env)
        .map(|url| url.as_str())
        .unwrap_or(state.config.routing.fallback_url.as_str());

    tracing::info!("query: {}", &prom_query_struct.query);
    tracing::info!("Routing query to: {}", target_url);

    // Build query params: modified query + all extra params
    let mut query_params: Vec<(&str, &str)> = vec![("query", modified_query.as_str())];
    for (key, value) in prom_query_struct.extra.iter() {
        tracing::debug!("Extra param: {} = {}", key, value);
        query_params.push((key.as_str(), value.as_str()));
    }
    tracing::debug!("All query params: {:?}", query_params);

    let response = state
        .client
        .get(format!("{}/api/v1/query", target_url))
        .query(&query_params)
        .timeout(Duration::from_secs(state.config.server.timeout_seconds))
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            let body_bytes = match res.bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    tracing::error!("Failed to read response body: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read upstream response",
                    )
                        .into_response();
                }
            };

            // Try to parse as JSON first
            match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(json) => (status, Json(json)).into_response(),
                Err(e) => {
                    // If JSON parsing fails, return the raw text
                    let text = String::from_utf8_lossy(&body_bytes);
                    tracing::error!(
                        "Failed to parse JSON from upstream (status {}): {}. Body: {}",
                        status,
                        e,
                        text
                    );
                    (
                        status,
                        format!("Upstream error: {}", text),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Proxy error: {}", e);
            (StatusCode::BAD_GATEWAY, "Target server unreachable").into_response()
        }
    }
}

pub async fn query(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PromQuery>,
) -> impl IntoResponse {
    query_impl(state, params).await
}

pub async fn query_post(
    State(state): State<Arc<AppState>>,
    Form(params): Form<PromQuery>,
) -> impl IntoResponse {
    query_impl(state, params).await
}

async fn query_range_impl(
    state: Arc<AppState>,
    prom_query_struct: PromQueryRange,
) -> impl IntoResponse {
    let (env, modified_query) = parse_promql(
        state.config.routing.target_label.clone(),
        prom_query_struct.query.as_str(),
    );

    let target_url = state
        .config
        .tenants
        .get(&env)
        .map(|url| url.as_str())
        .unwrap_or(state.config.routing.fallback_url.as_str());

    tracing::info!("query_range: {}", &prom_query_struct.query);
    tracing::info!("Routing query_range to: {}", target_url);

    let mut query_params = vec![
        ("query", modified_query.as_str()),
        ("start", prom_query_struct.start.as_str()),
        ("end", prom_query_struct.end.as_str()),
    ];

    if let Some(ref step) = prom_query_struct.step {
        query_params.push(("step", step.as_str()));
    }

    // Add all extra params
    for (key, value) in prom_query_struct.extra.iter() {
        tracing::debug!("Extra param: {} = {}", key, value);
        query_params.push((key.as_str(), value.as_str()));
    }
    tracing::debug!("All query params: {:?}", query_params);

    let response = state
        .client
        .get(format!("{}/api/v1/query_range", target_url))
        .query(&query_params)
        .timeout(Duration::from_secs(state.config.server.timeout_seconds))
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            let body_bytes = match res.bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    tracing::error!("Failed to read response body: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read upstream response",
                    )
                        .into_response();
                }
            };

            // Try to parse as JSON first
            match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(json) => (status, Json(json)).into_response(),
                Err(e) => {
                    // If JSON parsing fails, return the raw text
                    let text = String::from_utf8_lossy(&body_bytes);
                    tracing::error!(
                        "Failed to parse JSON from upstream (status {}): {}. Body: {}",
                        status,
                        e,
                        text
                    );
                    (
                        status,
                        format!("Upstream error: {}", text),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Proxy error: {}", e);
            (StatusCode::BAD_GATEWAY, "Target server unreachable").into_response()
        }
    }
}

pub async fn query_range(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PromQueryRange>,
) -> impl IntoResponse {
    query_range_impl(state, params).await
}

pub async fn query_range_post(
    State(state): State<Arc<AppState>>,
    Form(params): Form<PromQueryRange>,
) -> impl IntoResponse {
    query_range_impl(state, params).await
}
