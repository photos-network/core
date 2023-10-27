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

use super::routes::delete_media_id::delete_media_id;
use super::routes::get_albums::get_albums;
use super::routes::get_albums_id::get_albums_id;
use super::routes::get_media::get_media;
use super::routes::get_media_id::get_media_id;
use super::routes::patch_albums_id::patch_albums_id;
use super::routes::patch_albums_id_share::patch_albums_id_share;
use super::routes::patch_albums_id_unshare::patch_albums_id_unshare;
use super::routes::patch_media_id::patch_media_id;
use super::routes::post_albums::post_albums;
use super::routes::post_media::post_media;
use super::routes::post_media_id::post_media_id;
use crate::repository::{MediaRepository, MediaRepositoryState};
use axum::routing::{delete, get, patch, post};
use axum::Router;
use common::ApplicationState;
use std::sync::Arc;

pub struct MediaApi {}

impl MediaApi {
    pub async fn routes<S>(state: &ApplicationState) -> Router<S>
    where
        S: Send + Sync + Clone,
    {
        let media_repository: MediaRepository =
            MediaRepository::new(Arc::clone(&state.database), Arc::clone(&state.config)).await;
        let repository_state: MediaRepositoryState = Arc::new(media_repository);

        Router::new()
            // Returns a list of owned media items for current user
            // 200 Ok
            // 401 Unauthorized - Requesting user is unauthenticated
            // 403 Forbidden
            // 500 Internal Server Error
            .route("/media", get(get_media))
            // Creates a new media item to aggregate related files for current user
            // 201 - Created
            // 400 Bad Request - The request body was malformed or a field violated its constraints.
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 500 Internal Server Error
            .route("/media", post(post_media))
            // Returns a specific owned or shared media item for current user
            // 200 - Ok
            // 400 Bad Request - The request body was malformed or a field violated its constraints.
            // 401 Unauthorized - You are unauthenticated
            // 403 Forbidden - You are authenticated but have no permission to manage the target user.
            // 500 Internal Server Error
            .route("/media/:media_id", get(get_media_id))
            // Add files for a specific media item
            .route("/media/:media_id", post(post_media_id))
            // Updates fields from a specific media item for current user
            .route("/media/:media_id", patch(patch_media_id))
            // Deletes the given item owned by the user
            .route("/media/:media_id", delete(delete_media_id))
            // list owned and shared albums
            .route("/albums", get(get_albums))
            // create new album
            .route("/albums", post(post_albums))
            // get metadata of a specific owned or shared album
            .route("/albums/:entity_id", get(get_albums_id))
            // updates the given album owned by the user
            .route("/albums/:entity_id", patch(patch_albums_id))
            // shares the given album
            .route("/albums/:entity_id/share", patch(patch_albums_id_share))
            // unshares the given album
            .route("/albums/:entity_id/unshare", patch(patch_albums_id_unshare))
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .with_state(repository_state)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use common::{config::configuration::Configuration, database::MockDatabase};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn get_media_with_query_success() {
        // given
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_media_items()
            .return_once(|_| Ok(Vec::new()));
        let state: ApplicationState = ApplicationState {
            config: Configuration::empty().into(),
            plugins: HashMap::new(),
            router: None,
            database: Arc::new(mock_db),
        };
        let app = Router::new().nest("/", MediaApi::routes(&state).await);

        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/media?limit=100000&offset=1")
                    .method("GET")
                    .header("Authorization", "FakeAuth")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: String = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, "list media items. limit=100000, offset=1");
    }

    #[tokio::test]
    async fn get_media_without_query_success() {
        // given
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_media_items()
            .return_once(|_| Ok(Vec::new()));
        let state: ApplicationState = ApplicationState {
            config: Configuration::empty().into(),
            plugins: HashMap::new(),
            router: None,
            database: Arc::new(mock_db),
        };
        let app = Router::new().nest("/", MediaApi::routes(&state).await);

        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/media")
                    .method("GET")
                    .header("Authorization", "FakeAuth")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: String = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, "list media items. limit=1000, offset=0");
    }

    #[tokio::test]
    async fn post_media_without_user_fail() {
        // given
        let mock_db = MockDatabase::new();
        let state: ApplicationState = ApplicationState {
            config: Configuration::empty().into(),
            plugins: HashMap::new(),
            router: None,
            database: Arc::new(mock_db),
        };
        let app = Router::new().nest("/", MediaApi::routes(&state).await);

        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/media")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(
                        "Content-Disposition",
                        "attachment; filename=\"DSC_1234.NEF\"",
                    )
                    //.body(Body::from(bytes))
                    //.body(Body::empty())
                    // TODO: add multipart file to body
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    // TODO: test is failing due to missing multi-part body
    //#[tokio::test]
    #[allow(dead_code)]
    async fn post_media_success() {
        // given
        let mock_db = MockDatabase::new();
        let state: ApplicationState = ApplicationState {
            config: Configuration::empty().into(),
            plugins: HashMap::new(),
            router: None,
            database: Arc::new(mock_db),
        };
        let app = Router::new().nest("/", MediaApi::routes(&state).await);

        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/media")
                    .method("POST")
                    .header("Authorization", "FakeAuth")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(
                        "Content-Disposition",
                        "attachment; filename=\"DSC_1234.NEF\"",
                    )
                    //.body(Body::from(bytes))
                    //.body(Body::empty())
                    // TODO: add multipart file to body
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }
}
