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

use crate::api::handler::file_handler::file_handler;
use axum::routing::{get, patch, post};
use axum::Router;

pub struct MediaApi {}

impl MediaApi {
    pub fn routes<S>() -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            // list owned and shared items
            .route("/photos", get(file_handler))
            // get metadata of a specific item
            .route("/photos/:entity_id", get(file_handler))
            // update the given media item owned by the user
            .route("/photos/:entity_id", patch(file_handler))
            // creates one or multiple media items
            .route("/photos", post(file_handler))
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
