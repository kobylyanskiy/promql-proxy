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
    pub upstream_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MimirConfig {
    pub endpoint: String,
    pub tenant_header: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProxyConfig {
    pub server: ServerConfig,
    pub mimir: MimirConfig,
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
