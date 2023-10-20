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

use axum::response::Html;
use axum::Form;

use serde::Deserialize;

pub(crate) async fn get_realm_login_form(
    axum::extract::Path(_realm): axum::extract::Path<String>,
    // Query(query): Query<LoginQuery>,
) -> Html<String> {
    // create a VirtualDom with the app component
    // rebuild the VirtualDom before rendering

    // tracing::debug!(
    //     "Rendering form for request_id={} and realm={}",
    //     query.request_id,
    //     realm
    // );

    // render the VirtualDom to HTML
    // Html(dioxus_ssr::render(&app))
    Html("Login".to_string())
}

pub(crate) async fn post_realm_login(Form(login_form): Form<LoginFormData>) -> Html<String> {
    tracing::debug!(
        "username: {}, password: {}",
        login_form.username,
        login_form.password
    );

    // TODO: validate credentials

    Html(String::from("<div>Success</div>"))
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginQuery {
    _request_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginFormData {
    username: String,
    password: String,
}
