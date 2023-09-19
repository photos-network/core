//! Creates a new media item to aggregate related files for current user
//!
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use common::auth::user::User;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{data::error::DataAccessError, repository::MediaRepositoryState};

pub(crate) async fn post_media(
    State(repo): State<MediaRepositoryState>,
    user: User,
    mut multipart: Multipart,
) -> std::result::Result<String, StatusCode> {
    let mut name = None;
    let mut date_taken = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "name" => {
                    name = Some(field.text().await.unwrap());
                    // debug!("name={}", field.text().await.unwrap());
                }
                "date_taken" => {
                    date_taken = Some(field.text().await.unwrap());
                    // debug!("date_taken={}", field.text().await.unwrap());
                }
                _ => continue,
            }
        }
    }

    if name.is_none() || date_taken.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = repo.create_media_item_for_user(
        Uuid::parse_str(user.uuid.as_str()).unwrap(),
        name.clone().unwrap(),
        date_taken.clone().unwrap(),
    );

    match result {
        Ok(uuid) => {
            debug!(
                "name={}, taken={} => id={}",
                name.unwrap(),
                date_taken.unwrap(),
                uuid.clone().hyphenated().to_string()
            );

            Ok(uuid.hyphenated().to_string())
        }
        Err(error) => {
            match error {
                DataAccessError::AlreadyExist => {
                    // TODO: use Redirect::permanent to add a Location header to the already existing item
                    return Err(StatusCode::SEE_OTHER);
                }
                _ => {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }
}
