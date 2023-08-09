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
use mockall::predicate::*;
use sea_orm::DatabaseConnection;

use crate::data::error::DataAccessError;
use crate::data::media_item::MediaItem;
use crate::data::open_db_conn;

struct MediaRepository {
    db_url: &'static str,
    db: DatabaseConnection,
}

/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, mockall::automock)]
#[async_trait]
trait MediaRepositoryTrait {
    async fn new(db_url: &'static str) -> Self;
    
    // Gets a list of media items from the DB filted by user_id
    async fn get_media_items_for_user(&self, user_id: &str) -> Result<Vec<MediaItem>, DataAccessError>;
}

impl MediaRepository {
    // async fn new(&self) {
    // }        
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


#[cfg(test)]
mod tests {
    use sea_orm::{DbConn, Schema, DbBackend, sea_query::TableCreateStatement};

    use super::*;

    async fn setup_schema(db: &DbConn) {
        let schema = Schema::new(DbBackend::Sqlite);
    
        // Derive from Entity
        let stmt: TableCreateStatement = schema.create_table_from_entity(MyEntity);
    
        // Or setup manually
        assert_eq!(
            stmt.build(SqliteQueryBuilder),
            Table::create()
                .table(MyEntity)
                .col(
                    ColumnDef::new(MyEntity::Column::Id)
                        .integer()
                        .not_null()
                )
                //...
                .build(SqliteQueryBuilder)
        );
    
        // Execute create table statement
        let result = db
            .execute(db.get_database_backend().build(&stmt))
            .await;
    }

    #[rstest]
    #[case("/?name=Wonder", "Wonder%")] // Verify that % is appended to the filter
    async fn get_media_items_for_user_success(#[case] uri: &'static str, #[case] expected_filter: &'static str) {

        let mut repo_mock = MockMediaRepositoryTrait::new("sqlite::memory:");
        setup_schema(&db).await?;
        testcase(&db).await?;
    }
}
