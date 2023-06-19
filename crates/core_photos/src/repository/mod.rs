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

/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, automock)]
#[async_trait]
trait PhotosRepositoryTrait {
    /// Gets a list of heroes from the DB filted by name
    async fn get_photos_for_user(&self, user_id: &str) -> Result<Vec<Photo>, DataAccessError>;
}

struct PhotosRepository();

#[async_trait]
impl PhotosRepositoryTrait for PhotosRepository {
    async fn get_photos_for_user(&self, user_id: &str) -> Result<Vec<Photo>, DataAccessError> {
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    async fn get_photos_for_user_success(#[case] uri: &'static str, #[case] expected_filter: &'static str);
}
