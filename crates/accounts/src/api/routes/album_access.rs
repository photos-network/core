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
use serde::Deserialize;
use super::customer::extract_session;

// ── Access-code management ────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AddAccessCodeRequest {
    pub access_code: String,
}

#[derive(Deserialize)]
pub struct GenerateCodeRequest {
    pub display_name: String,
}

pub async fn list_album_codes(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    match db.get_access_codes_for_album(&album_id).await {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn add_album_code(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
    Json(req): Json<AddAccessCodeRequest>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    let customer = match db.get_customer_by_access_code(&req.access_code).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Access code not found"}))).into_response(),
    };
    match db.assign_album_to_customer(&album_id, &customer.customer_id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "assigned"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn remove_album_code(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path((album_id, access_code)): Path<(String, String)>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    let customer = match db.get_customer_by_access_code(&access_code).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Access code not found"}))).into_response(),
    };
    match db.unassign_album_from_customer(&album_id, &customer.customer_id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "removed"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

#[derive(Deserialize)]
pub struct GrantAccessRequest {
    pub account_id: String,
    pub role: String, // "owner" or "viewer"
}

pub async fn grant_album_access(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
    Json(req): Json<GrantAccessRequest>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner)
        .await
        .unwrap_or(false)
    {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    if req.role != "owner" && req.role != "viewer" {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "role must be 'owner' or 'viewer'"}))).into_response();
    }
    match db.grant_album_to_account(&req.account_id, &album_id, &req.role).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "granted"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn revoke_album_access(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path((album_id, target_account_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner)
        .await
        .unwrap_or(false)
    {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    match db.revoke_album_from_account(&target_account_id, &album_id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "revoked"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn list_album_access(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner)
        .await
        .unwrap_or(false)
    {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    match db.list_accounts_for_album(&album_id).await {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn generate_album_code(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
    Json(req): Json<GenerateCodeRequest>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner or admin access required"}))).into_response();
    }
    match db.generate_and_assign_code(&album_id, &req.display_name).await {
        Ok(code) => (StatusCode::OK, Json(serde_json::json!({"access_code": code}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
