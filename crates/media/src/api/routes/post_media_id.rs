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

//! Add files for a specific media item
//!

use axum::extract::{Multipart, Path};
use axum::http::StatusCode;
use common::auth::user::User;
use tracing::{debug, error};

pub(crate) async fn post_media_id(
    Path(media_id): Path<String>,
    user: User,
    mut multipart: Multipart,
) -> std::result::Result<String, StatusCode> {
    error!("POST /media/{}  user={}", media_id, user);
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "name" => {
                    debug!("name={}", field.text().await.unwrap());
                }
                "file" => {
                    // TODO: wrap bytes and write to persistency
                    debug!("filesize={}", field.chunk().await.unwrap().unwrap().len());
                }
                _ => continue,
            }
        }
    }

    // TODO: write file to storage
    // TODO: add file reference in database

    Err(StatusCode::OK)
}
