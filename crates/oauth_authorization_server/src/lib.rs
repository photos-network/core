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

//! This crate offers an **OAuth Authorization Server** for [Photos.network](https://photos.network) core application.
//! 
use axum::routing::get;
use axum::Router;
use handler::{
    authorize::authorization_handler,
    discovery::openid_discover_handler,
    jwks::openid_jwks_handler,
    login::{get_realm_login_form, post_realm_login},
};
use state::ServerState;
use std::sync::{Arc, RwLock};

pub mod client;
pub mod config;
pub mod error;
pub mod query;
pub mod realm;
pub mod request;
pub mod state;
pub mod handler {
    pub mod authorize;
    pub mod discovery;
    pub mod jwks;
    pub mod login;
}

pub struct AuthorizationServerManager {}

impl AuthorizationServerManager {
    pub fn routes<S>(server: ServerState) -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            .route(
                "/.well-known/openid-configuration",
                get(openid_discover_handler),
            )
            .route("/oidc/authorize", get(authorization_handler))
            .route("/jwk", get(openid_jwks_handler))
            .route(
                "/:realm/login",
                get(get_realm_login_form).post(post_realm_login),
            )
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .with_state(Arc::new(RwLock::new(server)))
    }
}
