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

//! POST /oidc/token — OAuth 2.0 token endpoint (RFC 6749).
//!
//! Supported grant types:
//!   - `password`                          — email + password (account login)
//!   - `urn:photos.network:access_code`    — single access code (customer login)

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use common::auth::auth_manager::AuthManager;
use serde::{Deserialize, Serialize};
use tracing::error;

use super::authorize::SharedState;

const GRANT_PASSWORD: &str = "password";
const GRANT_ACCESS_CODE: &str = "urn:photos.network:access_code";

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub access_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct TokenErrorResponse {
    pub error: &'static str,
    pub error_description: String,
}

pub(crate) async fn token_endpoint(
    State(state): State<SharedState>,
    Form(req): Form<TokenRequest>,
) -> impl IntoResponse {
    let db = Arc::clone(&state.read().unwrap().db);
    let auth = AuthManager::new(db);

    match req.grant_type.as_str() {
        GRANT_PASSWORD => handle_password_grant(auth, req).await,
        GRANT_ACCESS_CODE => handle_access_code_grant(auth, req).await,
        _ => (
            StatusCode::BAD_REQUEST,
            Json(TokenErrorResponse {
                error: "unsupported_grant_type",
                error_description: format!(
                    "Supported grant types: {GRANT_PASSWORD}, {GRANT_ACCESS_CODE}"
                ),
            }),
        )
            .into_response(),
    }
}

async fn handle_password_grant(auth: AuthManager, req: TokenRequest) -> axum::response::Response {
    let username = match req.username.filter(|s| !s.is_empty()) {
        Some(u) => u,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenErrorResponse {
                    error: "invalid_request",
                    error_description: "username required for password grant".to_string(),
                }),
            )
                .into_response()
        }
    };
    let password = match req.password.filter(|s| !s.is_empty()) {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenErrorResponse {
                    error: "invalid_request",
                    error_description: "password required for password grant".to_string(),
                }),
            )
                .into_response()
        }
    };

    match auth.verify_account_credentials(username, password).await {
        Ok(resp) => {
            let _ = auth.update_last_login_account(resp.account_id).await;
            (
                StatusCode::OK,
                Json(TokenResponse {
                    access_token: resp.jwt_token.unwrap_or_default(),
                    token_type: "Bearer",
                    expires_in: 86400,
                }),
            )
                .into_response()
        }
        Err(e) => {
            error!("password grant failed: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(TokenErrorResponse {
                    error: "invalid_client",
                    error_description: "Invalid email or password".to_string(),
                }),
            )
                .into_response()
        }
    }
}

async fn handle_access_code_grant(
    auth: AuthManager,
    req: TokenRequest,
) -> axum::response::Response {
    let code = match req.access_code.filter(|s| !s.is_empty()) {
        Some(c) => c,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TokenErrorResponse {
                    error: "invalid_request",
                    error_description: "access_code required for this grant type".to_string(),
                }),
            )
                .into_response()
        }
    };

    match auth.verify_access_code(code).await {
        Ok(resp) => {
            let _ = auth.update_last_login(resp.customer_id).await;
            (
                StatusCode::OK,
                Json(TokenResponse {
                    access_token: resp.jwt_token.unwrap_or_default(),
                    token_type: "Bearer",
                    expires_in: 86400,
                }),
            )
                .into_response()
        }
        Err(e) => {
            error!("access_code grant failed: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(TokenErrorResponse {
                    error: "invalid_client",
                    error_description: "Invalid access code".to_string(),
                }),
            )
                .into_response()
        }
    }
}
