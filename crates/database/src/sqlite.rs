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
use sqlx::sqlite::SqliteQueryResult;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::Row;
use sqlx::SqlitePool;
use std::i64;
use tracing::error;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteDatabase {
    pub pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(db_url).await?;

        // run migrations from `migrations` directory
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(SqliteDatabase { pool })
    }
}

#[async_trait]
impl Database for SqliteDatabase {
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
        info!("create new user with id `{}`.", &user.uuid);
        sqlx::query(query)
            .bind(&user.uuid)
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

    async fn get_media_items(&self, _user_id: &str) -> Result<Vec<MediaItem>> {
        unimplemented!()
    }
    async fn create_media_item(
        &self,
        user_id: &str,
        name: &str,
        date_taken: DateTime<Utc>,
    ) -> Result<String> {
        struct Item {
            uuid: String,
        }

        let rows: Option<Item> = sqlx::query_as!(
            Item,
            "SELECT uuid FROM media WHERE owner is $1 AND name is $2 AND taken_at is $3",
            user_id,
            name,
            date_taken,
        )
        .fetch_optional(&self.pool)
        .await?;

        return match rows {
            Some(r) => {
                info!("Found media item with same 'name' and 'taken_at' for owner.");

                Ok(r.uuid)
            }
            _ => {
                let query = "INSERT INTO media (uuid, owner, name, is_sensitive, added_at, taken_at) VALUES ($1, $2, $3, $4, $5, $6)";
                let id = Uuid::new_v4().hyphenated().to_string();

                let db_result = sqlx::query(query)
                    .bind(id.clone())
                    .bind(&user_id.to_string())
                    .bind(&name.to_string())
                    .bind(false)
                    .bind(Utc::now())
                    .bind(date_taken)
                    .execute(&self.pool)
                    .await;

                match db_result {
                    Ok(_) => {
                        info!("New media item created with id {}.", id)
                    }
                    Err(e) => {
                        error!("Could not create new media item in database! {}", e);
                    }
                }

                Ok(id)
            }
        };
    }

    async fn get_media_item(&self, _media_id: &str) -> Result<MediaItem> {
        unimplemented!()
    }

    async fn add_reference(
        &self,
        user_id: &str,
        media_id: &str,
        reference: &Reference,
    ) -> Result<String> {
        let query = "INSERT INTO reference (uuid, media, owner, filepath, filename, size) VALUES ($1, $2, $3, $4, $5, $6)";
        let id = Uuid::new_v4().hyphenated().to_string();
        let _res: SqliteQueryResult = sqlx::query(query)
            .bind(id.clone())
            .bind(media_id)
            .bind(user_id)
            .bind(&reference.filepath)
            .bind(&reference.filename)
            .bind(i64::try_from(reference.size).unwrap())
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    async fn update_reference(&self, _reference_id: &str, _reference: &Reference) -> Result<()> {
        unimplemented!()
    }

    async fn remove_reference(&self, _media_id: &str, _reference_id: &str) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use testdir::testdir;

    #[sqlx::test]
    async fn create_user_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/create_user_should_succeed.sqlite",
        )
        .await?;

        // when
        for i in 0..3 {
            let user = User {
                uuid: Uuid::new_v4().hyphenated().to_string(),
                email: format!("test_{}@photos.network", i),
                password: Some("unsecure".into()),
                lastname: Some("Stuermer".into()),
                firstname: Some("Benjamin".into()),
                is_locked: false,
                created_at: Utc::now(),
                updated_at: None,
                last_login: None,
            };

            // when
            let _ = db.create_user(&user).await;
        }

