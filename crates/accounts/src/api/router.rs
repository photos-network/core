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

use std::sync::Arc;

use axum::routing::{delete, get, patch, post};
use axum::Router;
use common::ApplicationState;

use super::routes::account::{handle_account_login, handle_account_register};
use super::routes::admin;
use super::routes::album_access;
use super::routes::download;
use super::routes::customer::{
    get_customer_album_media, get_customer_albums, get_customer_media_file,
    handle_customer_login, handle_customer_register,
};
use super::routes::get_user_id_profile::get_user_id_profile;
use super::routes::stats;

pub struct AccountsApi {}

impl AccountsApi {
    pub fn routes<S>(state: &ApplicationState) -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        let db = Arc::clone(&state.database);

        Router::new()
            // Customer authentication (access-code based)
            .route("/auth/customer/login", post(handle_customer_login))
            .route("/auth/customer/register", post(handle_customer_register))
            .route("/auth/customer/albums", get(get_customer_albums))
            .route("/auth/customer/albums/:album_id/media", get(get_customer_album_media))
            .route("/auth/customer/media/:media_id/file", get(get_customer_media_file))
            // Account authentication (email + password based)
            .route("/auth/account/register", post(handle_account_register))
            .route("/auth/account/login", post(handle_account_login))
            // User profile management
            // Returns information about a single account by ID
            // 200 OK
            // 401 Unauthorized - Requesting user is unauthenticated
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/profile", get(get_user_id_profile))
            // Update a single account when `admin.users:write` scope is present
            // 200 - OK
            // 400 Bad Request - The request body was malformed or a field violated its constraints.
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/profile", patch(get_user_id_profile))
            // Disable a single account by ID when `admin.users:write` scope is present
            // 204 No Content - Account was disabled successful
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/disable", patch(get_user_id_profile))
            // Enable a single account by ID when `admin.users:write` scope is present
            // 204 No Content - Account was enabled successful
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 404 Not Found - The requested resource does not exist.
            .route("/users/:user_id/enable", patch(get_user_id_profile))
            // Admin routes
            .route("/admin/users", get(admin::list_users).post(admin::create_user))
            .route("/admin/users/detailed", get(admin::list_users_detailed))
            .route("/admin/albums", get(admin::list_albums))
            .route("/admin/customers", get(admin::list_customers).post(admin::create_customer_code))
            // Album access management
            .route(
                "/albums/:album_id/access",
                post(album_access::grant_album_access).get(album_access::list_album_access),
            )
            .route(
                "/albums/:album_id/access/:account_id",
                delete(album_access::revoke_album_access),
            )
            // Album access-code management
            .route(
                "/albums/:album_id/codes",
                get(album_access::list_album_codes).post(album_access::add_album_code),
            )
            .route(
                "/albums/:album_id/codes/:access_code",
                delete(album_access::remove_album_code),
            )
            .route(
                "/albums/:album_id/codes/generate",
                post(album_access::generate_album_code),
            )
            .route(
                "/albums/:album_id/download",
                get(download::download_album_zip),
            )
            .route("/albums/:album_id/stats", get(stats::get_album_stats))
            .route("/albums/stats", get(stats::get_owned_album_stats))
            .with_state(db)
            .layer(tower_http::trace::TraceLayer::new_for_http())
    }
}
