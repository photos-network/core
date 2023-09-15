//! Returns a list of owned media items for current user
//!
use axum::extract::State;
use axum::{extract::Query, http::StatusCode, Json};
use common::auth::user::User;
use serde::{Deserialize, Serialize};
use std::result::Result;
use tracing::error;
use uuid::Uuid;

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
        repo.get_media_items_for_user(Uuid::parse_str(user.uuid.as_str()).unwrap());
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
    use std::collections::HashMap;

    use axum::Router;
    use common::{config::configuration::Configuration, ApplicationState};
    use database::sqlite::SqliteDatabase;
    use hyper::{Body, Request};
    use sqlx::SqlitePool;
    use tower::ServiceExt;

    use crate::api::router::MediaApi;

    use super::*;

    #[sqlx::test]
    async fn get_media_unauthorized_should_not_fail(pool: SqlitePool) {
        // given
        let state: ApplicationState<SqliteDatabase> = ApplicationState {
            config: Configuration::empty(),
            plugins: HashMap::new(),
            router: None,
            database: SqliteDatabase { pool },
        };

        let app = Router::new().nest("/", MediaApi::routes(state).await);

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
