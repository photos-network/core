//! The PluginManager will setup & initialize configured and available plugins.
//!
//!

use std::path::{PathBuf};

use abi_stable::{
    library::{lib_header_from_path, LibrarySuffix, RawLibrary},
};
use abi_stable::external_types::crossbeam_channel;
use abi_stable::std_types::{RErr, ROk};
use anyhow::Result;
use core_extensions::SelfOps;
use photos_network_plugin::{PluginFactory_Ref, PluginId, Plugin_TO};
use tracing::{debug, info, warn};
use crate::config::{Configuration};

pub struct PluginManager {
    config: Configuration,
    path: String,
}

impl PluginManager {
    pub async fn new(config: Configuration, path: String) -> Result<Self> {
        Ok(Self { config, path })
    }

    pub async fn init(self) -> Result<()> {
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

                match res {
                    Ok(plugin_factory) => {
                        info!("Loading {}({}) plugin was successful.", configured_plugin.domain, header.version_strings().version);

                        let plugin_constructor = plugin_factory.new();

                        // TODO: learn crossbeam
                        let (sender, _receiver) = crossbeam_channel::unbounded();
                        let plugin = match plugin_constructor(sender.clone(), PluginId::from("test")) {
                            ROk(x) => x,
                            RErr(e) => {
                                // TODO: handle errors
                                // plugin_new_errs.push((plugin_id.clone(), e));
                                continue;
                            }
                        };

                        // TODO: persist plugin in app state?
                        // plugins.push(plugin);
                    }
                    Err(e) => {
                        warn!("Loading {} plugin was failed. [{}]", configured_plugin.domain, e);
                        continue;
                    }
                };
            }
        }

        Ok(())
    }
}
