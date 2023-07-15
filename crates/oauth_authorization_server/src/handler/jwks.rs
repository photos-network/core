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

use axum::extract::State;
use axum::Json;
use axum::{headers::Host, TypedHeader};
use openidconnect::core::CoreJsonWebKeySet;

use super::authorize::SharedState;

pub(crate) async fn openid_jwks_handler(
    State(state): State<SharedState>,
    TypedHeader(host): TypedHeader<Host>,
) -> Json<CoreJsonWebKeySet> {
    for realm in state.read().unwrap().realms.iter() {
        if realm.domain == host.hostname() {
            return Json(realm.jwks.clone());
        }
    }

    Json(state.read().unwrap().master_realm.jwks.clone())
}
