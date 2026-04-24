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

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

use super::exif_info::ExifInformation;
use super::file::File;
use super::location::Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub uuid: String,
    pub name: String,
    pub date_added: DateTime<Utc>,
    pub date_taken: Option<DateTime<Utc>>,
    pub details: Option<ExifInformation>,
    pub tags: Option<Vec<String>>,
    pub location: Option<Location>,
    pub references: Option<Vec<File>>,
}
