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
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Row;
use std::error::Error;
use tracing::{error, info};
use uuid::Uuid;

pub struct User {
    pub uuid: String,
    pub email: String,
    pub password: String,
    pub lastname: String,
    pub firstname: String,
}

#[async_trait]
pub trait Database {
    async fn setup(&mut self) -> Result<(), Box<dyn Error>>;
    async fn create_user(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn update_user(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>>;
}

pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(db_url: &str) -> Self {
        let pool = sqlx::postgres::PgPool::connect(db_url).await.unwrap();

        PostgresDatabase { pool }
    }
}

#[async_trait]
impl Database for PostgresDatabase {
    async fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        // run migrations from `migrations` directory
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }

    async fn create_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
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

    async fn update_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
        let query = "UPDATE users SET email = %1 WHERE uuid = $2";

        sqlx::query(query)
            .bind(&user.email)
            .bind(&user.uuid)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
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
            })
            .collect();

        Ok(users)
    }
}
