use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub listen_address: String,
    pub log_level: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoutingConfig {
    pub target_label: String,
    pub fallback_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProxyConfig {
    pub server: ServerConfig,
    pub routing: RoutingConfig,
    pub tenants: HashMap<String, String>,
}

impl ProxyConfig {
    pub fn load() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("settings.toml"))
            .merge(Env::prefixed("PROXY_").split("__"))
            .extract()
    }
}
