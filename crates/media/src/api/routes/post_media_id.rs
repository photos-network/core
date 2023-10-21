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
use common::auth::user::User;
use hyper::header::LOCATION;
use hyper::HeaderMap;
use tempfile::tempfile;
use tokio::fs::File;
use tracing::{debug, error};
use uuid::Uuid;

use std::io::SeekFrom;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

use crate::data::error::DataAccessError;
use crate::repository::MediaRepositoryState;

pub(crate) async fn post_media_id(
    State(repo): State<MediaRepositoryState>,
    Path(media_id): Path<String>,
    user: User,
    mut multipart: Multipart,
) -> Result<String, StatusCode> {
    error!("POST /media/{}  user={}", media_id, user);
    let mut headers = HeaderMap::new();
    let tempfile = tempfile().unwrap();
    let mut tempfile = File::from_std(tempfile);
    let mut name: String = "".to_string();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "name" => {
                    name = field.text().await.unwrap();
                    debug!("name={}", name.clone());
                }
                "file" => {
                    while let Some(chunk) = field
                        .chunk()
                        .await
                        .expect("Could not read file from multipart upload!")
                    {
                        tempfile
                            .write_all(&chunk)
                            .await
                            .expect("Could not write reference file to tmp!")
                    }
                    tempfile.seek(SeekFrom::Start(0)).await.unwrap();

                    // TODO: wrap bytes and write to persistence
                    debug!("filesize={}", field.chunk().await.unwrap().unwrap().len());
                }
                _ => continue,
            }
        }
    }

    let result = repo
        .add_reference_for_media_item(
            Uuid::parse_str(user.uuid.as_str()).unwrap(),
            media_id,
            name,
            tempfile,
        )
        .await;

    match result {
        Ok(uuid) => {
            debug!("reference added. uuid={}", uuid.hyphenated().to_string());

            Ok(uuid.hyphenated().to_string())
        }
        Err(error) => {
            match error {
                DataAccessError::AlreadyExist(id) => {
                    // TODO: use Redirect::permanent to add a Location header to the already existing item

                    let location = format!("/media/{}", id);
                    headers.insert(LOCATION, location.parse().unwrap());

                    Err(StatusCode::SEE_OTHER)
                }
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}
