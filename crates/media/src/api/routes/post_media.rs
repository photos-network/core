//! Creates a new media item to aggregate related files for current user
//!
use axum::{extract::Multipart, http::StatusCode};
use common::auth::user::User;
use tracing::{debug, error};

pub(crate) async fn post_media(
    user: User,
    mut _multipart: Multipart,
) -> std::result::Result<String, StatusCode> {
    error!("POST /media user={}", user);

    let id = uuid::Uuid::new_v4();
    debug!("add media with id {} into database", id);
    // TODO: check if media already exists for user

    // TODO: add item in database for user
    Err(StatusCode::NOT_IMPLEMENTED)
}
