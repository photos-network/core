//! This is the main entry point of the Photos.network core application.
//!
//!
//!
use std::{fs, process};

use anyhow::{Context, Result};
use tracing::info;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
};

use config::Configuration;
use plugin::PluginManager;

pub mod config;
pub mod plugin;

const CONFIG_PATH: &str = "./config/configuration.json";
const PLUGIN_PATH: &str = "./plugins";

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("error: {:#}", e);
        process::exit(1);
    }
}

pub async fn run() -> Result<()> {
    // enable logging
    let file_appender = tracing_appender::rolling::daily("./logs", "core");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            // subscriber configuration
            .with_max_level(tracing::Level::DEBUG)
            .with_target(false)
            .finish()
            // add additional writers
            .with(fmt::Layer::default().with_ansi(false).with_writer(file_writer))
    ).expect("Unable to set global tracing subscriber");

    info!("Photos.network core is starting...");

    fs::create_dir_all("data")?;
    fs::create_dir_all("config")?;
    fs::create_dir_all("plugins")?;

    // read config file
    let config = Configuration::new(CONFIG_PATH).expect("Could not parse configuration!");

    PluginManager::new(config, PLUGIN_PATH.to_string()).await?.init().await.context("PluginManager: initialization failed!")

    // TODO: start routing
}
