//! The PluginManager will setup & initialize configured and available plugins.
//!
//!

use std::path::{PathBuf};

use abi_stable::{
    library::{lib_header_from_path, LibrarySuffix, RawLibrary},
};

use anyhow::Result;
use core_extensions::SelfOps;
use photos_network_plugin::{PluginFactory_Ref, PluginId};
use tracing::{debug, info, error};
use crate::{config::{Configuration}, ApplicationState};

pub struct PluginManager<'a> {
    config: Configuration,
    path: String,
    state: &'a mut ApplicationState,
}

impl<'a> PluginManager<'a> {
    pub fn new(config: Configuration, path: String, state: &'a mut ApplicationState) -> Result<Self> {
        Ok(Self { config, path, state })
    }

    pub async fn init<'b>(&mut self) -> Result<()> {        
        info!("Found {} plugin(s) in the configuration.", self.config.plugins.len());

        for configured_plugin in &self.config.plugins {
            info!("Addon '{}' found in the configuration", configured_plugin.domain);

            let base_name = configured_plugin.domain.clone();
            let plugin_dir: PathBuf = self.path.clone().into_::<PathBuf>();
            let plugin_path: PathBuf = RawLibrary::path_in_directory(&plugin_dir, &base_name, LibrarySuffix::NoSuffix);

            if plugin_path.exists() {
                debug!("Addon '{}' also found in the `plugins` directory", configured_plugin.domain);

                debug!("Try to load plugin...");
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
                loaded_libraries.push(PluginId::from(base_name.clone()));
                self.state.plugins.insert(PluginId::from(base_name), root_module);
            }
        }

        Ok(())
    }

    pub async fn trigger_on_init(&self) -> () {

    }
}
