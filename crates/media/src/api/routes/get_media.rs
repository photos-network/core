//! Returns a list of owned media items for current user
//!
use axum::extract::State;
use axum::{extract::Query, http::StatusCode, Json};
use common::model::auth::user::User;
use serde::{Deserialize, Serialize};
use std::result::Result;
use tracing::error;

use crate::data::error::DataAccessError;
use crate::data::media_item::MediaItem;
use crate::repository::MediaRepositoryState;

#[derive(Serialize, Deserialize)]
pub(crate) struct MediaListQuery {
    offset: Option<i32>,
    limit: Option<i32>,
}

pub(crate) async fn get_media(
    State(repo): State<MediaRepositoryState>,
    user: User,
    Query(query): Query<MediaListQuery>,
) -> Result<Json<String>, StatusCode> {
    let items: Result<Vec<MediaItem>, DataAccessError> =
        repo.get_media_items_for_user(user.uuid.into());
    match items {
        Ok(i) => {
            error!("Found {} items for user.", i.len());
        }
        Err(_) => {
            error!("Failed to get media items!");
        }
    }
    //tracing::error!("GET /media user={}", user);
    // TODO: check auth header
    // TODO: read list from persistency
    // TODO: return list
    Ok(Json(
        format!(
            "list media items. limit={}, offset={}",
            query.limit.unwrap_or(1000),
            query.offset.unwrap_or(0)
        )
        .to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use common::config::configuration::Configuration;
    use hyper::{Body, Request};
    use tower::ServiceExt;

    use crate::api::router::MediaApi;

    use super::*;

    #[tokio::test]
    async fn get_media_unauthorized_should_not_fail() {
        // given
        let app = Router::new().nest("/", MediaApi::routes(Configuration::empty()).await);

        // when
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/media")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
