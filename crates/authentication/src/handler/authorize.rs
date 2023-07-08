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

//! Validate the request to ensure that all required parameters are present and valid.
//!
//! See Section 4.1.1: https://tools.ietf.org/html/rfc6749#section-4.1.1
//!
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use std::sync::{Arc, RwLock};

use crate::query::AuthorizeQuery;
use crate::request::AuthRequest;
use crate::state::ServerState;

pub(crate) type SharedState = Arc<RwLock<ServerState>>;

pub(crate) async fn authorization_handler(
    Query(query): Query<AuthorizeQuery>,
    State(state): State<SharedState>,
) -> std::result::Result<Redirect, StatusCode> {
    let req = AuthRequest {
        id: uuid::Uuid::new_v4(),
        code_challenge: query.code_challenge,
        code: None,
        created_at: chrono::Utc::now().naive_utc(),
        state: query.state,
        nonce: query.nonce,
    };
    for realm in state.write().unwrap().realms.iter_mut() {
        for client in realm.clients.iter() {
            if &client.id == &query.client_id {
                realm.requests.push(req);
                let realm_login_url = format!("/{}/login", &realm.name);
                return Ok(Redirect::to(&realm_login_url));
            }
        }
    }
    
    for client in state.read().unwrap().master_realm.clients.iter() {
        if &client.id == &query.client_id {
            state.write().unwrap().master_realm.requests.push(req);
            let realm_login_url = format!("/{}/login", &state.read().unwrap().master_realm.name);
            return Ok(Redirect::to(&realm_login_url));
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}
