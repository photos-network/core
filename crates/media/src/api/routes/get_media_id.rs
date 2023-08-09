//! Returns a specific owned or shared media item for current user
//!

use axum::http::StatusCode;

pub(crate) async fn get_media_id() -> std::result::Result<String, StatusCode> {
    // TODO: parse params  max-with / max-height   =wmax-width-hmax-height  (=w2048-h1024)
    // -wmax-width   (preserving the aspect ratio)
    // -hmax-height  (preserving the aspect ratio)
    // -c  crop images to max-width / max-height
    // -d  remove exif data

    Err(StatusCode::NOT_IMPLEMENTED)
}
