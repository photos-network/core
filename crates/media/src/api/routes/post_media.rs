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

//! Creates a new media item to aggregate related files for current user
//!
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use common::auth::user::User;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{data::error::DataAccessError, repository::MediaRepositoryState};

#[derive(Serialize, Deserialize)]
pub struct ResponseId {
    pub id: String,
}

pub(crate) async fn post_media(
    State(repo): State<MediaRepositoryState>,
    user: User,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    info!("POST /media");

    let mut name = None;
    let mut date_taken = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "name" => name = Some(field.text().await.unwrap()),
                "date_taken" => date_taken = Some(field.text().await.unwrap()),
                _ => continue,
            }
        }
    }

    if name.is_none() || date_taken.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let date = date_taken.unwrap().parse::<DateTime<Utc>>();
    if date.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = repo
        .create_media_item_for_user(
            Uuid::parse_str(user.uuid.as_str()).unwrap(),
            name.clone().unwrap(),
            date.unwrap(),
        )
        .await;

    match result {
        Ok(uuid) => {
            debug!(
                "name={}, taken={} => id={}",
                name.unwrap(),
                date.unwrap(),
                uuid.hyphenated().to_string()
            );

            Ok((
                StatusCode::CREATED,
                Json(ResponseId {
                    id: uuid.hyphenated().to_string(),
                }),
            )
                .into_response())
        }
        Err(error) => match error {
            DataAccessError::AlreadyExist(id) => {
                Ok(Redirect::to(&format!("/media/{id}")).into_response())
            }
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io;
    use std::sync::Arc;

    use axum::Router;
    use common::{config::configuration::Configuration, ApplicationState};
    use database::sqlite::SqliteDatabase;
    use hyper::{Body, Request};
    use mime::BOUNDARY;
    use sqlx::SqlitePool;
    use tokio::fs::File;
    use tower::ServiceExt;

    use crate::api::router::MediaApi;
    use axum::http::header::CONTENT_TYPE;
    use std::io::Write;
    use std::path::PathBuf;
    use testdir::testdir;
    use tokio::io::AsyncReadExt;

    use super::*;

    #[sqlx::test]
    async fn post_media_unauthorized_should_fail(pool: SqlitePool) {
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
                    .method("POST")
                    .uri("/media")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test]
    #[ignore]
    async fn post_media_authorized_without_name_field(pool: SqlitePool) {
        // given
        let state: ApplicationState = ApplicationState {
            config: Configuration::empty().into(),
            plugins: HashMap::new(),
            router: None,
            database: Arc::new(SqliteDatabase { pool }),
        };
        let app = Router::new().nest("/", MediaApi::routes(&state).await);
        let data = media_item_form_data().await.unwrap();

        // when
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/media")
                    .header(hyper::header::AUTHORIZATION, "FakeAuth")
                    .header(
                        CONTENT_TYPE,
                        format!("multipart/form-data; boundary={}", BOUNDARY),
                    )
                    // .header(CONTENT_TYPE, &*format!("multipart/form-data; boundary={}", BOUNDARY))
                    .body(data.into())
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);
    }

    async fn media_item_form_data() -> io::Result<Vec<u8>> {
        let mut data: Vec<u8> = Vec::new();

        write!(data, "--{}\r\n", BOUNDARY)?;
        write!(data, "Content-Disposition: form-data; name=\"name\";\r\n")?;
        write!(data, "\r\n")?;
        write!(data, "DSC_1234")?;
        write!(data, "\r\n")?;

        write!(data, "--{}\r\n", BOUNDARY)?;
        write!(
            data,
            "Content-Disposition: form-data; name=\"date_taken\";\r\n"
        )?;
        write!(data, "\r\n")?;
        write!(data, "1985-04-12T23:20:50.52Z")?;
        write!(data, "\r\n")?;

        write!(data, "--{}--\r\n", BOUNDARY)?;

        Ok(data)
    }

    #[allow(dead_code)]
    async fn image_data() -> io::Result<Vec<u8>> {
        let dir: PathBuf = testdir!();
        let path = dir.join("11.jpg");
        std::fs::write(&path, "fake image data").ok();

        let mut data: Vec<u8> = Vec::new();
        write!(data, "--{}\r\n", BOUNDARY)?;
        write!(
            data,
            "Content-Disposition: form-data; name=\"DSC_1234\"; filename=\"11.jpg\"\r\n"
        )?;
        write!(data, "Content-Type: image/jpeg\r\n")?;
        write!(data, "\r\n")?;

        let mut f = File::open(path).await?;
        f.read_to_end(&mut data).await?;

        write!(data, "\r\n")?; // The key thing you are missing
        write!(data, "--{}--\r\n", BOUNDARY)?;

        Ok(data)
    }
}
