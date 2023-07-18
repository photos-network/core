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

use crate::api::handler::list_media_items_handler::list_media_items_handler;
use crate::api::handler::create_media_item_handler::create_media_item_handler;
use crate::api::handler::file_handler::file_handler;
use axum::routing::{get, patch, post, delete};
use axum::Router;

pub struct MediaApi {}

impl MediaApi {
    pub fn routes<S>() -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            // Returns a list of owned and shared photos for current user
            // 200 Ok
            // 401 Unauthorized - Requesting user is unauthenticated
            // 403 Forbidden
            // 500 Internal Server Error
            .route("/media", get(list_media_items_handler))

            // Creates one or multiple items
            // 201 - Created
            // 400 Bad Request - The request body was malformed or a field violated its constraints. 
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 500 Internal Server Error
            .route("/media", post(create_media_item_handler))

            // get metadata of a specific item
            // 200 - Ok
            // 400 Bad Request - The request body was malformed or a field violated its constraints. 
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 500 Internal Server Error
            .route("/media/:media_id", get(file_handler))

            // update the given item owned by the user
            .route("/media/:media_id", patch(file_handler))

            // delete the given item owned by the user
            .route("/media/:media_id", delete(file_handler))

            // list owned and shared albums
            .route("/albums", get(file_handler))
            // create new album
            .route("/albums", post(file_handler))
            // get metadata of a specific owned or shared album
            .route("/albums/:entity_id", get(file_handler))
            // updates the given album owned by the user
            .route("/albums/:entity_id", patch(file_handler))
            // shares the given album
            .route("/albums/:entity_id/share", patch(file_handler))
            // unshares the given album
            .route("/albums/:entity_id/unshare", patch(file_handler))
            
            .layer(tower_http::trace::TraceLayer::new_for_http())
    }
}
