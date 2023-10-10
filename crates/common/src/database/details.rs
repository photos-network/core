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

use uuid::Uuid;

pub struct Details {
    pub uuid: &'static Uuid,
    pub camera_manufacturer: &'static str,
    pub camera_model: &'static str,
    pub camera_serial: &'static str,
    pub lens_model: &'static str,
    pub lens_serial: &'static str,
    pub orientation: &'static str,
    pub compression: &'static str,
    pub resolution_x: &'static str,
    pub resolution_y: &'static str,
    pub resolution_unit: &'static str,
    pub exposure_time: &'static str,
    pub exposure_mode: &'static str,
    pub exposure_program: &'static str,
    pub exposure_bias: &'static str,
    pub aperture: &'static f32,
    pub iso: &'static i32,
    pub color_space: &'static str,
    pub pixel_x: &'static i64,
    pub pixel_y: &'static i64,
    pub user_comment: &'static str,
    pub white_balance: &'static str,
    pub flash: bool,
    pub exif_version: &'static f32,
}