        // then
        let count = sqlx::query("SELECT COUNT(*) AS 'count!' FROM users")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.get::<i32, _>("count!"), 3);

        Ok(())
    }

    #[sqlx::test]
    async fn create_already_existing_user_should_fail(pool: SqlitePool) -> Result<()> {
        // given
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/create_already_existing_user_should_fail.sqlite",
        )
        .await?;

        // when
        let uuid = uuid::Uuid::new_v4().hyphenated().to_string();
        let user = User {
            uuid,
            email: "info@photos.network".into(),
            password: Some("unsecure".into()),
            lastname: Some("Stuermer".into()),
            firstname: Some("Benjamin".into()),
            is_locked: false,
            created_at: Utc::now(),
            updated_at: None,
            last_login: None,
        };

        // then
        let result1 = db.create_user(&user.clone()).await;
        assert!(result1.is_ok());

        let result2 = db.create_user(&user.clone()).await;
        assert!(result2.is_err());

        let count = sqlx::query("SELECT COUNT(*) AS 'count!' FROM users")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.get::<i32, _>("count!"), 1);

        Ok(())
    }

    #[sqlx::test]
    async fn update_email_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/update_email_should_succeed.sqlite",
        )
        .await?;

        // when
        let result = db
            .update_email(
                "security@photos.network",
                "570DC079-664A-4496-BAA3-668C445A447",
            )
            .await;

        // then
        assert!(result.is_ok());
        let count = sqlx::query("SELECT email FROM users LIMIT 1")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.get::<String, _>("email"), "security@photos.network");

        Ok(())
    }

    #[sqlx::test]
    async fn update_email_to_existing_should_fail(pool: SqlitePool) -> Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;

        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("0D341AD3-D38F-455F-8411-E25186665FC5")
            .bind("security@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;

        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/update_email_to_existing_should_fail.sqlite",
        )
        .await?;

        // when
        let result = db
            .update_email(
                "security@photos.network",
                "570DC079-664A-4496-BAA3-668C445A447",
            )
            .await;

        // then
        assert!(result.is_err());

        let rows = sqlx::query("SELECT email FROM users")
            .fetch_all(&pool)
            .await?;
        assert_eq!(rows[0].get::<String, _>("email"), "info@photos.network");
        assert_eq!(rows[1].get::<String, _>("email"), "security@photos.network");

        Ok(())
    }

    #[sqlx::test]
    async fn get_users_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/get_users_should_succeed.sqlite",
        )
        .await?;

        // when
        let users = db.get_users().await.unwrap();

        // then
        assert_eq!(users.clone().len(), 1);
        assert_eq!(
            users.get(0).unwrap().uuid,
            "570DC079-664A-4496-BAA3-668C445A447"
        );

        Ok(())
    }

    //noinspection DuplicatedCode
    #[sqlx::test]
    async fn create_media_item_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        // create fake user - used as FOREIGN KEY in media
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind(user_id)
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/create_media_item_should_succeed.sqlite",
        )
        .await?;

        let name = "DSC_1234";
        let date_taken = Utc::now();

        // when
        let media_item_result = db.create_media_item(user_id, name, date_taken).await;

        // then
        assert!(media_item_result.is_ok());

        Ok(())
    }

    //noinspection DuplicatedCode
    #[sqlx::test]
    async fn create_media_item_should_return_existing_uuid(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        let media_id = "ef9ac799-02f3-4b3f-9d96-7576be0434e6";
        let added_at = "2023-02-03T13:37:01.234567Z"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let taken_at = "2023-01-01T13:37:01.234567Z"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let name = "DSC_1234";

        // create fake user - used as FOREIGN KEY in media
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind(user_id)
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;

        sqlx::query("INSERT INTO media (uuid, owner, name, is_sensitive, added_at, taken_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(media_id)
            .bind(user_id)
            .bind("DSC_1234")
            .bind(false)
            .bind(added_at)
            .bind(taken_at)
            .execute(&pool).await?;

        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/create_media_item_should_return_existing_uuid.sqlite",
        )
        .await?;

        // when
        let media_item_result = db.create_media_item(user_id, name, taken_at).await;

        // then
        assert!(media_item_result.is_ok());
        assert_eq!(media_item_result.ok().unwrap(), media_id.to_string());

        Ok(())
    }

    //noinspection DuplicatedCode
    #[sqlx::test]
    async fn add_reference_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        let media_id = "ef9ac799-02f3-4b3f-9d96-7576be0434e6";
        let reference_id = "ef9ac799-02f3-4b3f-9d96-7576be0434e6";
        let added_at = "2023-02-03T13:37:01.234567Z"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let taken_at = "2023-01-01T13:37:01.234567Z"
            .parse::<DateTime<Utc>>()
            .unwrap();
        // create fake user - used as FOREIGN KEY in reference
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind(user_id)
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        // create fake media item - used as FOREIGN KEY in reference
        sqlx::query("INSERT INTO media (uuid, owner, name, is_sensitive, added_at, taken_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(media_id)
            .bind(user_id)
            .bind("DSC_1234")
            .bind(false)
            .bind(added_at)
            .bind(taken_at)
            .execute(&pool).await?;
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/add_reference_should_succeed.sqlite",
        )
        .await?;

        let filename = "DSC_1234.jpg";
        let dir: PathBuf = testdir!();
        let path = dir.join(filename);
        let filepath = path.clone().to_str().unwrap().to_string();
        std::fs::write(&path, "fake image data").ok();
        let metadata = std::fs::metadata(path.clone()).unwrap();

        let reference = Reference {
            uuid: reference_id.to_string(),
            filepath,
            filename: filename.to_string(),
            size: metadata.len(),
            description: "",
            last_modified: "2023-02-03T13:37:01.234567Z"
                .parse::<DateTime<Utc>>()
                .unwrap(),
            is_missing: false,
        };

        // when
        let add_reference_result = db.add_reference(user_id, media_id, &reference).await;

        // then
        assert!(add_reference_result.is_ok());

        Ok(())
    }
}
