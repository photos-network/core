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

use std::path::PathBuf;

use abi_stable::library::{lib_header_from_path, LibrarySuffix, RawLibrary};

use anyhow::Result;

use crate::{config::configuration::Configuration, ApplicationState};
use core_extensions::SelfOps;
use photos_network_plugin::{PluginFactory_Ref, PluginId};
use tracing::{debug, error, info};

pub struct PluginManager<'a> {
    config: Configuration,
    path: String,
    state: &'a mut ApplicationState,
}

impl<'a> PluginManager<'a> {
    pub fn new(
        config: Configuration,
        path: String,
        state: &'a mut ApplicationState,
    ) -> Result<Self> {
        Ok(Self {
            config,
            path,
            state,
        })
    }

    pub async fn init<'b>(&mut self) -> Result<()> {
        info!(
            "Found {} plugin(s) in the configuration.",
            self.config.plugins.len()
        );

        for configured_plugin in &self.config.plugins {
            info!(
                "Plugin '{}' found in the configuration file.",
                configured_plugin.name
            );

            let mut base_name = String::from("plugin_").to_owned();
            let plugin_name = configured_plugin.name.to_lowercase().to_owned();
            base_name.push_str(&plugin_name);
            let plugin_dir: PathBuf = self.path.clone().into_::<PathBuf>();

            let plugin_path: PathBuf =
                RawLibrary::path_in_directory(&plugin_dir, &base_name, LibrarySuffix::NoSuffix);

            if plugin_path.exists() {
                debug!(
                    "Plugin '{}' also found in the `plugins` directory",
                    plugin_name
                );

                debug!("Try to load plugin '{}'...", plugin_name);
                let header = lib_header_from_path(&plugin_path)?;
                let res = header.init_root_module::<PluginFactory_Ref>();

                let root_module = match res {
                    Ok(x) => x,
                    Err(e) => {
                        error!("Could not init plugin! {}", e);
                        continue;
                    }
                };

                let mut loaded_libraries = Vec::<PluginId>::new();
                loaded_libraries.push(PluginId::from(plugin_name.clone()));

                // TODO: insert loaded plugin instead?
                self.state
                    .plugins
                    .insert(PluginId::from(plugin_name), root_module);
            }
        }

        Ok(())
    }

    pub async fn trigger_on_init(&mut self) -> () {
        // self.state.router.as_mut().unwrap().route("/foo", get( || async { "It's working!" } ));
        // ERROR: move occurs because value has type `Router`, which does not implement the `Copy` trait
    }
}
