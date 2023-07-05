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

use std::sync::{Arc, RwLock};

use crate::client::Client;
use crate::config::ServerConfig;
use crate::error::Error;
use crate::realm::Realm;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub addr: String,
    pub realms: Vec<Realm>,
    pub master_realm: Realm,
}

type SharedState = Arc<RwLock<ServerState>>;

impl ServerState {
    pub fn new(config: ServerConfig) -> Result<Self, Error> {
        let realms = config
            .realms
            .iter()
            .filter_map(|r| {
                Realm::new(
                    &r.name,
                    &r.domain.clone().unwrap_or(config.domain.clone()),
                    helper_get_scheme_from_config(config.use_ssl),
                    r.clients.clone(),
                    config.realm_keys_base_path.clone(),
                )
                .ok()
            })
            .collect::<Vec<Realm>>();
        Ok(Self {
            addr: config.listen_addr,
            realms,
            master_realm: Realm::new(
                "master",
                &config.domain,
                helper_get_scheme_from_config(config.use_ssl),
                vec![Client {
                    id: String::from("master_client"),
                    secret: None,
                    redirect_uri: String::from("/callback"),
                }],
                config.realm_keys_base_path.clone(),
            )?,
        })
    }
}

fn helper_get_scheme_from_config(use_ssl: bool) -> &'static str {
    if use_ssl {
        "https"
    } else {
        "http"
    }
}
