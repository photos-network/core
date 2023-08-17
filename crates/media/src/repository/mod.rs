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

use axum::async_trait;
use mockall::predicate::*;
use sea_orm::DatabaseConnection;

use crate::data::error::DataAccessError;
use crate::data::media_item::MediaItem;
use crate::data::open_db_conn;

pub struct MediaRepository {
    #[allow(dead_code)]
    db_url: &'static str,
    #[allow(dead_code)]
    db: DatabaseConnection,
}

#[allow(dead_code)]
pub(crate) type MediaRepositoryState = Arc<MediaRepository>;

/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, mockall::automock)]
#[async_trait]
trait MediaRepositoryTrait {
    #[allow(dead_code)]
    async fn new(db_url: &'static str) -> Self;
    
    // Gets a list of media items from the DB filted by user_id
    async fn get_media_items_for_user(&self, user_id: &str) -> Result<Vec<MediaItem>, DataAccessError>;
}

impl MediaRepository {
    #[allow(dead_code)]
    pub(crate) async fn new() -> Self {
        Self {
            db_url: "",
            db: DatabaseConnection::Disconnected
        }
    }        
}
    
#[async_trait]
impl MediaRepositoryTrait for MediaRepository {
    async fn new(db_url: &'static str) -> MediaRepository {
        let db = open_db_conn("sqlite://data/media.sqlita".to_string()).await.expect("Could not connect do database 'media'!");

        MediaRepository {
            db,
            db_url

        }
    }

    async fn get_media_items_for_user(&self, _user_id: &str) -> Result<Vec<MediaItem>, DataAccessError> {
        // TODO: read from database

        Err(DataAccessError::OtherError)
    }
}
