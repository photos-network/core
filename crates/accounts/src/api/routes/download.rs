use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use common::{
    auth::permissions::{has_album_permission, AlbumPermission},
    database::ArcDynDatabase,
    zip_cache::{build_zip_to_file, zip_tmp_path, ZipCacheManager},
};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

use super::customer::extract_session;

const BUILDING_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="10">
  <title>Preparing download…</title>
  <style>
    body { font-family: sans-serif; display: flex; flex-direction: column;
           align-items: center; justify-content: center; min-height: 100vh;
           margin: 0; background: #f5f5f5; color: #333; }
    .card { background: #fff; border-radius: 8px; padding: 2rem 3rem;
            box-shadow: 0 2px 8px rgba(0,0,0,.1); text-align: center; max-width: 400px; }
    h1 { font-size: 1.2rem; margin-bottom: .5rem; }
    p  { color: #666; font-size: .9rem; margin: .25rem 0; }
    .spinner { width: 40px; height: 40px; border: 4px solid #e0e0e0;
               border-top-color: #555; border-radius: 50%;
               animation: spin 1s linear infinite; margin: 1.5rem auto; }
    @keyframes spin { to { transform: rotate(360deg); } }
  </style>
</head>
<body>
  <div class="card">
    <h1>Preparing your download</h1>
    <div class="spinner"></div>
    <p>Your album ZIP is being created.</p>
    <p>This page refreshes automatically every 10 seconds.</p>
  </div>
</body>
</html>"#;

/// Parses a single `bytes=start-end` range. Returns `Ok(Some(start, end))` for a valid range,
/// `Ok(None)` to ignore (serve full file), `Err(())` for a satisfiable-but-out-of-bounds range (→ 416).
fn parse_byte_range(range_str: &str, file_size: u64) -> Result<Option<(u64, u64)>, ()> {
    let s = match range_str.strip_prefix("bytes=") {
        Some(s) if !s.contains(',') => s, // skip multi-range
        _ => return Ok(None),
    };
    let (start_s, end_s) = match s.split_once('-') {
        Some(p) => p,
        None => return Ok(None),
    };
    // Suffix ranges (bytes=-N) not needed for resumption — ignore.
    if start_s.trim().is_empty() {
        return Ok(None);
    }
    let start: u64 = start_s.trim().parse().map_err(|_| ())?;
    let end: u64 = if end_s.trim().is_empty() {
        file_size.saturating_sub(1)
    } else {
        end_s.trim().parse().map_err(|_| ())?
    };
    if start >= file_size || start > end {
        return Err(()); // 416
    }
    Ok(Some((start, end.min(file_size - 1))))
}

pub async fn download_album_zip(
    State(db): State<ArcDynDatabase>,
    headers: HeaderMap,
    Path(album_id): Path<String>,
) -> impl IntoResponse {
    let (caller_id, role) = match extract_session(&headers) {
        Ok(p) => p,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    let has_access = match role.as_str() {
        "account" => has_album_permission(&db, &caller_id, &album_id, AlbumPermission::Read)
            .await
            .unwrap_or(false),
        "customer" => db
            .get_albums_for_customer(&caller_id)
            .await
            .map(|albums| albums.iter().any(|a| a.album_id == album_id))
            .unwrap_or(false),
        _ => false,
    };

    if !has_access {
        return StatusCode::FORBIDDEN.into_response();
    }

    let album_name = db
        .get_album(&album_id)
        .await
        .map(|a| a.name)
        .unwrap_or_else(|_| album_id.clone());

    let selected = if role == "customer" {
        db.get_customer_album_items(&caller_id, &album_id)
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };

    let cache_path = if selected.is_empty() {
        ZipCacheManager::all_zip_path(&album_id)
    } else {
        ZipCacheManager::customer_zip_path(&album_id, &caller_id)
    };

    // Cache miss: kick off background build and return a self-refreshing page.
    // This avoids blocking the proxy connection for the duration of the zip build.
    if !cache_path.exists() {
        let tmp = zip_tmp_path(&cache_path);
        if !tmp.exists() {
            // No build in progress — start one.
            let mut media_items = match db.get_media_for_album(&album_id).await {
                Ok(items) => items,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };
            if !selected.is_empty() {
                media_items.retain(|m| selected.contains(&m.uuid));
            }
            let path_clone = cache_path.clone();
            let db_clone = db.clone();
            tokio::spawn(async move {
                if let Err(e) = build_zip_to_file(&media_items, &path_clone, &db_clone).await {
                    tracing::warn!("ZIP build failed for album {}: {:?}", album_id, e);
                }
            });
        }
        return (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            BUILDING_HTML,
        )
            .into_response();
    }

    let safe_name: String = album_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    let disposition = format!("attachment; filename=\"{}.zip\"", safe_name);

    let mut file = match tokio::fs::File::open(&cache_path).await {
        Ok(f) => f,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let file_size = match file.metadata().await {
        Ok(m) => m.len(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/zip"));
    resp_headers.insert(axum::http::header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    if let Ok(val) = HeaderValue::from_str(&disposition) {
        resp_headers.insert(axum::http::header::CONTENT_DISPOSITION, val);
    }

    // Handle Range header for partial / resumable downloads.
    if let Some(range_val) = headers.get(axum::http::header::RANGE) {
        if let Ok(range_str) = range_val.to_str() {
            match parse_byte_range(range_str, file_size) {
                Ok(Some((start, end))) => {
                    let length = end - start + 1;
                    if file.seek(std::io::SeekFrom::Start(start)).await.is_err() {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                    let stream = ReaderStream::with_capacity(file.take(length), 256 * 1024);
                    if let Ok(cr) = HeaderValue::from_str(&format!("bytes {}-{}/{}", start, end, file_size)) {
                        resp_headers.insert(axum::http::header::CONTENT_RANGE, cr);
                    }
                    if let Ok(cl) = HeaderValue::from_str(&length.to_string()) {
                        resp_headers.insert(axum::http::header::CONTENT_LENGTH, cl);
                    }
                    return (StatusCode::PARTIAL_CONTENT, resp_headers, StreamBody::new(stream)).into_response();
                }
                Err(()) => {
                    // Valid syntax but out of bounds → 416.
                    let cr = format!("bytes */{}", file_size);
                    return (
                        StatusCode::RANGE_NOT_SATISFIABLE,
                        [(axum::http::header::CONTENT_RANGE, cr.as_str())],
                        "",
                    ).into_response();
                }
                Ok(None) => {} // unrecognised range unit — fall through to full response
            }
        }
    }

    if let Ok(val) = HeaderValue::from_str(&file_size.to_string()) {
        resp_headers.insert(axum::http::header::CONTENT_LENGTH, val);
    }
    let stream = ReaderStream::with_capacity(file, 256 * 1024);
    (StatusCode::OK, resp_headers, StreamBody::new(stream)).into_response()
}
