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
use time::OffsetDateTime;

use crate::auth::user::User;

use self::{media_item::MediaItem, reference::Reference};

pub mod details;
pub mod location;
pub mod media_item;
pub mod reference;
pub mod tag;
pub mod user;

pub type ArcDynDatabase = Arc<dyn Database + Send + Sync>;

#[cfg_attr(any(test, feature = "mock"), mockall::automock)]
#[async_trait]
pub trait Database {
    /// List registered user accounts
    async fn get_users(&self) -> Result<Vec<User>>;

    /// Create a new user account
    async fn create_user(&self, user: &User) -> Result<()>;

    /// Get user by user_id
    async fn get_user(&self, user_id: &str) -> Result<User>;

    /// Partial update a single user account
    async fn update_email(&self, email: &str, user_id: &str) -> Result<()>;
    async fn update_nickname(&self, nickname: &str) -> Result<()>;
    async fn update_names(&self, firstname: &str, lastname: &str, user_id: &str) -> Result<()>;

    async fn disable_user(&self, user_id: &str) -> Result<()>;
    async fn enable_user(&self, user_id: &str) -> Result<()>;

    async fn get_media_items(&self, user_id: &str) -> Result<Vec<MediaItem>>;
    async fn create_media_item(
        &self,
        user_id: &str,
        name: &str,
        date_taken: OffsetDateTime,
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
}
