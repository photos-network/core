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

use anyhow::Result;
use async_trait::async_trait;
use sqlx::types::chrono::DateTime;

use sqlx::types::chrono::Utc;

use crate::auth::account::Account;
use crate::auth::account_with_albums::AccountWithAlbums;
use crate::auth::album_account::AlbumAccountEntry;
use crate::auth::customer::Customer;

use self::{album::Album, media_item::MediaItem, reference::Reference};
use crate::database::album_stats::AlbumStats;

pub mod album;
pub mod album_stats;
pub mod details;
pub mod location;
pub mod media_item;
pub mod reference;
pub mod tag;

pub type ArcDynDatabase = Arc<dyn Database + Send + Sync>;

#[async_trait]
pub trait Database {
    async fn get_media_items(&self, user_id: &str) -> Result<Vec<MediaItem>>;
    async fn delete_media_item(&self, media_id: &str) -> Result<()>;
    async fn create_media_item(
        &self,
        user_id: &str,
        name: &str,
        date_taken: DateTime<Utc>,
    ) -> Result<String>;
    async fn get_media_item(&self, media_id: &str) -> Result<MediaItem>;
    async fn add_reference(
        &self,
        user_id: &str,
        media_id: &str,
        reference: &Reference,
    ) -> Result<String>;

    async fn update_reference(&self, reference_id: &str, reference: &Reference) -> Result<()>;

    async fn remove_reference(&self, media_id: &str, reference_id: &str) -> Result<()>;

    ///// Customer operations /////

    async fn get_customer(&self, customer_id: &str) -> Result<Customer>;

    async fn create_customer(
        &self,
        customer_id: String,
        access_code: String,
        display_name: String,
    ) -> Result<()>;

    async fn get_customer_by_access_code(&self, code: &str) -> Result<Customer>;

    async fn update_last_login_for_customer(&self, customer_id: &str) -> Result<()>;

    ///// Customer management (photographer operations) /////

    async fn list_customers(&self) -> Result<Vec<Customer>>;

    async fn delete_customer(&self, customer_id: &str) -> Result<()>;

    ///// Account operations /////

    async fn create_account(
        &self,
        account_id: String,
        email: String,
        password_hash: String,
        display_name: Option<String>,
    ) -> Result<()>;

    async fn get_account_by_email(&self, email: &str) -> Result<Account>;

    async fn get_account_by_id(&self, account_id: &str) -> Result<Account>;

    async fn link_access_code_to_account(&self, account_id: &str, customer_id: &str) -> Result<()>;

    async fn get_customer_ids_for_account(&self, account_id: &str) -> Result<Vec<String>>;

    async fn get_albums_for_account(&self, account_id: &str) -> Result<Vec<Album>>;

    async fn update_last_login_for_account(&self, account_id: &str) -> Result<()>;

    async fn is_account_admin(&self, account_id: &str) -> Result<bool, sqlx::Error>;
    async fn set_account_admin(&self, account_id: &str, is_admin: bool) -> Result<(), sqlx::Error>;
    async fn grant_album_to_account(&self, account_id: &str, album_id: &str, role: &str) -> Result<(), sqlx::Error>;
    async fn revoke_album_from_account(&self, account_id: &str, album_id: &str) -> Result<(), sqlx::Error>;
    async fn get_album_account_role(&self, account_id: &str, album_id: &str) -> Result<Option<String>, sqlx::Error>;
    async fn list_accounts_for_album(&self, album_id: &str) -> Result<Vec<AlbumAccountEntry>, sqlx::Error>;
    async fn get_albums_owned_by_account(&self, account_id: &str) -> Result<Vec<Album>, sqlx::Error>;
    async fn list_all_accounts(&self) -> Result<Vec<Account>, sqlx::Error>;
    async fn list_all_albums(&self) -> Result<Vec<Album>, sqlx::Error>;
    async fn get_accounts_with_albums(&self) -> Result<Vec<AccountWithAlbums>, sqlx::Error>;

    ///// Album operations /////

    async fn create_album(
        &self,
        owner_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<String>;

    async fn get_albums_for_user(&self, owner_id: &str) -> Result<Vec<Album>>;

    async fn get_album(&self, album_id: &str) -> Result<Album>;

    async fn update_album(
        &self,
        album_id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<()>;

    async fn delete_album(&self, album_id: &str) -> Result<()>;

    ///// Album-media junction /////

    async fn add_media_to_album(&self, album_id: &str, media_id: &str) -> Result<()>;

    async fn remove_media_from_album(&self, album_id: &str, media_id: &str) -> Result<()>;

    async fn get_media_for_album(&self, album_id: &str) -> Result<Vec<MediaItem>>;

    ///// Customer-album assignment /////

    async fn assign_album_to_customer(
        &self,
        album_id: &str,
        customer_id: &str,
    ) -> Result<()>;

    async fn unassign_album_from_customer(
        &self,
        album_id: &str,
        customer_id: &str,
    ) -> Result<()>;

    async fn get_albums_for_customer(&self, customer_id: &str) -> Result<Vec<Album>>;

    ///// Per-customer item selection /////

    /// Overrides which items a customer can see in an album.
    /// Passing an empty slice clears the selection (customer sees all items).
    async fn set_customer_album_items(
        &self,
        customer_id: &str,
        album_id: &str,
        media_ids: &[&str],
    ) -> Result<()>;

    /// Returns the manually selected item IDs for a customer+album pair.
    /// An empty Vec means the customer sees all items in the album.
    async fn get_customer_album_items(
        &self,
        customer_id: &str,
        album_id: &str,
    ) -> Result<Vec<String>>;

    /// Returns the (filepath, filename) of the first reference for a media item.
    async fn get_media_file_path(&self, media_id: &str) -> Result<Option<(String, String)>>;

    ///// Stats /////

    async fn record_album_view(&self, album_id: &str, viewer_id: &str, viewer_role: &str) -> Result<()>;
    async fn record_media_download(&self, media_id: &str, album_id: Option<&str>, downloader_id: &str, downloader_role: &str) -> Result<()>;
    async fn get_album_stats(&self, album_id: &str) -> Result<AlbumStats>;
    async fn get_stats_for_owned_albums(&self, account_id: &str) -> Result<Vec<AlbumStats>>;

    /// Returns access codes with display names for all customers assigned to the album.
    async fn get_access_codes_for_album(&self, album_id: &str) -> Result<Vec<AlbumCodeEntry>>;

    /// Creates a new customer with a generated access code and assigns them to the album.
    /// Returns the generated access code.
    async fn generate_and_assign_code(&self, album_id: &str, display_name: &str) -> Result<String>;

    /// Creates a customer with a generated access code, NOT assigned to any album.
    /// Returns the generated access_code string.
    async fn generate_code(&self, display_name: &str) -> Result<String>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlbumCodeEntry {
    pub access_code: String,
    pub display_name: Option<String>,
    pub customer_id: String,
}
