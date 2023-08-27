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

use std::fmt;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    uuid: Uuid,
    email: String,
    password: Option<String>,
    lastname: Option<String>,
    firstname: Option<String>,
    is_locked: bool,
    created_at: OffsetDateTime,
    updated_at: Option<OffsetDateTime>,
    last_login: Option<OffsetDateTime>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}), locked:{})",
            self.email, self.uuid, self.is_locked
        )
    }
}

impl User {
    pub(crate) fn new(email: String) -> User {
        User {
            uuid: Uuid::new_v4(),
            email,
            password: Option::None,
            lastname: Option::None,
            firstname: Option::None,
            is_locked: false,
            created_at: OffsetDateTime::now_utc(),
            updated_at: Option::None,
            last_login: Option::None,
        }
    }
}
