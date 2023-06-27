
use std::fs;

use serde::Deserialize;
use serde_json::Map;
use tracing::{debug, info};

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub internal_url: String,
    pub external_url: String,
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub config: Option<Map<String, serde_json::Value>>
}

impl Configuration {
    pub fn new(path: &str) -> Option<Self> {
        info!("Load configuration file {}", path);
        let data = fs::read_to_string(path).expect("Unable to read configuration file!");
        let config: Configuration = serde_json::from_str(&data).expect("Configuration file could not be parsed as JSON!");

        debug!("internal: {}", config.internal_url);
        debug!("external: {}", config.external_url);
        debug!("plugins: {:?}", config.plugins);
        
        Some(config)
    }
}
