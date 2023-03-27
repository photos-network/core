//! This is the main entry point of the Photos.network core application.
//!
//!
//!
use std::fs;
use std::collections::HashMap;
use std::net::SocketAddr;

use abi_stable::external_types::crossbeam_channel;
use abi_stable::std_types::RResult::{ROk, RErr};
use anyhow::{Context, Result};
use axum::{Json, Router};
use axum::routing::{get, head};
use photos_network_plugin::{PluginFactory_Ref, PluginId};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{info, error};
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
    let mut app_state = ApplicationState::new(config.clone());

    let plugin_manager = PluginManager::new(
        config.clone(), 
        PLUGIN_PATH.to_string(),
        &mut app_state
    );

    plugin_manager?.init().await.context("PluginManager: initialization failed!");
    //plugin_manager?.trigger_on_init(); // ERROR: use of moved value: `plugin_manager` value used here after move

    
    // trigger `on_core_init` on all loaded plugins
    for (plugin_id, factory) in app_state.plugins {
        info!("plugin {} found in AppState.", plugin_id);
        
        let plugin_constructor = factory.new();

        let (sender, receiver) = crossbeam_channel::unbounded();

        let plugin = match plugin_constructor(sender.clone(), plugin_id.clone()) {
            ROk(x) => x,
            RErr(e) => {
                // TODO: handle error
                error!("Not able to trigger plugin constructor for {}", plugin_id);
                //plugin_new_errs.push((plugin_id.clone(), e));
                continue;
            }
        };

        plugin.on_core_init();
    }


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
