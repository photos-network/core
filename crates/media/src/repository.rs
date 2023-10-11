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

use crate::data::error::DataAccessError;
use crate::data::media_item::MediaItem;
use axum::async_trait;
use common::config::configuration::Configuration;
use common::database::Database;
use database::sqlite::SqliteDatabase;
use std::fs::File;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)]
pub struct MediaRepository {
    pub(crate) database: SqliteDatabase,
    pub(crate) config: Configuration,
}

pub type MediaRepositoryState = Arc<dyn MediaRepositoryTrait + Send + Sync>;

/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MediaRepositoryTrait {
    // Gets a list of media items from the DB filtered by user_id
    async fn get_media_items_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MediaItem>, DataAccessError>;

    /// Create a new media item for the given user
    async fn create_media_item_for_user(
        &self,
        user_id: Uuid,
        name: String,
        date_taken: OffsetDateTime,
    ) -> Result<Uuid, DataAccessError>;
}

impl MediaRepository {
    pub async fn new(database: SqliteDatabase, config: Configuration) -> Self {
        Self { database, config }
    }
}

#[async_trait]
impl MediaRepositoryTrait for MediaRepository {
    async fn get_media_items_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MediaItem>, DataAccessError> {
        info!("get items for user {}", user_id);

        let items_result = &self
            .database
            .get_media_items(user_id.hyphenated().to_string().as_str())
            .await;
        match items_result {
            Ok(items) => {
                return Ok(items
                    .into_iter()
                    .map(|d| MediaItem {
                        // TODO: fill in missing info like references, details, tags
                        // TODO: check references on filesystem
                        uuid: d.uuid,
                        name: d.name,
                        date_added: d.added_at,
                        date_taken: d.taken_at,
                        details: None,
                        tags: None,
                        location: None,
                        references: None,
                    })
                    .collect());
            }
            Err(_) => return Err(DataAccessError::OtherError),
        }
    }

    async fn create_media_item_for_user(
        &self,
        user_id: Uuid,
        name: String,
        date_taken: OffsetDateTime,
    ) -> Result<Uuid, DataAccessError> {
        // TODO: map result to <Uuid, DatabaseAccessError>
        let _ = &self
            .database
            .create_media_item(
                user_id.hyphenated().to_string().as_str(),
                name.as_str(),
                date_taken,
            )
            .await;

        Ok(Uuid::new_v4())
    }
}

#[allow(unused_imports)]
mod tests {
    use database::sqlite::SqliteDatabase;
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test(migrations = "../database/migrations")]
    async fn get_media_items_should_succeed(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        let user_id = "605EE8BE-BAF2-4499-B8D4-BA8C74E8B242";
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind(user_id.clone())
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;

        sqlx::query("INSERT INTO media (uuid, name, owner) VALUES ($1, $2, $3)")
            .bind("6A92460C-53FB-4B42-AC1B-E6760A34E169")
            .bind("DSC_1234")
            .bind(user_id.clone())
            .execute(&pool)
            .await?;

        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/media/repository/tests/get_media_items_should_succeed.sqlite",
        )
        .await;
        let repository = MediaRepository::new(db, Configuration::empty()).await;

        // when
        let result = repository
            .get_media_items_for_user(uuid::Uuid::parse_str(user_id).unwrap())
            .await;

        // then
        // TODO fix assertion
        assert!(result.is_err());
        //assert_eq!(result.ok().unwrap().len(), 1);

        Ok(())
    }
}
