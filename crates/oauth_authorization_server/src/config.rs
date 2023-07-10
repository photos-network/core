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

use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub domain: String,
    pub use_ssl: bool,
    pub realm_keys_base_path: PathBuf,
    pub realms: Vec<ConfigRealm>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listen_addr: String::from("127.0.0.1:7777"),
            domain: String::from("localhost:7777"),
            use_ssl: false,
            realm_keys_base_path: Path::new("keys").to_path_buf(),
            realms: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRealm {
    pub name: String,
    pub domain: Option<String>,
    pub clients: Vec<Client>,
}
