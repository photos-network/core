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

//! This crate offers a database abstraction for [Photos.network](https://photos.network) core application.
//!
use anyhow::Result;
use async_trait::async_trait;
use common::auth::user::User;
use common::database::media_item::MediaItem;
use common::database::reference::Reference;
use common::database::Database;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;
use sqlx::Row;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = PgPool::connect(db_url).await?;

        // run migrations from `migrations` directory
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(PostgresDatabase { pool })
    }
}

#[async_trait]
impl Database for PostgresDatabase {
    async fn get_users(&self) -> Result<Vec<User>> {
        let query = "SELECT uuid, email, password, lastname, firstname FROM users";

        let res = sqlx::query(query);

        let rows = res.fetch_all(&self.pool).await?;

        let users = rows
            .iter()
            .map(|row| User {
                uuid: row.get("uuid"),
                email: row.get("email"),
                password: row.get("password"),
                lastname: row.get("lastname"),
                firstname: row.get("firstname"),
                is_locked: false,
                created_at: Utc::now(),
                updated_at: None,
                last_login: None,
            })
            .collect();

        Ok(users)
    }

    async fn create_user(&self, user: &User) -> Result<()> {
        let query = "INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)";
        let id = Uuid::new_v4().hyphenated().to_string();
        info!("create new user with id `{}`.", id);
        sqlx::query(query)
            .bind(id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.lastname)
            .bind(&user.firstname)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_user(&self, _user_id: &str) -> Result<User> {
        unimplemented!()
    }

    async fn update_email(&self, email: &str, user_id: &str) -> Result<()> {
        let query = "UPDATE users SET email = $1 WHERE uuid = $2";

        sqlx::query(query)
            .bind(email)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_nickname(&self, _nickname: &str) -> Result<()> {
        unimplemented!()
    }

    async fn update_names(&self, _firstname: &str, _lastname: &str, _user_id: &str) -> Result<()> {
        unimplemented!()
    }

    async fn disable_user(&self, _user_id: &str) -> Result<()> {
        unimplemented!()
    }
    async fn enable_user(&self, _user_id: &str) -> Result<()> {
        unimplemented!()
    }

    async fn get_media_items(
        &self,
        _user_id: &str,
        _years: Vec<i32>,
        _months: Vec<i32>,
    ) -> Result<Vec<MediaItem>> {
        unimplemented!()
    }

    /// Creates a new media item if it doesn't exist and returns the media_id
    async fn create_media_item(
        &self,
        user_id: &str,
        name: &str,
        date_taken: DateTime<Utc>,
    ) -> Result<String> {
        let query = "SELECT COUNT(*) FROM media WHERE owner is $1 and taken_at like $2";
        let res = sqlx::query(query).bind(user_id).bind(date_taken);
        let rows = res.fetch_all(&self.pool).await?;

        if rows.len() > 1 {
            // TODO: return media item id for existing item
            // rows.first()
        } else {
            let query = "INSERT INTO media (uuid, owner, name, is_sensitive, added_at, taken_at) VALUES ($1, $2, $3, $4, $5, $6)";
            let id = Uuid::new_v4().hyphenated().to_string();
            info!("create new media item with id `{}`.", id);

            sqlx::query(query)
                .bind(id.clone())
                .bind(user_id)
                .bind(name)
                .bind(false)
                .bind(Utc::now())
                .bind(date_taken)
                .execute(&self.pool)
                .await?;
        }

        Ok("".to_string())
    }
    async fn get_media_item(&self, _media_id: &str) -> Result<MediaItem> {
        unimplemented!()
    }
    async fn add_reference(
        &self,
        _user_id: &str,
        _media_id: &str,
        _reference: &Reference,
    ) -> Result<String> {
        unimplemented!()
    }

    async fn update_reference(&self, _reference_id: &str, _reference: &Reference) -> Result<()> {
        unimplemented!()
    }

    async fn remove_reference(&self, _media_id: &str, _reference_id: &str) -> Result<()> {
        unimplemented!()
    }
}
