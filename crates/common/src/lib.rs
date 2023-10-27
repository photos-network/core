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

//! This crate offers shared data models for [Photos.network](https://photos.network) core application.
//!

use std::{collections::HashMap, sync::Arc};

use axum::Router;
use config::configuration::Configuration;
use database::ArcDynDatabase;
use photos_network_plugin::{PluginFactoryRef, PluginId};

pub mod auth;
pub mod config;
pub mod database;
pub mod http;
pub mod model {
    pub mod sensitive;
}

/// Aggregates the applications configuration, its loaded plugins and the router for all REST APIs
#[derive(Clone)]
pub struct ApplicationState {
    pub config: Arc<Configuration>,
    pub plugins: HashMap<PluginId, PluginFactoryRef>,
    pub router: Option<Router>,
    pub database: ArcDynDatabase,
}

impl ApplicationState {
    pub fn new(config: Arc<Configuration>, database: ArcDynDatabase) -> Self {
        Self {
            config,
            plugins: HashMap::new(),
            router: None,
            database,
        }
    }
}
