/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 */

//! Remove a media item from an album and delete the underlying file.
//!
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::{
    auth::{
        permissions::{has_album_permission, AlbumPermission},
        user::User,
    },
    database::ArcDynDatabase,
    zip_cache::ZipCacheManager,
};
use std::path::Path as FsPath;
use std::sync::Arc;

use crate::repository::MediaRepositoryState;

pub(crate) async fn delete_album_media(
    State(_repo): State<MediaRepositoryState>,
    Extension(db): Extension<ArcDynDatabase>,
    Extension(zip_cache): Extension<Arc<ZipCacheManager>>,
    Path((album_id, media_id)): Path<(String, String)>,
    user: User,
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

    if let Ok(Some((filepath, filename))) = db.get_media_file_path(&media_id).await {
        let full_path = FsPath::new(&filepath).join(&filename);
        let _ = tokio::fs::remove_file(&full_path).await;
        let _ = tokio::fs::remove_dir(&filepath).await;
    }

    match db.delete_media_item(&media_id).await {
        Ok(()) => {
            zip_cache.invalidate(&album_id).await;
            zip_cache.schedule_generation(album_id.clone(), db.clone()).await;
            (StatusCode::OK, Json(serde_json::json!({"status": "deleted"}))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
