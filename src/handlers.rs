use crate::models::{PromQuery, QueryResult};
use axum::{Json, extract::Query};
use promql_parser::parser;
use promql_parser::parser::Expr;

pub async fn query(query: Query<PromQuery>) -> Json<QueryResult> {
    let promql: PromQuery = query.0;

    match parser::parse(promql.query.as_str()) {
        Ok(expr) => {
            extract_labels(&expr);
        }
        Err(info) => println!("Err: {info:?}"),
    }

    let query_result = QueryResult {
        status: "ok".to_string(),
        data: vec!["development".to_string(), "staging".to_string()],
    };

    Json(query_result)
}

fn extract_labels(expr: &Expr) {
    tracing::info!("Prettify: {}", expr.prettify());
    tracing::info!("AST: {expr:?}");
}
