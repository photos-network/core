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

//! This extractor requires an `Authorization` header present with a valid JWT token.
//! Otherwise it will respond with `StatusCode::UNAUTHORIZED`
//!
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use http::request::Parts;

use crate::model::auth::user::User;

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let _auth_token = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        // TODO: get user for Authtoken
        Ok(User::new("info@photos.network".to_string()))
        // TODO: verify Token
        //        verify_auth_token(auth_header)
        //            .await
        //            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized"))
        //Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}
