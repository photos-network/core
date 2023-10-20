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

//! Returns a specific owned or shared media item for current user
//!

use axum::http::StatusCode;

pub(crate) async fn get_media_id() -> std::result::Result<String, StatusCode> {
    // TODO: parse params  max-with / max-height   =wmax-width-hmax-height  (=w2048-h1024)
    // -wmax-width   (preserving the aspect ratio)
    // -hmax-height  (preserving the aspect ratio)
    // -c  crop images to max-width / max-height
    // -d  remove exif data

    Err(StatusCode::NOT_IMPLEMENTED)
}
