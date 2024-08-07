/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! This defines the app configuration
use std::{fmt, fs};

use serde::{Deserialize, Serialize};
use tracing::info;

use super::{
    client::OAuthClientConfig,
    database_config::{DatabaseConfig, DatabaseDriver},
    plugin::Plugin,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub internal_url: String,
    pub external_url: String,
    pub database: Option<DatabaseConfig>,
    // pub auth_provider: Vec<AuthProvider>,
    pub clients: Vec<OAuthClientConfig>,
    pub plugins: Vec<Plugin>,
}

impl Configuration {
    pub fn new(path: &str) -> Option<Self> {
        info!("Load configuration file {}", path);

        let read_file_result = fs::read_to_string(path);

        let config = match read_file_result {
            Ok(data) => serde_json::from_str(&data)
                .expect("Configuration file could not be parsed as JSON!"),
            Err(_) => {
                let default_config = Configuration::empty();

                fs::write(path, serde_json::to_string_pretty(&default_config).unwrap())
                    .expect("Could not write default Configuration to file!");

                default_config
            }
        };

        Some(config)
    }

    pub fn empty() -> Self {
        Configuration {
            internal_url: "127.0.0.1".into(),
            external_url: "127.0.0.1".into(),
            database: Some(DatabaseConfig {
                driver: DatabaseDriver::SQLite,
                url: "sqlite://data/core.sqlite3".into(),
            }),
            clients: vec![],
            plugins: vec![],
        }
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let clients = &self.clients;
        let plugins = &self.plugins;

        write!(f, "{{")?;
        write!(f, "\n\tinternal: {}", self.internal_url)?;
        write!(f, "\n\texternal: {}", self.external_url)?;

        // clients
        write!(f, "\n\tclients: [ ")?;
        for (count, v) in clients.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "\n\t\t{}", v)?;
        }
        write!(f, "\n\t] ")?;

        // plugins
        write!(f, "\n\tplugins: [ ")?;
        for (count, v) in plugins.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "\n\t\t{}", v)?;
        }
        write!(f, "\n\t]")?;
        write!(f, "\n}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Map;

    #[test]
    fn test_minimal_deserialization() {
        // given
        let json = r#"{
            "internal_url": "192.168.0.1",
            "external_url": "demo.photos.network",
            "clients": [],
            "plugins": []
        }"#;

        let data = Configuration {
            internal_url: "192.168.0.1".into(),
            external_url: "demo.photos.network".into(),
            database: None,
            clients: vec![],
            plugins: vec![],
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn test_full_deserialization() {
        // given
        let json = r#"{
            "internal_url": "192.168.0.1",
            "external_url": "demo.photos.network",
            "clients": [
                {
                    "name": "Client",
                    "client_id": "clientId",
                    "client_secret": "clientSecret",
                    "redirect_uris": []
                }
            ],
            "plugins": [
                {
                    "name": "Plugin",
                    "config": {
                        "property1": null,
                        "property2": true,
                        "property3": "aBc",
                        "property4": 42
                    }
                }
            ]
        }"#;

        let mut config = Map::new();
        config.insert("property1".to_string(), serde_json::Value::Null);
        config.insert("property2".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "property3".to_string(),
            serde_json::Value::String("aBc".into()),
        );
        config.insert(
            "property4".to_string(),
            serde_json::Value::Number(42.into()),
        );

        let data = Configuration {
            internal_url: "192.168.0.1".into(),
            external_url: "demo.photos.network".into(),
            database: None,
            clients: vec![OAuthClientConfig {
                name: "Client".into(),
                client_id: "clientId".into(),
                client_secret: "clientSecret".into(),
                redirect_uris: vec![],
            }],
            plugins: vec![Plugin {
                name: "Plugin".into(),
                config: Some(config),
            }],
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }
}
