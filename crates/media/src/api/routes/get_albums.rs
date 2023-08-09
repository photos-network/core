//! Returns the binary of a given entity
//!

use axum::http::StatusCode;

pub(crate) async fn get_albums() -> std::result::Result<String, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
