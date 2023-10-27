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

use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use common::auth::user::User;
use tracing::{debug, info};
use uuid::Uuid;

use bytes::Bytes;

use crate::data::error::DataAccessError;
use crate::repository::MediaRepositoryState;

pub(crate) async fn post_media_id(
    State(repo): State<MediaRepositoryState>,
    Path(media_id): Path<String>,
    user: User,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    info!("POST /media/..");

    let mut name: String = "".to_string();
    let mut bytes = Bytes::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "name" => {
                    name = field.text().await.unwrap();
                    debug!("name={}", name.clone());
                }
                "file" => {
                    bytes = field.bytes().await.unwrap();
                    debug!("{} bytes received", bytes.clone().len());
                }
                _ => continue,
            }
        }
    }

    debug!("{} bytes received", bytes.clone().len());

    let result = repo
        .add_reference_for_media_item(
            Uuid::parse_str(user.uuid.as_str()).unwrap(),
            &media_id,
            name,
            bytes, // tempfile,
        )
        .await;

    match result {
        Ok(uuid) => {
            debug!("reference added. uuid={}", uuid.hyphenated().to_string());

            Ok(uuid.hyphenated().to_string().into_response())
        }
        Err(error) => match error {
            DataAccessError::AlreadyExist(id) => {
                Ok(Redirect::to(&format!("/media/{media_id}/{id}")).into_response())
            }
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
