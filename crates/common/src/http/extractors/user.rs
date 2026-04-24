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

//! Extracts the authenticated `User` from a request.
//!
//! Requires an `Authorization: Bearer <jwt>` header. Decodes the JWT,
//! resolves the `sub` claim (account_id) from the database, and returns a User.
//! Responds with `401 Unauthorized` if the header is missing, the token is
//! invalid/expired, or the account cannot be found.
//!
use async_trait::async_trait;
use axum::extract::{Extension, FromRequestParts};
use axum::http::StatusCode;
use http::request::Parts;

use crate::auth::auth_manager::AuthManager;
use crate::auth::user::User;
use crate::database::ArcDynDatabase;

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let raw = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        let token = raw
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        let (sub, _role) = AuthManager::validate_jwt_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        let Extension(db): Extension<ArcDynDatabase> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB unavailable"))?;

        let account = db
            .get_account_by_id(&sub)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        Ok(User {
            uuid: account.account_id,
            email: account.email,
            password: None,
            lastname: None,
            firstname: None,
            is_locked: false,
            is_admin: account.is_admin,
            created_at: account.created_at,
            updated_at: account.updated_at,
            last_login: account.last_login_at,
        })
    }
}
