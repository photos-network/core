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
use common::auth::account::Account;
use common::auth::account_with_albums::{AccountWithAlbums, AlbumRef};
use common::auth::album_account::AlbumAccountEntry;
use common::auth::customer::Customer;
use common::database::album::Album;
use common::database::album_stats::{AlbumStats, ViewerEntry};
use common::database::{AlbumCodeEntry, Database};
use common::database::media_item::MediaItem;
use common::database::reference::Reference;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::SqlitePool;
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
    async fn get_media_items(&self, _user_id: &str) -> Result<Vec<MediaItem>> {
        error!("get_media_items not implemented!");
        Ok(vec![])
    }

    async fn delete_media_item(&self, media_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM reference WHERE media = $1")
            .bind(media_id)
            .execute(&self.pool)
            .await?;
        sqlx::query("DELETE FROM album_media WHERE media_id = $1")
            .bind(media_id)
            .execute(&self.pool)
            .await?;
        sqlx::query("DELETE FROM media WHERE uuid = $1")
            .bind(media_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn create_media_item(
        &self,
        user_id: &str,
        name: &str,
        date_taken: DateTime<Utc>,
    ) -> Result<String> {
        let rows = sqlx::query(
            "SELECT uuid FROM media WHERE owner = $1 AND name = $2 AND taken_at = $3",
        )
        .bind(user_id)
        .bind(name)
        .bind(date_taken)
        .fetch_optional(&self.pool)
        .await?;

        return match rows {
            Some(r) => {
                info!("Found media item with same 'name' and 'taken_at' for owner.");

                Ok(r.get("uuid"))
            }
            _ => {
                let query = "INSERT INTO media (uuid, owner, name, is_sensitive, added_at, taken_at) VALUES ($1, $2, $3, $4, $5, $6)";
                let id = Uuid::new_v4().hyphenated().to_string();

                let db_result = sqlx::query(query)
                    .bind(id.clone())
                    .bind(user_id.to_string())
                    .bind(name.to_string())
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

    ///// Customer operations /////

    async fn get_customer(&self, customer_id: &str) -> Result<Customer> {
        let query = "SELECT customer_id, access_code, display_name, created_at, updated_at, last_login_at FROM customers WHERE customer_id = $1";

        let row = sqlx::query_as::<_, Customer>(query)
            .bind(customer_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn create_customer(
        &self,
        customer_id: String,
        access_code: String,
        display_name: String,
    ) -> Result<()> {
        sqlx::query("INSERT INTO customers (customer_id, access_code, display_name) VALUES ($1, $2, $3)")
            .bind(&customer_id)
            .bind(&access_code)
            .bind(&display_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_customer_by_access_code(&self, code: &str) -> Result<Customer> {
        let customer = sqlx::query_as::<_, Customer>(
            "SELECT customer_id, access_code, display_name, created_at, updated_at, last_login_at FROM customers WHERE access_code = $1"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await?;

        customer.ok_or_else(|| anyhow::anyhow!("No customer found for access code"))
    }

    async fn update_last_login_for_customer(&self, customer_id: &str) -> Result<()> {
        sqlx::query("UPDATE customers SET last_login_at = $1 WHERE customer_id = $2")
            .bind(Utc::now())
            .bind(customer_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ///// Customer management /////

    async fn list_customers(&self) -> Result<Vec<Customer>> {
        let customers = sqlx::query_as::<_, Customer>(
            "SELECT customer_id, access_code, display_name, created_at, updated_at, last_login_at FROM customers ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(customers)
    }

    async fn delete_customer(&self, customer_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM customers WHERE customer_id = $1")
            .bind(customer_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ///// Account operations /////

    async fn create_account(
        &self,
        account_id: String,
        email: String,
        password_hash: String,
        display_name: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO accounts (account_id, email, password_hash, display_name) VALUES ($1, $2, $3, $4)"
        )
        .bind(&account_id)
        .bind(&email)
        .bind(&password_hash)
        .bind(&display_name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_account_by_email(&self, email: &str) -> Result<Account> {
        let account = sqlx::query_as::<_, Account>(
            "SELECT account_id, email, password_hash, display_name, created_at, updated_at, last_login_at, is_admin FROM accounts WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        account.ok_or_else(|| anyhow::anyhow!("No account found for email"))
    }

    async fn get_account_by_id(&self, account_id: &str) -> Result<Account> {
        let account = sqlx::query_as::<_, Account>(
            "SELECT account_id, email, password_hash, display_name, created_at, updated_at, last_login_at, is_admin FROM accounts WHERE account_id = $1"
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        account.ok_or_else(|| anyhow::anyhow!("No account found: {}", account_id))
    }

    async fn link_access_code_to_account(&self, account_id: &str, customer_id: &str) -> Result<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO account_access_codes (account_id, customer_id) VALUES ($1, $2)"
        )
        .bind(account_id)
        .bind(customer_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_customer_ids_for_account(&self, account_id: &str) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT customer_id FROM account_access_codes WHERE account_id = $1"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| r.get("customer_id")).collect())
    }

    async fn get_albums_for_account(&self, account_id: &str) -> Result<Vec<Album>> {
        let albums = sqlx::query_as::<_, Album>(
            "SELECT DISTINCT a.album_id, a.owner, a.name, a.description, a.cover_media_id, a.is_archived, a.created_at, a.updated_at \
             FROM albums a \
             WHERE a.is_archived = FALSE AND ( \
                 EXISTS ( \
                     SELECT 1 FROM album_accounts aa \
                     WHERE aa.album_id = a.album_id AND aa.account_id = $1 \
                 ) \
                 OR EXISTS ( \
                     SELECT 1 FROM account_access_codes aac \
                     JOIN customer_albums ca ON ca.customer_id = aac.customer_id \
                     WHERE aac.account_id = $1 AND ca.album_id = a.album_id \
                 ) \
             ) \
             ORDER BY a.created_at DESC"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    async fn update_last_login_for_account(&self, account_id: &str) -> Result<()> {
        sqlx::query("UPDATE accounts SET last_login_at = $1 WHERE account_id = $2")
            .bind(Utc::now())
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn is_account_admin(&self, account_id: &str) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT is_admin FROM accounts WHERE account_id = $1")
            .bind(account_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.get::<bool, _>("is_admin")).unwrap_or(false))
    }

    async fn set_account_admin(&self, account_id: &str, is_admin: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE accounts SET is_admin = $1 WHERE account_id = $2")
            .bind(is_admin)
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn grant_album_to_account(
        &self,
        account_id: &str,
        album_id: &str,
        role: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR REPLACE INTO album_accounts (account_id, album_id, role) VALUES ($1, $2, $3)"
        )
        .bind(account_id)
        .bind(album_id)
        .bind(role)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn revoke_album_from_account(
        &self,
        account_id: &str,
        album_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM album_accounts WHERE account_id = $1 AND album_id = $2"
        )
        .bind(account_id)
        .bind(album_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_album_account_role(
        &self,
        account_id: &str,
        album_id: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT role FROM album_accounts WHERE account_id = $1 AND album_id = $2"
        )
        .bind(account_id)
        .bind(album_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.get::<String, _>("role")))
    }

    async fn list_accounts_for_album(
        &self,
        album_id: &str,
    ) -> Result<Vec<AlbumAccountEntry>, sqlx::Error> {
        let entries = sqlx::query_as::<_, AlbumAccountEntry>(
            "SELECT account_id, album_id, role FROM album_accounts WHERE album_id = $1"
        )
        .bind(album_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    async fn get_albums_owned_by_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<Album>, sqlx::Error> {
        let albums = sqlx::query_as::<_, Album>(
            "SELECT a.album_id, a.owner, a.name, a.description, a.cover_media_id, a.is_archived, a.created_at, a.updated_at \
             FROM albums a \
             JOIN album_accounts aa ON aa.album_id = a.album_id \
             WHERE aa.account_id = $1 AND aa.role = 'owner' \
             ORDER BY a.created_at DESC"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    async fn list_all_accounts(&self) -> Result<Vec<Account>, sqlx::Error> {
        let accounts = sqlx::query_as::<_, Account>(
            "SELECT account_id, email, password_hash, display_name, created_at, updated_at, last_login_at \
             FROM accounts ORDER BY created_at"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }

    async fn list_all_albums(&self) -> Result<Vec<Album>, sqlx::Error> {
        let albums = sqlx::query_as::<_, Album>(
            "SELECT album_id, owner, name, description, cover_media_id, is_archived, created_at, updated_at \
             FROM albums ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    async fn get_accounts_with_albums(&self) -> Result<Vec<AccountWithAlbums>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT a.account_id, a.email, a.display_name, a.last_login_at, a.is_admin, \
                    aa.album_id, al.name as album_name, aa.role \
             FROM accounts a \
             LEFT JOIN album_accounts aa ON aa.account_id = a.account_id \
             LEFT JOIN albums al ON al.album_id = aa.album_id \
             ORDER BY a.account_id, aa.role"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut map: std::collections::BTreeMap<String, AccountWithAlbums> =
            std::collections::BTreeMap::new();

        for row in &rows {
            let account_id: String = row.get("account_id");
            let entry = map.entry(account_id.clone()).or_insert_with(|| AccountWithAlbums {
                account_id: account_id.clone(),
                email: row.get("email"),
                display_name: row.get("display_name"),
                last_login_at: row
                    .get::<Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, _>(
                        "last_login_at",
                    ),
                is_admin: row.get("is_admin"),
                albums: vec![],
            });
            if let Some(album_id) = row.get::<Option<String>, _>("album_id") {
                entry.albums.push(AlbumRef {
                    album_id,
                    album_name: row.get("album_name"),
                    role: row.get("role"),
                });
            }
        }

        Ok(map.into_values().collect())
    }

    ///// Album operations /////

    async fn create_album(&self, owner_id: &str, name: &str, description: Option<&str>) -> Result<String> {
        let album_id = Uuid::new_v4().hyphenated().to_string();

        sqlx::query(
            "INSERT INTO albums (album_id, owner, name, description) VALUES ($1, $2, $3, $4)"
        )
        .bind(&album_id)
        .bind(owner_id)
        .bind(name)
        .bind(description)
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "INSERT OR IGNORE INTO album_accounts (account_id, album_id, role) VALUES ($1, $2, 'owner')"
        )
        .bind(owner_id)
        .bind(&album_id)
        .execute(&self.pool)
        .await?;

        info!("Created album '{}' with id {}", name, album_id);
        Ok(album_id)
    }

    async fn get_albums_for_user(&self, owner_id: &str) -> Result<Vec<Album>> {
        let albums = sqlx::query_as::<_, Album>(
            "SELECT album_id, owner, name, description, cover_media_id, is_archived, created_at, updated_at FROM albums WHERE owner = $1 ORDER BY created_at DESC"
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    async fn get_album(&self, album_id: &str) -> Result<Album> {
        let album = sqlx::query_as::<_, Album>(
            "SELECT album_id, owner, name, description, cover_media_id, is_archived, created_at, updated_at FROM albums WHERE album_id = $1"
        )
        .bind(album_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Album not found: {}", album_id))?;

        Ok(album)
    }

    async fn update_album(&self, album_id: &str, name: Option<&str>, description: Option<&str>) -> Result<()> {
        if let Some(n) = name {
            sqlx::query("UPDATE albums SET name = $1, updated_at = $2 WHERE album_id = $3")
                .bind(n)
                .bind(Utc::now())
                .bind(album_id)
                .execute(&self.pool)
                .await?;
        }
        if let Some(d) = description {
            sqlx::query("UPDATE albums SET description = $1, updated_at = $2 WHERE album_id = $3")
                .bind(d)
                .bind(Utc::now())
                .bind(album_id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    async fn delete_album(&self, album_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM albums WHERE album_id = $1")
            .bind(album_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ///// Album-media junction /////

    async fn add_media_to_album(&self, album_id: &str, media_id: &str) -> Result<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO album_media (album_id, media_id) VALUES ($1, $2)"
        )
        .bind(album_id)
        .bind(media_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_media_from_album(&self, album_id: &str, media_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM album_media WHERE album_id = $1 AND media_id = $2")
            .bind(album_id)
            .bind(media_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_media_for_album(&self, album_id: &str) -> Result<Vec<MediaItem>> {
        let rows = sqlx::query(
            "SELECT m.uuid, m.name, m.added_at, m.taken_at FROM media m \
             JOIN album_media am ON am.media_id = m.uuid \
             WHERE am.album_id = $1 ORDER BY am.position ASC, m.name ASC"
        )
        .bind(album_id)
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .iter()
            .map(|row| MediaItem {
                uuid: row.get("uuid"),
                name: row.get("name"),
                added_at: row.get("added_at"),
                taken_at: row.get("taken_at"),
                details: None,
                tags: None,
                location: None,
                references: None,
            })
            .collect();

        Ok(items)
    }

    ///// Customer-album assignment /////

    async fn assign_album_to_customer(&self, album_id: &str, customer_id: &str) -> Result<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO customer_albums (customer_id, album_id) VALUES ($1, $2)"
        )
        .bind(customer_id)
        .bind(album_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn unassign_album_from_customer(&self, album_id: &str, customer_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM customer_albums WHERE customer_id = $1 AND album_id = $2")
            .bind(customer_id)
            .bind(album_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_albums_for_customer(&self, customer_id: &str) -> Result<Vec<Album>> {
        let albums = sqlx::query_as::<_, Album>(
            "SELECT a.album_id, a.owner, a.name, a.description, a.cover_media_id, a.is_archived, a.created_at, a.updated_at \
             FROM albums a \
             JOIN customer_albums ca ON ca.album_id = a.album_id \
             WHERE ca.customer_id = $1 AND a.is_archived = FALSE \
             ORDER BY ca.assigned_at DESC"
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(albums)
    }

    ///// Per-customer item selection /////

    async fn set_customer_album_items(&self, customer_id: &str, album_id: &str, media_ids: &[&str]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("DELETE FROM customer_album_items WHERE customer_id = $1 AND album_id = $2")
            .bind(customer_id)
            .bind(album_id)
            .execute(&mut *tx)
            .await?;

        for media_id in media_ids {
            sqlx::query(
                "INSERT INTO customer_album_items (customer_id, album_id, media_id) VALUES ($1, $2, $3)"
            )
            .bind(customer_id)
            .bind(album_id)
            .bind(media_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn get_customer_album_items(&self, customer_id: &str, album_id: &str) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT media_id FROM customer_album_items WHERE customer_id = $1 AND album_id = $2"
        )
        .bind(customer_id)
        .bind(album_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| r.get("media_id")).collect())
    }

    async fn get_media_file_path(&self, media_id: &str) -> Result<Option<(String, String)>> {
        let row = sqlx::query("SELECT filepath, filename FROM reference WHERE media = $1 LIMIT 1")
            .bind(media_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| (r.get::<String, _>("filepath"), r.get::<String, _>("filename"))))
    }

    ///// Stats /////

    async fn record_album_view(&self, album_id: &str, viewer_id: &str, viewer_role: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO album_views (album_id, viewer_id, viewer_role) VALUES ($1, $2, $3)"
        )
        .bind(album_id)
        .bind(viewer_id)
        .bind(viewer_role)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn record_media_download(&self, media_id: &str, album_id: Option<&str>, downloader_id: &str, downloader_role: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO media_downloads (media_id, album_id, downloader_id, downloader_role) VALUES ($1, $2, $3, $4)"
        )
        .bind(media_id)
        .bind(album_id)
        .bind(downloader_id)
        .bind(downloader_role)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_album_stats(&self, album_id: &str) -> Result<AlbumStats> {
        let total_views: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM album_views WHERE album_id = $1"
        )
        .bind(album_id)
        .fetch_one(&self.pool)
        .await?;

        let unique_viewers: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT viewer_id) FROM album_views WHERE album_id = $1"
        )
        .bind(album_id)
        .fetch_one(&self.pool)
        .await?;

        let total_downloads: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM media_downloads WHERE album_id = $1"
        )
        .bind(album_id)
        .fetch_one(&self.pool)
        .await?;

        let rows = sqlx::query(
            "SELECT viewer_id, viewer_role, COUNT(*) as view_count \
             FROM album_views WHERE album_id = $1 \
             GROUP BY viewer_id, viewer_role \
             ORDER BY view_count DESC"
        )
        .bind(album_id)
        .fetch_all(&self.pool)
        .await?;

        let viewers = rows
            .iter()
            .map(|row| ViewerEntry {
                viewer_id: row.get("viewer_id"),
                viewer_role: row.get("viewer_role"),
                view_count: row.get("view_count"),
            })
            .collect();

        Ok(AlbumStats {
            album_id: album_id.to_string(),
            total_views,
            unique_viewers,
            total_downloads,
            viewers,
        })
    }

    async fn get_stats_for_owned_albums(&self, account_id: &str) -> Result<Vec<AlbumStats>> {
        let rows = sqlx::query(
            "SELECT album_id FROM album_accounts WHERE account_id = $1 AND role = 'owner'"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        let album_ids: Vec<String> = rows.iter().map(|r| r.get("album_id")).collect();

        let mut stats = Vec::new();
        for album_id in album_ids {
            let s = self.get_album_stats(&album_id).await?;
            stats.push(s);
        }

        Ok(stats)
    }

    async fn get_access_codes_for_album(&self, album_id: &str) -> Result<Vec<AlbumCodeEntry>> {
        let rows = sqlx::query(
            "SELECT c.access_code, c.display_name, c.customer_id \
             FROM customer_albums ca \
             JOIN customers c ON c.customer_id = ca.customer_id \
             WHERE ca.album_id = $1 \
             ORDER BY c.display_name, c.access_code"
        )
        .bind(album_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| AlbumCodeEntry {
            access_code: r.get("access_code"),
            display_name: r.get("display_name"),
            customer_id: r.get("customer_id"),
        }).collect())
    }

    async fn generate_and_assign_code(&self, album_id: &str, display_name: &str) -> Result<String> {
        let customer_id = Uuid::new_v4().hyphenated().to_string();
        let access_code = self.generate_access_code();
        self.create_customer(customer_id.clone(), access_code.clone(), display_name.to_string()).await?;
        self.assign_album_to_customer(album_id, &customer_id).await?;
        Ok(access_code)
    }

    async fn generate_code(&self, display_name: &str) -> Result<String> {
        let customer_id = Uuid::new_v4().hyphenated().to_string();
        let access_code = self.generate_access_code();
        self.create_customer(customer_id, access_code.clone(), display_name.to_string()).await?;
        Ok(access_code)
    }

}

impl SqliteDatabase {
    #[inline]
    fn generate_access_code(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        let mut rng = rand::thread_rng();
        (0..8)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate and create a new customer with a generated access code
    pub async fn create_customer_with_generated_code(&self, display_name: &str) -> Result<String> {
        let customer_id = Uuid::new_v4().to_string();
        let access_code = self.generate_access_code();

        self.create_customer(customer_id.clone(), access_code, display_name.to_string()).await?;

        Ok(customer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use testdir::testdir;

    //noinspection DuplicatedCode
    #[sqlx::test]
    async fn create_media_item_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        sqlx::query("INSERT INTO accounts (account_id, email, password_hash) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind("info@photos.network")
            .bind("hash")
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

        sqlx::query("INSERT INTO accounts (account_id, email, password_hash) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind("info@photos.network")
            .bind("hash")
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
        sqlx::query("INSERT INTO accounts (account_id, email, password_hash) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind("info@photos.network")
            .bind("hash")
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
            description: String::new(),
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

    // Shared test helper: insert an account row directly into the pool
    async fn insert_test_user(pool: &SqlitePool, user_id: &str) -> Result<()> {
        sqlx::query("INSERT INTO accounts (account_id, email, password_hash, display_name) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(format!("{}@photos.network", user_id))
            .bind("hash")
            .bind("Test User")
            .execute(pool)
            .await?;
        Ok(())
    }

    // Shared test helper: insert a media row
    async fn insert_test_media(pool: &SqlitePool, media_id: &str, owner_id: &str) -> Result<()> {
        sqlx::query("INSERT INTO media (uuid, owner, name, is_sensitive, added_at, taken_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(media_id)
            .bind(owner_id)
            .bind("test_photo")
            .bind(false)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool)
            .await?;
        Ok(())
    }

    // Shared test helper: insert a customer row directly into the pool
    async fn insert_test_customer(pool: &SqlitePool, customer_id: &str, access_code: &str) -> Result<()> {
        sqlx::query("INSERT INTO customers (customer_id, access_code, display_name) VALUES ($1, $2, $3)")
            .bind(customer_id)
            .bind(access_code)
            .bind("Test Customer")
            .execute(pool)
            .await?;
        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_album_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        insert_test_user(&pool, user_id).await?;
        let db = SqliteDatabase { pool: pool.clone() };

        // when
        let result = db.create_album(user_id, "Vacation 2024", Some("Summer trip")).await;

        // then
        assert!(result.is_ok());
        let album_id = result.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM albums WHERE album_id = $1")
            .bind(&album_id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(count, 1);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_albums_for_user_should_return_owned_albums(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        insert_test_user(&pool, user_id).await?;
        let db = SqliteDatabase { pool: pool.clone() };
        db.create_album(user_id, "Album A", None).await?;
        db.create_album(user_id, "Album B", Some("desc")).await?;

        // when
        let albums = db.get_albums_for_user(user_id).await?;

        // then
        assert_eq!(albums.len(), 2);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn add_and_remove_media_from_album_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        let media_id = "ef9ac799-02f3-4b3f-9d96-7576be0434e6";
        insert_test_user(&pool, user_id).await?;
        insert_test_media(&pool, media_id, user_id).await?;
        let db = SqliteDatabase { pool: pool.clone() };
        let album_id = db.create_album(user_id, "Test Album", None).await?;

        // when
        db.add_media_to_album(&album_id, media_id).await?;
        let items = db.get_media_for_album(&album_id).await?;

        // then
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].uuid, media_id);

        // when - remove
        db.remove_media_from_album(&album_id, media_id).await?;
        let items_after = db.get_media_for_album(&album_id).await?;

        // then
        assert_eq!(items_after.len(), 0);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn assign_and_unassign_album_to_customer_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        let customer_id = "CUST-0001";
        insert_test_user(&pool, user_id).await?;
        insert_test_customer(&pool, customer_id, "ABC123").await?;
        let db = SqliteDatabase { pool: pool.clone() };
        let album_id = db.create_album(user_id, "Shared Album", None).await?;

        // when
        db.assign_album_to_customer(&album_id, customer_id).await?;
        let albums = db.get_albums_for_customer(customer_id).await?;

        // then
        assert_eq!(albums.len(), 1);
        assert_eq!(albums[0].name, "Shared Album");

        // when - unassign
        db.unassign_album_from_customer(&album_id, customer_id).await?;
        let albums_after = db.get_albums_for_customer(customer_id).await?;

        // then
        assert_eq!(albums_after.len(), 0);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn set_customer_album_items_should_override_selection(pool: SqlitePool) -> Result<()> {
        // given
        let user_id = "570DC079-664A-4496-BAA3-668C445A447";
        let customer_id = "CUST-0001";
        let media_id_1 = "ef9ac799-02f3-4b3f-9d96-7576be0434e6";
        let media_id_2 = "af9ac799-02f3-4b3f-9d96-7576be0434e7";
        insert_test_user(&pool, user_id).await?;
        insert_test_media(&pool, media_id_1, user_id).await?;
        insert_test_media(&pool, media_id_2, user_id).await?;
        insert_test_customer(&pool, customer_id, "ABC123").await?;
        let db = SqliteDatabase { pool: pool.clone() };
        let album_id = db.create_album(user_id, "Album", None).await?;
        db.assign_album_to_customer(&album_id, customer_id).await?;

        // when - set selection to both items
        db.set_customer_album_items(customer_id, &album_id, &[media_id_1, media_id_2]).await?;
        let items = db.get_customer_album_items(customer_id, &album_id).await?;
        assert_eq!(items.len(), 2);

        // when - override to one item
        db.set_customer_album_items(customer_id, &album_id, &[media_id_1]).await?;
        let items_after = db.get_customer_album_items(customer_id, &album_id).await?;
        assert_eq!(items_after.len(), 1);
        assert_eq!(items_after[0], media_id_1);

        // when - clear selection
        db.set_customer_album_items(customer_id, &album_id, &[]).await?;
        let items_cleared = db.get_customer_album_items(customer_id, &album_id).await?;
        assert_eq!(items_cleared.len(), 0);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_customer_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let db = SqliteDatabase { pool: pool.clone() };

        // when
        let result = db.create_customer(
            "CUST-0001".to_string(),
            "ABC123".to_string(),
            "Test Customer".to_string(),
        ).await;

        // then
        assert!(result.is_ok());
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers WHERE customer_id = 'CUST-0001'")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count, 1);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_customer_by_access_code_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        insert_test_customer(&pool, "CUST-0001", "XYZ789").await?;
        let db = SqliteDatabase { pool: pool.clone() };

        // when
        let result = db.get_customer_by_access_code("XYZ789").await;

        // then
        assert!(result.is_ok());
        let customer = result.unwrap();
        assert_eq!(customer.customer_id, "CUST-0001");
        assert_eq!(customer.access_code, "XYZ789");

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_account_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        let db = SqliteDatabase { pool: pool.clone() };

        // when
        let result = db.create_account(
            "ACC-0001".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Some("Test Account".to_string()),
        ).await;

        // then
        assert!(result.is_ok());
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM accounts WHERE account_id = 'ACC-0001'")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count, 1);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_account_by_email_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        sqlx::query("INSERT INTO accounts (account_id, email, password_hash, display_name) VALUES ($1, $2, $3, $4)")
            .bind("ACC-0001")
            .bind("test@example.com")
            .bind("hashed_pw")
            .bind("Test Account")
            .execute(&pool).await?;
        let db = SqliteDatabase { pool: pool.clone() };

        // when
        let result = db.get_account_by_email("test@example.com").await;

        // then
        assert!(result.is_ok());
        let account = result.unwrap();
        assert_eq!(account.account_id, "ACC-0001");
        assert_eq!(account.email, "test@example.com");

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn link_access_code_to_account_should_succeed(pool: SqlitePool) -> Result<()> {
        // given
        insert_test_customer(&pool, "CUST-0001", "ABC123").await?;
        sqlx::query("INSERT INTO accounts (account_id, email, password_hash, display_name) VALUES ($1, $2, $3, $4)")
            .bind("ACC-0001")
            .bind("test@example.com")
            .bind("hashed_pw")
            .bind("Test Account")
            .execute(&pool).await?;
        let db = SqliteDatabase { pool: pool.clone() };

        // when
        let result = db.link_access_code_to_account("ACC-0001", "CUST-0001").await;

        // then
        assert!(result.is_ok());
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM account_access_codes WHERE account_id = 'ACC-0001'")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count, 1);

        Ok(())
    }
}
