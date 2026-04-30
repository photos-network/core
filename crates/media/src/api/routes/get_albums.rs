/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 */

//! Returns a list of albums owned by the current user
//!
use axum::{extract::{Extension, State}, http::StatusCode, Json};
use common::auth::user::User;
use common::database::album::Album;
use common::database::ArcDynDatabase;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use tracing::error;
use uuid::Uuid;

use crate::repository::MediaRepositoryState;

#[derive(Serialize)]
pub struct AlbumResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Album> for AlbumResponse {
    fn from(a: Album) -> Self {
        Self {
            id: a.album_id,
            name: a.name,
            description: a.description,
            created_at: a.created_at,
        }
    }
}

pub(crate) async fn get_albums(
    State(repo): State<MediaRepositoryState>,
    Extension(db): Extension<ArcDynDatabase>,
    user: User,
) -> Result<Json<Vec<AlbumResponse>>, StatusCode> {
    let user_id = Uuid::parse_str(user.uuid.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;

    if user.is_admin {
        match db.list_all_albums().await {
            Ok(albums) => return Ok(Json(albums.into_iter().map(AlbumResponse::from).collect())),
            Err(e) => {
                error!("Failed to list all albums: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    match repo.get_albums_for_user(user_id).await {
        Ok(albums) => Ok(Json(albums.into_iter().map(AlbumResponse::from).collect())),
        Err(e) => {
            error!("Failed to get albums: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
