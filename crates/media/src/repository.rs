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

use std::sync::Arc;
use std::time::Instant;

use axum::async_trait;
use common::config::database_config::DatabaseConfig;
use common::config::database_config::DatabaseDriver;
use database::Database;

use rand::{distributions::Alphanumeric, Rng};
use tracing::error;
use tracing::info;
use uuid::Uuid;

use crate::data::error::DataAccessError;
use crate::data::media_item::MediaItem;

pub struct MediaRepository {
    pub(crate) database: Database,
}

pub type MediaRepositoryState = Arc<dyn MediaRepositoryTrait + Send + Sync>;

/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MediaRepositoryTrait {
    // Gets a list of media items from the DB filted by user_id
    fn get_media_items_for_user(&self, user_id: Uuid) -> Result<Vec<MediaItem>, DataAccessError>;
}

impl MediaRepository<'a> {
    pub async fn new(database: Database) -> self {
        self { database }
    }
}

#[async_trait]
impl MediaRepositoryTrait for MediaRepository {
    fn get_media_items_for_user(&self, user_id: Uuid) -> Result<Vec<MediaItem>, DataAccessError> {
        info!("get items for user {}", user_id);

        // TODO: read from database
        Ok(vec![MediaItem {
            uuid: "".into(),
            name: "",
            date_added: Instant::now(),
            date_taken: None,
            details: None,
            tags: None,
            location: None,
            references: None,
        }])
    }

    //    async fn get_media_item() {
    // TODO: read from filesystem
    //  }
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new() {
        // given
        let repository = MediaRepository::new(
            common::config::database_config::DatabaseDriver::SQLite,
            "file::memory:?cache=shared".into(),
        )
        .await;

        // when
        let result = repository.get_media_items_for_user(Uuid::new_v4());

        // then
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap().len(), 1);
    }
}
