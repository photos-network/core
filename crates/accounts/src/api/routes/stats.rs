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

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use common::{
    auth::permissions::{has_album_permission, AlbumPermission},
    database::ArcDynDatabase,
};

use super::customer::extract_session;

pub async fn get_album_stats(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    match db.get_album_stats(&album_id).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn get_owned_album_stats(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account required"}))).into_response();
    }
    match db.get_stats_for_owned_albums(&caller_id).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
