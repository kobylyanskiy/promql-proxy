// use serde::{Deserialize, Serialize};
use serde::Deserialize;

// #[derive(Serialize)]
// pub struct QueryResult {
//     pub status: String,
//     pub data: Vec<String>,
// }

#[derive(Deserialize)]
pub struct PromQuery {
    pub query: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct PromQueryRange {
    pub query: String,
    pub start: String,
    pub end: String,
    pub step: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, String>,
}
