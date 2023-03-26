//! This is the main entry point of the Photos.network core application.
//!
//!
//!
use std::fs;
use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::{Json, Router};
use axum::routing::{get, head};
use photos_network_plugin::{PluginFactory_Ref, PluginId};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
};

use config::Configuration;
use plugin::PluginManager;

pub mod config;
pub mod plugin;
pub mod api {
    pub mod authentication;
}


const CONFIG_PATH: &str = "./config/configuration.json";
const PLUGIN_PATH: &str = "./plugins";
const LOGGING_PATH: &str = "./logs";


pub async fn run() -> Result<()> {
    // enable logging
    let file_appender = tracing_appender::rolling::daily(LOGGING_PATH, "core");
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

    // init application state
    let mut _app_state = ApplicationState::new(config.clone());

    let _plugin_manager = PluginManager::new(config.clone(), PLUGIN_PATH.to_string()).await?.init().await.context("PluginManager: initialization failed!").unwrap();

    let router = Router::new()
        .nest_service("/assets", ServeDir::new("src/api/static"))
        .route("/", get(status))
        .route("/", head(status))
        .nest("/oauth", api::authentication::AuthManager::routes())
        .layer(TraceLayer::new_for_http())
        
        // TODO: share app state with routes
        // .with_state(Arc::new(app_state))
        ;
        
        
        // TODO: add routes lazy (e.g. from plugin)
        // app_state.router = Some(router);
        // router.route("/test", get( || async { "It's working!" } ));
        

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 7777));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}


pub struct ApplicationState {
    pub config: Configuration,
    pub plugins: HashMap<PluginId, PluginFactory_Ref>,
    pub router: Option<Router>,
}

impl ApplicationState {
    pub fn new(config: Configuration) -> Self {
        Self {
            config,
            plugins: HashMap::new(),
            router: None,
        }
    }
}


async fn status() -> Json<Status> {
    // TODO: get app state

    // TODO: print loaded plugins from appState
    let status = Status { message: String::from("API running") };
    Json(status)
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    message: String,
}
