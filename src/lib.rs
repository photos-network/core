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

//! Photos.network · A privacy first photo storage and sharing service.
//!
//! The core application is responsible for main tasks like:
//!  * Authentication (validate the identity of users)
//!  * Authorization (handle access privileges of resources like photos or albums)
//!  * Plugins (manage and trigger plugins)
//!  * Persistency (read / write data)
//!  * Task Processing (keep track of running tasks)
//!
//! See also the following crates
//!  * [Authentication](../oauth_authentication/index.html)

use std::fs::{self, OpenOptions};
use std::net::SocketAddr;

use abi_stable::external_types::crossbeam_channel;
use abi_stable::std_types::RResult::{RErr, ROk};
use accounts::api::router::AccountsApi;
use anyhow::Result;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, head};
use axum::{Json, Router};
use common::auth::user::User;
use common::database::Database;
use common::ApplicationState;
use database::sqlite::SqliteDatabase;
use media::api::router::MediaApi;
use oauth_authentication::AuthenticationManager;
use oauth_authorization_server::client::Client;
use oauth_authorization_server::config::ConfigRealm;
use oauth_authorization_server::config::ServerConfig;
use oauth_authorization_server::state::ServerState;
use oauth_authorization_server::AuthorizationServerManager;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use std::path::Path;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt};

use common::config::configuration::Configuration;
use plugin::plugin_manager::PluginManager;

pub mod plugin;

const CONFIG_PATH: &str = "./config/core.json";
const DATA_PATH: &str = "./data";
const PLUGIN_PATH: &str = "./plugins";
const LOGGING_PATH: &str = "./logs";

/// starts the core applications server, reading the user configuration, connecting to databases and spinning up the REST API.
pub async fn start_server() -> Result<()> {
    // enable logging
    let file_appender = tracing_appender::rolling::daily(LOGGING_PATH, "core");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            // subscriber configuration
            .with_max_level(tracing::Level::TRACE)
            .with_target(false)
            .finish()
            // add additional writers
            .with(
                fmt::Layer::default()
                    .with_ansi(false)
                    .with_writer(file_writer),
            ),
    )
    .expect("Unable to set global tracing subscriber");

    info!("Photos.network core is starting...");

    // create mandatory application directories if necessary
    fs::create_dir_all("data")?;
    fs::create_dir_all("config")?;
    fs::create_dir_all("plugins")?;

    // read config file
    let configuration = Configuration::new(CONFIG_PATH).expect("Could not parse configuration!");
    debug!("Configuration: {}", configuration);

    // init database
    //let db = PostgresDatabase::new("postgres://postgres:unsecure@localhost:5432/postgres").await;

    let _file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("data/core.sqlite3");

    let mut db = SqliteDatabase::new("data/core.sqlite3").await;

    {
        let _ = db.setup().await;
    }

    let users = db.get_users().await;
    if users.unwrap().is_empty() {
        info!("No user found, create a default admin user. Please check `data/credentials.txt` for details.");
        let default_user = "photo@photos.network";
        let default_pass = "unsecure";
        let path = Path::new(DATA_PATH).join("credentials.txt");
        let _ = fs::write(path, format!("{}\n{}", default_user, default_pass));
        // let mut output = File::create(path)?;
        // let line = "hello";
        // write!(output, "{}\n{}", default_user, default_pass);

        let user = User {
            uuid: "".to_string(),
            email: default_user.to_string(),
            password: Some(default_pass.to_string()),
            lastname: Some("Admin".to_string()),
            firstname: Some("".to_string()),
            is_locked: false,
            created_at: OffsetDateTime::now_utc(),
            updated_at: None,
            last_login: None,
        };
        let _ = db.clone().create_user(&user).await;
    }

    // init application state
    //let mut app_state = ApplicationState::<PostgresDatabase>::new(configuration.clone(), db);
    let mut app_state = ApplicationState::<SqliteDatabase>::new(configuration.clone(), db);

    let cfg = ServerConfig {
        listen_addr: configuration.internal_url.to_owned(),
        domain: configuration.external_url.to_owned(),
        use_ssl: true,
        realm_keys_base_path: Path::new("config").to_path_buf(),
        realms: vec![ConfigRealm {
            name: String::from("master"),
            domain: Some(configuration.external_url.to_owned()),
            clients: vec![Client {
                id: String::from("mobile-app"),
                secret: None,
                redirect_uri: String::from("photosapp://authenticate"),
            }],
        }],
    };
    let server = ServerState::new(cfg)?;

    // TODO: check if `data/credentials.txt` still exists and stop immediately!
    let mut router = Router::new()
        // favicon
        .nest_service("/assets", ServeDir::new("src/api/static"))

        // health check
        .route("/", get(status))
        .route("/", head(status))

        // Media items
        .nest("/", MediaApi::routes(app_state.clone()).await)

        // OAuth 2.0 Authentication
        .nest("/", AuthenticationManager::routes())

        // OAuth Authorization Server
        .nest("/", AuthorizationServerManager::routes(server))

        // Account management
        .nest("/", AccountsApi::routes())
        .layer(TraceLayer::new_for_http())
        // grant all CORS OPTIONS requests
        .layer(CorsLayer::very_permissive())

        // allow to receive bodies larger than the default limit of 2MB
        .layer(DefaultBodyLimit::disable())

        // add database connection pool
        //.with_state(pool)


        // TODO: share app state with routes
        // .with_state(Arc::new(app_state))
        ;
    app_state.router = Some(router);

    // initialize plugin manager
    let mut plugin_manager = PluginManager::new(
        configuration.clone(),
        PLUGIN_PATH.to_string(),
        &mut app_state,
    )?;

    match plugin_manager.init().await {
        Ok(_) => info!("PluginManager: initialization succed."),
        Err(e) => error!("PluginManager: initialization failed! {}", e),
    }
    plugin_manager.trigger_on_init().await;

    // trigger `on_core_init` on all loaded plugins
    for (plugin_id, factory) in app_state.plugins {
        info!("Plugin '{}' found in AppState.", plugin_id);

        let plugin_constructor = factory.new();

        let (sender, _receiver) = crossbeam_channel::unbounded();

        let plugin = match plugin_constructor(sender.clone(), plugin_id.clone()) {
            ROk(x) => x,
            RErr(_) => {
                // TODO: handle error
                error!(
                    "Not able to trigger plugin constructor for '{}'!",
                    plugin_id
                );
                //plugin_new_errs.push((plugin_id.clone(), e));
                continue;
            }
        };

        plugin.on_core_init();
    }

    // TODO: add routes lazy (e.g. from plugin)
    router = app_state
        .router
        .unwrap()
        .route("/test", get(|| async { "" }));

    // task::spawn_blocking(move || {
    //     tracing::debug!("setup Authentication Manager...");
    //     let manager = AuthenticationManager::new();
    //     let nonce = AuthenticationManager::create_authorization_url(
    //         manager::client,
    //         manager::pkce_challenge,
    //     );
    // }).await?;

    // start server with all routes
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 7777));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn status() -> Json<Status> {
    // TODO: get app state

    // TODO: print loaded plugins from appState
    let status = Status {
        message: String::from("API running"),
    };
    Json(status)
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    message: String,
}
