/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 */

//! Upload a file, create a media item, and link it to an album in one step.
//!
use axum::{
    extract::{Extension, Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::types::chrono::Utc;
use common::{
    auth::{
        permissions::{has_album_permission, AlbumPermission},
        user::User,
    },
    database::ArcDynDatabase,
    zip_cache::ZipCacheManager,
};
use std::fs;
use std::path::Path as FsPath;
use std::sync::Arc;
use uuid::Uuid;

use crate::repository::MediaRepositoryState;

pub(crate) async fn post_albums_id_media(
    State(repo): State<MediaRepositoryState>,
    Extension(db): Extension<ArcDynDatabase>,
    Extension(zip_cache): Extension<Arc<ZipCacheManager>>,
    Path(album_id): Path<String>,
    user: User,
    mut multipart: Multipart,
) -> impl IntoResponse {
    if !has_album_permission(&db, &user.uuid, &album_id, AlbumPermission::Owner)
        .await
        .unwrap_or(false)
    {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "Owner access required"})),
        )
            .into_response();
    }

    let mut filename: Option<String> = None;
    let mut file_bytes: Option<bytes::Bytes> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name().unwrap_or("") == "file" {
            filename = field.file_name().map(|s| s.to_string());
            file_bytes = field.bytes().await.ok();
        }
    }

    let filename = match filename {
        Some(f) if !f.is_empty() => f,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "No file provided"})),
            )
                .into_response()
        }
    };
    let bytes = match file_bytes {
        Some(b) if !b.is_empty() => b,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Empty file"})),
            )
                .into_response()
        }
    };

    let user_id = match uuid::Uuid::parse_str(&user.uuid) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Invalid user id"})),
            )
                .into_response()
        }
    };

    let media_id = match repo
        .create_media_item_for_user(user_id, filename.clone(), Utc::now())
        .await
    {
        Ok(id) => id.hyphenated().to_string(),
        Err(_) => Uuid::new_v4().hyphenated().to_string(),
    };

    let storage_dir = FsPath::new("data/files/").join(&user.uuid).join(&media_id);
    let _ = fs::create_dir_all(&storage_dir);
    if let Err(e) = tokio::fs::write(storage_dir.join(&filename), &bytes).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("File write failed: {}", e)})),
        )
            .into_response();
    }

    if let Err(e) = repo
        .add_reference_for_media_item(user_id, &media_id, filename.clone(), bytes)
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("DB reference failed: {:?}", e)})),
        )
            .into_response();
    }

    if let Err(e) = repo.add_media_to_album(&album_id, &media_id).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Album link failed: {:?}", e)})),
        )
            .into_response();
    }

    // Invalidate stale cache and schedule eager re-generation after the debounce window.
    zip_cache.invalidate(&album_id).await;
    zip_cache.schedule_generation(album_id.clone(), db.clone()).await;

    (
        StatusCode::CREATED,
        Json(serde_json::json!({"media_id": media_id, "name": filename})),
    )
        .into_response()
}
