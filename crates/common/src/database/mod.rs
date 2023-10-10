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

use async_trait::async_trait;
use std::error::Error;
use time::OffsetDateTime;

use crate::auth::user::User;

use self::{media_item::MediaItem, reference::Reference};

pub mod details;
pub mod location;
pub mod media_item;
pub mod reference;
pub mod tag;

#[async_trait]
pub trait Database {
    /// Initialize the database and run required migrations
    async fn setup(&mut self) -> Result<(), Box<dyn Error>>;

    /// List registered user accounts
    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>>;

    /// Create a new user account
    async fn create_user(&self, user: &User) -> Result<(), Box<dyn Error>>;

    /// Get user by user_id
    async fn get_user(&self, user_id: &str) -> Result<User, Box<dyn Error>>;

    /// Partial update a single user account
    async fn update_email(&self, email: &str, user_id: &str) -> Result<(), Box<dyn Error>>;
    async fn update_nickname(&self, nickname: &str) -> Result<(), Box<dyn Error>>;
    async fn update_names(
        &self,
        firstname: &str,
        lastname: &str,
        user_id: &str,
    ) -> Result<(), Box<dyn Error>>;

    async fn disable_user(&self, user_id: &str) -> Result<(), Box<dyn Error>>;
    async fn enable_user(&self, user_id: &str) -> Result<(), Box<dyn Error>>;

    async fn get_media_items(&self, user_id: &str) -> Result<Vec<MediaItem>, Box<dyn Error>>;
    async fn create_media_item(
        &self,
        user_id: &str,
        name: &str,
        date_taken: OffsetDateTime,
    ) -> Result<String, Box<dyn Error>>;
    async fn get_media_item(&self, media_id: &str) -> Result<MediaItem, Box<dyn Error>>;
    async fn add_reference(
        &self,
        user_id: &str,
        media_id: &str,
        reference: &Reference,
    ) -> Result<String, Box<dyn Error>>;

    async fn update_reference(
        &self,
        reference_id: &str,
        reference: &Reference,
    ) -> Result<(), Box<dyn Error>>;

    async fn remove_reference(
        &self,
        media_id: &str,
        reference_id: &str,
    ) -> Result<(), Box<dyn Error>>;
}
