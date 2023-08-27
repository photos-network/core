//! Returns a list of owned media items for current user
//!
use axum::{extract::Query, http::StatusCode, Json};
use common::http::extractors::{self, optuser::OptionalUser};
use common::model::auth::user::User;
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize)]
pub(crate) struct MediaListQuery {
    offset: Option<i32>,
    limit: Option<i32>,
}

pub(crate) async fn get_media(
    user: OptionalUser,
    Query(query): Query<MediaListQuery>,
) -> Result<Json<String>, StatusCode> {
    //tracing::error!("GET /media user={}", user);
    // TODO: check auth header
    // TODO: read list from persistency
    // TODO: return list
    Ok(Json(
        format!(
            "list media items. limit={}, offset={}",
            query.limit.unwrap_or_else(|| 1000),
            query.offset.unwrap_or_else(|| 0)
        )
        .to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use hyper::{Body, Request};
    use tower::ServiceExt;

    use crate::api::router::MediaApi;

    use super::*;

    #[tokio::test]
    async fn get_media_unauthorized_should_not_fail() {
        // given
        let app = Router::new().nest("/", MediaApi::routes());

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
        assert_eq!(response.status(), StatusCode::OK);
    }
}
