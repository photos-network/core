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

use axum::async_trait;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;
use uuid::Uuid;

use crate::data::error::DataAccessError;
use crate::data::media_item::MediaItem;

#[allow(dead_code)]
pub struct MediaRepository<D> {
    pub(crate) database: D,
}

pub type MediaRepositoryState = Arc<dyn MediaRepositoryTrait + Send + Sync>;

/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MediaRepositoryTrait {
    // Gets a list of media items from the DB filted by user_id
    fn get_media_items_for_user(&self, user_id: Uuid) -> Result<Vec<MediaItem>, DataAccessError>;
}

impl<D> MediaRepository<D> {
    pub async fn new(database: D) -> Self {
        Self { database }
    }
}

#[async_trait]
impl<D> MediaRepositoryTrait for MediaRepository<D> {
    fn get_media_items_for_user(&self, user_id: Uuid) -> Result<Vec<MediaItem>, DataAccessError> {
        info!("get items for user {}", user_id);

        // TODO: read from database
        // TODO: read from filesystem
        Ok(vec![MediaItem {
            uuid: "",
            name: "",
            date_added: Instant::now(),
            date_taken: None,
            details: None,
            tags: None,
            location: None,
            references: None,
        }])
    }
}

#[allow(unused_imports)]
mod tests {
    use database::sqlite::SqliteDatabase;
    use sqlx::SqlitePool;

    use super::*;

    #[sqlx::test(migrations = "../database/migrations")]
    async fn test_new(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        sqlx::query("INSERT INTO media (uuid, name, owner) VALUES ($1, $2, $3)")
            .bind("6A92460C-53FB-4B42-AC1B-E6760A34E169")
            .bind("DSC_1234")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .execute(&pool)
            .await?;

        let db = SqliteDatabase::new("target/sqlx/test-dbs/media/repository/tests/test_new.sqlite")
            .await;
        let repository = MediaRepository::new(db).await;

        // when
        let result = repository.get_media_items_for_user(Uuid::new_v4());

        // then
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap().len(), 1);

        Ok(())
    }
}
