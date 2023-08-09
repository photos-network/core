//! Creates a new media item to aggregate related files for current user
//! 

use axum::http::StatusCode;

pub(crate) async fn post_media() -> std::result::Result<String, StatusCode> {
    // TODO: read authentication header
    Err(StatusCode::NOT_IMPLEMENTED)
}
