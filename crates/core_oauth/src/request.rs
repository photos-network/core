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

use dioxus::prelude::*;
use miette::Diagnostic;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    pub(crate) id: uuid::Uuid,
    pub(crate) code: Option<String>,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) state: String,
    pub(crate) code_challenge: String,
    pub(crate) nonce: String,
}
