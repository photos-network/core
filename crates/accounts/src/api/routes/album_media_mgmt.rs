use axum::{
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use common::{
    auth::permissions::{has_album_permission, AlbumPermission},
    database::{reference::Reference, ArcDynDatabase},
};
use std::fs;
use std::path::Path as FsPath;
use uuid::Uuid;

use super::customer::extract_session;

pub async fn upload_album_media(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner access required"}))).into_response();
    }

    let mut filename: Option<String> = None;
    let mut file_bytes: Option<bytes::Bytes> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            file_bytes = field.bytes().await.ok();
        }
    }

    let filename = match filename {
        Some(f) if !f.is_empty() => f,
        _ => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "No file provided"}))).into_response(),
    };
    let bytes = match file_bytes {
        Some(b) if !b.is_empty() => b,
        _ => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Empty file"}))).into_response(),
    };

    let media_id = Uuid::new_v4().hyphenated().to_string();
    let name = filename.clone();

    let created_id = match db.create_media_item(&caller_id, &name, Utc::now()).await {
        Ok(id) if !id.is_empty() => id,
        _ => media_id.clone(),
    };

    let storage_dir = FsPath::new("data/files/").join(&caller_id).join(&created_id);
    let _ = fs::create_dir_all(&storage_dir);
    if let Err(e) = tokio::fs::write(storage_dir.join(&filename), &bytes).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("File write failed: {}", e)}))).into_response();
    }

    let reference = Reference {
        uuid: Uuid::new_v4().hyphenated().to_string(),
        filepath: storage_dir.to_str().unwrap_or("").to_string(),
        filename: filename.clone(),
        size: bytes.len() as u64,
        description: String::new(),
        last_modified: Utc::now(),
        is_missing: false,
    };
    if let Err(e) = db.add_reference(&caller_id, &created_id, &reference).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("DB reference failed: {}", e)}))).into_response();
    }

    if let Err(e) = db.add_media_to_album(&album_id, &created_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Album link failed: {}", e)}))).into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"media_id": created_id, "name": filename}))).into_response()
}

pub async fn delete_album_media(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path((album_id, media_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    if role != "account" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Account token required"}))).into_response();
    }
    if !has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Owner).await.unwrap_or(false) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Owner access required"}))).into_response();
    }

    // Remove file from disk before removing from DB
    if let Ok(Some((filepath, filename))) = db.get_media_file_path(&media_id).await {
        let full_path = FsPath::new(&filepath).join(&filename);
        let _ = tokio::fs::remove_file(&full_path).await;
        // Remove directory if empty
        let _ = tokio::fs::remove_dir(&filepath).await;
    }

    if let Err(e) = db.delete_media_item(&media_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"status": "deleted"}))).into_response()
}
