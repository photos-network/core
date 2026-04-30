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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    pub uuid: String,
    pub camera_manufacturer: String,
    pub camera_model: String,
    pub camera_serial: String,
    pub lens_model: String,
    pub lens_serial: String,
    pub orientation: String,
    pub compression: String,
    pub resolution_x: String,
    pub resolution_y: String,
    pub resolution_unit: String,
    pub exposure_time: String,
    pub exposure_mode: String,
    pub exposure_program: String,
    pub exposure_bias: String,
    pub aperture: f32,
    pub iso: i32,
    pub color_space: String,
    pub pixel_x: i64,
    pub pixel_y: i64,
    pub user_comment: String,
    pub white_balance: String,
    pub flash: bool,
    pub exif_version: f32,
}
