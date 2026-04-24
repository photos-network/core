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

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use common::auth::auth_manager::AuthManager;
use common::database::ArcDynDatabase;

#[derive(Debug, Deserialize)]
pub struct AccountRegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub access_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccountLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AccountLoginResponse {
    pub account_id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub jwt_token: Option<String>,
}

pub async fn handle_account_register(
    State(db): State<ArcDynDatabase>,
    Json(request): Json<AccountRegisterRequest>,
) -> impl IntoResponse {
    info!("Account registration attempt for email: {}", request.email);

    if request.email.is_empty() || request.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Email and password are required." })),
        )
            .into_response();
    }

    let auth_manager = AuthManager::new(Arc::clone(&db));
    let display_name = request.display_name.unwrap_or_default();

    match auth_manager
        .create_account(
            request.email.clone(),
            request.password.clone(),
            display_name.clone(),
            request.access_code.clone(),
        )
        .await
    {
        Ok(account_id) => {
            info!("Account successfully registered for email: {}", request.email);
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "account_id": account_id,
                    "email": request.email,
                    "display_name": display_name,
                    "message": "Account registered successfully"
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Account registration failed: {}", e);
            let msg = e.to_string();
            (
                StatusCode::CONFLICT,
                Json(serde_json::json!({ "error": msg })),
            )
                .into_response()
        }
    }
}

pub async fn handle_account_login(
    State(db): State<ArcDynDatabase>,
    Json(request): Json<AccountLoginRequest>,
) -> impl IntoResponse {
    info!("Account login attempt for email: {}", request.email);

    let auth_manager = AuthManager::new(Arc::clone(&db));

    match auth_manager
        .verify_account_credentials(request.email.clone(), request.password.clone())
        .await
    {
        Ok(response) => {
            info!("Account login successful for email: {}", request.email);
            let _ = auth_manager
                .update_last_login_account(response.account_id.clone())
                .await;

            let response_json = AccountLoginResponse {
                account_id: response.account_id,
                email: response.email,
                display_name: response.display_name,
                jwt_token: response.jwt_token,
            };

            (StatusCode::OK, Json(response_json)).into_response()
        }
        Err(e) => {
            error!("Account login failed: {}", e);
            let msg = e.to_string();
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": msg })),
            )
                .into_response()
        }
    }
}
