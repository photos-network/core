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

pub struct AccountsApi {}

impl AccountsApi {
    pub fn routes<S>() -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            // Returns information about a single account by ID
            // 200 OK
            // 401 Unauthorized - Requesting user is unauthenticated
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/profile", get(list_photos_handler))

            // Update a single account when `admin.users:write` scope is present
            // 200 - OK
            // 400 Bad Request - The request body was malformed or a field violated its constraints. 
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/profile", patch(list_photos_handler))
            
            // Disable a single account by ID when `admin.users:write` scope is present
            // 204 No Content - Account was disabled successful
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/disable", patch(list_photos_handler))

            // Enable a single account by ID when `admin.users:write` scope is present
            // 204 No Content - Account was enabled successful
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/enabled", patch(list_photos_handler))

            .layer(tower_http::trace::TraceLayer::new_for_http())
    }
}
