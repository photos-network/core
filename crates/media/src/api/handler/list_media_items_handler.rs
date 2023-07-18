//! Returns a list of media items owned or shared with the user
//! 

use axum::http::StatusCode;

pub(crate) async fn list_media_items_handler() -> std::result::Result<String, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
