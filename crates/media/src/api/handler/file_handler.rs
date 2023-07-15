//! Returns the binary of a given entity
//!

use axum::http::StatusCode;

pub(crate) async fn file_handler() -> std::result::Result<String, StatusCode> {
    // TODO: parse params  max-with / max-height   =wmax-width-hmax-height  (=w2048-h1024)
    // -wmax-width   (preserving the aspect ratio)
    // -hmax-height  (preserving the aspect ratio)
    // -c  crop images to max-width / max-height
    // -d  remove exif data

    Err(StatusCode::NOT_IMPLEMENTED)
}
