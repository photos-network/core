/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
    let items: Result<Vec<MediaItem>, DataAccessError> = repo
        .get_media_items_for_user(Uuid::parse_str(user.uuid.as_str()).unwrap())
        .await;
    match items {
        Ok(i) => {
            error!("Found {} items for user.", i.len());
        }
        Err(_) => {
            error!("Failed to get media items!");
        }
    }
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
    use std::{collections::HashMap, sync::Arc};

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
        let state: ApplicationState = ApplicationState {
            config: Configuration::empty().into(),
            plugins: HashMap::new(),
            router: None,
            database: Arc::new(SqliteDatabase { pool }),
        };

        let app = Router::new().nest("/", MediaApi::routes(&state).await);

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
