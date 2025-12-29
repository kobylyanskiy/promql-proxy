use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct QueryResult {
    pub status: String,
    pub data: Vec<String>,
}

#[derive(Deserialize)]
pub struct PromQuery {
    pub query: String,
}
