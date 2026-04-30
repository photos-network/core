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

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use common::auth::auth_manager::AuthManager;
use common::database::ArcDynDatabase;

#[derive(Debug, Deserialize)]
pub struct CustomerLoginRequest {
    pub access_code: String,
}

#[derive(Serialize)]
pub struct CustomerLoginResponse {
    pub access_code: String,
    pub customer_id: String,
    pub display_name: Option<String>,
    pub jwt_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerRegisterRequest {
    pub access_code: String,
    pub display_name: String,
}

pub async fn handle_customer_login(
    State(db): State<ArcDynDatabase>,
    Json(request): Json<CustomerLoginRequest>,
) -> impl IntoResponse {
    info!("Customer login attempt with access_code: {}", request.access_code);

    let auth_manager = AuthManager::new(Arc::clone(&db));

    match auth_manager
        .verify_access_code(request.access_code.clone())
        .await
    {
        Ok(response) => {
            info!("Customer login successful for access_code: {}", request.access_code);
            let _ = auth_manager.update_last_login(response.customer_id.clone()).await;

            let response_json = CustomerLoginResponse {
                access_code: response.access_code,
                customer_id: response.customer_id,
                display_name: response.display_name,
                jwt_token: response.jwt_token,
            };

            (StatusCode::OK, Json(response_json)).into_response()
        }
        Err(e) => {
            error!("Customer login failed: {}", e);
            let msg = e.to_string();
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": msg })),
            )
                .into_response()
        }
    }
}

pub async fn get_customer_albums(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let (id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    let albums_result = if role == "account" {
        db.get_albums_for_account(&id).await
    } else {
        db.get_albums_for_customer(&id).await
    };

    match albums_result {
        Ok(albums) => (StatusCode::OK, Json(albums)).into_response(),
        Err(e) => {
            error!("Failed to fetch albums for {} (role={}): {}", id, role, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to fetch albums" })),
            )
                .into_response()
        }
    }
}

pub async fn get_customer_album_media(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
) -> impl IntoResponse {
    let (id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    // For customer role: respect manual item selection; for account role: show all items
    let selected_ids = if role == "customer" {
        db.get_customer_album_items(&id, &album_id)
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };

    match db.get_media_for_album(&album_id).await {
        Ok(mut items) => {
            if !selected_ids.is_empty() {
                items.retain(|m| selected_ids.contains(&m.uuid));
            }

            // Record album view (non-blocking — don't fail request on error)
            let db_clone = db.clone();
            let album_id_clone = album_id.clone();
            let id_clone = id.clone();
            let role_clone = role.clone();
            tokio::spawn(async move {
                let _ = db_clone.record_album_view(&album_id_clone, &id_clone, &role_clone).await;
            });

            (StatusCode::OK, Json(items)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch media for album {}: {}", album_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to fetch media" })),
            )
                .into_response()
        }
    }
}

pub async fn get_customer_media_file(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(media_id): Path<String>,
) -> impl IntoResponse {
    use axum::http::header;

    let (id, role) = match extract_session(&headers) {
        Ok(pair) => pair,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let (filepath, filename) = match db.get_media_file_path(&media_id).await {
        Ok(Some(info)) => info,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!("Failed to get file path for media {}: {}", media_id, e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let full_path = format!("{}/{}", filepath, filename);

    let bytes = match tokio::fs::read(&full_path).await {
        Ok(b) => b,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // Record media download (non-blocking — don't fail request on error)
    let db_clone = db.clone();
    let media_id_clone = media_id.clone();
    let id_clone = id.clone();
    let role_clone = role.clone();
    tokio::spawn(async move {
        let _ = db_clone.record_media_download(&media_id_clone, None, &id_clone, &role_clone).await;
    });

    let content_type = if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
        "image/jpeg"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".gif") {
        "image/gif"
    } else if filename.ends_with(".webp") {
        "image/webp"
    } else {
        "application/octet-stream"
    };

    (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], bytes).into_response()
}

/// Extracts the session identity from the Authorization header.
/// Returns `(id, role)` where role is either `"customer"` or `"account"`.
pub fn extract_session(headers: &HeaderMap) -> Result<(String, String), anyhow::Error> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| anyhow::anyhow!("Missing or invalid Authorization header"))?;

    AuthManager::validate_jwt_token(token)
}

pub async fn handle_customer_register(
    State(db): State<ArcDynDatabase>,
    Json(request): Json<CustomerRegisterRequest>,
) -> impl IntoResponse {
    info!("Customer registration attempt with access_code: {}", request.access_code);

    if !AuthManager::validate_access_code(&request.access_code) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid access code format. Must be 6 uppercase letters or numbers."
            })),
        )
            .into_response();
    }

    let auth_manager = AuthManager::new(Arc::clone(&db));

    match auth_manager
        .create_customer(
            request.access_code.clone(),
            request.display_name.clone(),
        )
        .await
    {
        Ok(customer_id) => {
            info!("Customer successfully registered with access_code: {}", request.access_code);

            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "customer_id": customer_id,
                    "access_code": request.access_code,
                    "display_name": request.display_name,
                    "message": "Customer registered successfully"
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Customer registration failed: {}", e);
            let msg = e.to_string();
            (
                StatusCode::CONFLICT,
                Json(serde_json::json!({ "error": msg })),
            )
                .into_response()
        }
    }
}
