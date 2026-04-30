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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewerEntry {
    pub viewer_id: String,
    pub viewer_role: String,
    pub view_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumStats {
    pub album_id: String,
    pub total_views: i64,
    pub unique_viewers: i64,
    pub total_downloads: i64,
    pub viewers: Vec<ViewerEntry>,
}
