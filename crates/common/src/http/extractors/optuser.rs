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

//! This extractor checks if an `Authorization` is header and contains a valid JWT token.
//! Otherwise it will respond `Some(None)` to indicate an unauthorized user or a visiter without an account at all.
//!
use crate::model::auth::user::User;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;

pub struct OptionalUser(pub Option<User>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
{
    type Rejection = String;

    #[allow(clippy::bind_instead_of_map)]
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("Authorization")
            .and_then(|header| {
                let _auth_token = header.to_str().ok();

                // TODO: verify auth token
                Some(Self(Some(User::new("info@photos.network".to_string()))))
            })
            .ok_or("".to_string())
    }
}
