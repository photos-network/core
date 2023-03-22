//! The PluginManager will setup & initialize configured and available plugins.
//!
//!

use std::path::{Path, PathBuf};

use abi_stable::{
    library::{lib_header_from_path, LibrarySuffix, RawLibrary},
};
use anyhow::Result;
use core_extensions::SelfOps;
use photos_network_plugin::PluginFactory_Ref;
use tracing::{debug, info, warn};

use crate::config::Configuration;

pub struct PluginManager {
    config: Configuration,
    path: String
}

impl PluginManager {
    pub async fn new(config: Configuration, path: String) -> Result<Self> {
        Ok(Self { config, path })
    }

    pub async fn init(self) -> Result<()> {
        info!("Found {} plugin(s) in the configuration.", self.config.plugins.len());

        for plugin in self.config.plugins {
            info!("Addon '{}' found in the configuration", plugin.domain);

            let base_name = plugin.domain.clone();
            // let debug_dir: PathBuf = "./target/debug/".as_ref_::<Path>().into_::<PathBuf>();
            let plugin_dir: PathBuf = self.path.clone().into_::<PathBuf>();
            let plugin_path: PathBuf = RawLibrary::path_in_directory(&plugin_dir, &base_name, LibrarySuffix::NoSuffix);

            if plugin_path.exists() {
                debug!("Addon '{}' also found in the `plugins` directory", plugin.domain);

                debug!("Try to load plugin...");
                let header = lib_header_from_path(&plugin_path)?;
                let res = header.init_root_module::<PluginFactory_Ref>();

                match res {
                    Ok(_) => {
                        info!("Loading {}({}) plugin was successful.", plugin.domain, header.version_strings().version);
                    }
                    Err(e) => {
                        warn!("Loading {} plugin was failed. [{}]", plugin.domain, e);
                        continue;
                    }
                };
            }
        }

        Ok(())
    }
}
