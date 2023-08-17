//! Creates a new media item to aggregate related files for current user
//! 

use axum::http::StatusCode;
use axum::extract::Multipart;

pub(crate) async fn post_media(mut multipart: Multipart) -> std::result::Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }
    // TODO: read authentication header
    Err(StatusCode::NOT_IMPLEMENTED)
}
