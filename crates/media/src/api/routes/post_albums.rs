/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 */

//! Creates a new album for the current user
//!
use axum::{extract::State, http::StatusCode, Json};
use common::auth::user::User;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use crate::repository::MediaRepositoryState;

#[derive(Deserialize)]
pub struct CreateAlbumRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CreateAlbumResponse {
    pub id: String,
}

pub(crate) async fn post_albums(
    State(repo): State<MediaRepositoryState>,
    user: User,
    Json(body): Json<CreateAlbumRequest>,
) -> Result<(StatusCode, Json<CreateAlbumResponse>), StatusCode> {
    let user_id = Uuid::parse_str(user.uuid.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;

    if body.name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    match repo.create_album(user_id, body.name, body.description).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreateAlbumResponse { id }))),
        Err(e) => {
            error!("Failed to create album: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
