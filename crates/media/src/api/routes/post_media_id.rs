//! Add files for a specific media item
//!

use axum::extract::{Multipart, Path};
use axum::http::StatusCode;
use common::model::auth::user::User;
use tracing::{debug, error};

pub(crate) async fn post_media_id(
    Path(media_id): Path<String>,
    user: User,
    mut multipart: Multipart,
) -> std::result::Result<String, StatusCode> {
    error!("POST /media/{}  user={}", media_id, user);
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "description" => {
                    debug!("description={}", field.text().await.unwrap());
                }
                "file" => {
                    // TODO: wrap bytes and write to persistency
                    debug!("filesize={}", field.chunk().await.unwrap().unwrap().len());
                }
                _ => continue,
            }
        }
    }

    // TODO: write file to storage
    // TODO: add file reference in database

    Err(StatusCode::OK)
}
