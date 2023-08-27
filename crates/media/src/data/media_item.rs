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

use std::time::Instant;

use super::exif_info::ExifInformation;
use super::file::File;
use super::location::Location;

pub struct MediaItem {
    pub uuid: &'static str,
    pub name: &'static str,
    pub date_added: Instant,
    pub date_taken: Option<Instant>,
    pub details: Option<ExifInformation>,
    pub tags: Option<Vec<String>>,
    pub location: Option<Location>,
    pub references: Option<Vec<File>>,
}

impl MediaItem {
    #[allow(dead_code)]
    fn new(name: &'static str) -> Self {
        MediaItem {
            uuid: "",
            name,
            date_added: Instant::now(),
            date_taken: None,
            location: None,
            details: None,
            tags: None,
            references: None,
        }
    }
}
