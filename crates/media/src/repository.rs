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
use anyhow::Result;
use axum::async_trait;
use bytes::Bytes;
use common::config::configuration::Configuration;
use common::database::reference::Reference;
use common::database::ArcDynDatabase;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

#[allow(dead_code)]
pub struct MediaRepository {
    pub(crate) database: ArcDynDatabase,
    pub(crate) config: Arc<Configuration>,
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

    async fn add_reference_for_media_item(
        &self,
        user_id: Uuid,
        media_id: &str,
        name: String,
        bytes: Bytes,
    ) -> Result<Uuid, DataAccessError>;
}

impl MediaRepository {
    pub async fn new(database: ArcDynDatabase, config: Arc<Configuration>) -> Self {
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
        return match items_result {
            Ok(items) => {
                Ok(items
                    .iter()
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
                    .collect())
            }
            Err(_) => Err(DataAccessError::OtherError),
        };
    }

    async fn create_media_item_for_user(
        &self,
        user_id: Uuid,
        name: String,
        date_taken: OffsetDateTime,
    ) -> Result<Uuid, DataAccessError> {
        debug!("user_id: {}", user_id.clone().hyphenated().to_string());
        let db_result = &self
            .database
            .create_media_item(
                user_id.hyphenated().to_string().as_str(),
                name.as_str(),
                date_taken,
            )
            .await;

        match db_result {
            Ok(id) => Ok(Uuid::parse_str(id.as_str()).unwrap()),
            Err(_) => Err(DataAccessError::OtherError),
        }
    }

    async fn add_reference_for_media_item(
        &self,
        user_id: Uuid,
        media_id: &str,
        name: String,
        bytes: Bytes,
    ) -> Result<Uuid, DataAccessError> {
        let path = Path::new("data/files/")
            .join(user_id.hyphenated().to_string())
            .join(media_id);

        let file_path = path.join(&name);
        let _ = fs::create_dir_all(&path);

        info!("target {}", path.clone().to_str().unwrap().to_string());
        debug!("got {} bytes to handle", bytes.len());
        let size = bytes.len();

        let file_result = tokio::fs::write(&path.join(&name), &bytes).await;
        match file_result {
            Ok(_) => {
                info!("wrote to {}", file_path.to_str().unwrap().to_string());
            }
            Err(_) => {
                warn!(
                    "Could not write file to path {}",
                    path.clone().to_str().unwrap().to_string()
                );
            }
        }

        let reference = Reference {
            uuid: Uuid::new_v4().hyphenated().to_string(),
            filepath: path.clone().to_str().unwrap().to_string(),
            filename: name.to_string(),
            size: size.try_into().unwrap(),
            description: "",
            last_modified: OffsetDateTime::now_utc(),
            is_missing: false,
        };
        let db_result = &self
            .database
            .add_reference(
                user_id.hyphenated().to_string().as_str(),
                media_id,
                &reference,
            )
            .await;

        match db_result {
            Ok(uuid) => {
                info!("added reference with id {}", uuid.clone());
                Ok(Uuid::parse_str(uuid.as_str()).unwrap())
            }
            Err(e) => {
                error!("Could not write reference to database! {:?}", e);
                Err(DataAccessError::OtherError)
            }
        }
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::*;

    #[tokio::test]
    async fn get_media_items_should_succeed() -> Result<()> {
        // given
        let user_id = "605EE8BE-BAF2-4499-B8D4-BA8C74E8B242";
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_media_items()
            .return_once(|_| Ok(Vec::new()));
        let repository =
            MediaRepository::new(Arc::new(mock_db), Configuration::empty().into()).await;

        // when
        let result = repository
            .get_media_items_for_user(Uuid::parse_str(user_id).unwrap())
            .await;

        // then
        assert!(result.is_ok());
        //assert_eq!(result.ok().unwrap().len(), 1);

        Ok(())
    }
}
