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

static LOGIN_FORM_TEMPLATE: &str = r#"
<html>
<head>
  <script src="https://cdn.tailwindcss.com?plugins=forms,typography,aspect-ratio,line-clamp"></script>
  <script>
    tailwind.config = {
      theme: {
        extend: {
          colors: {
            clifford: '#da373d',
            accent: '#706CF6',
            error: '#F2BAB9',
            success: '#D2ECAE',
            warn: '#F9F4BC',
            neutral: '#CEEFF4',
          }
        }
      }
    }
  </script>

  <style type="text/tailwindcss">
    @layer utilities {
      .content-auto {
        content-visibility: auto;
      }
    }
  </style>

  <style>
    body {font-family: Arial, Helvetica, sans-serif;}
    form {border: 3px solid #f1f1f1;}
  </style>
</head>
<body>
<form method="post">
    <label for="username" class="leading-7 text-sm text-gray-600"><b>Username</b></label>
    <input type="text" id="username" name="username" class="w-full bg-white rounded border border-gray-300 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-200 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out" />

    <label for="password" class="leading-7 text-sm text-gray-600"><b>Password</b></label>
    <input type="password" id="password" name="password" class="w-full bg-white rounded border border-gray-300 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-200 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out" />
    
    <input type="hidden" id="request_id" name="request_id" value="{{request_id}}" />
    <input type="submit" id="submit" value="Submit" class="text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg" />
</form>
</body>
</html>
"#;

pub(crate) async fn get_realm_login_form(
    axum::extract::Path(realm): axum::extract::Path<String>,
    axum::extract::Query(query): axum::extract::Query<LoginQuery>,
) -> Html<String> {
    // create a VirtualDom with the app component
    // rebuild the VirtualDom before rendering

    tracing::debug!(
        "Rendering form for request_id={} and realm={}",
        query.request_id,
        realm
    );

    // render the VirtualDom to HTML
    // Html(dioxus_ssr::render(&app))
    Html(LOGIN_FORM_TEMPLATE.to_string())
    //Html("<h1>Login</h1><form method='post'><label for=\"username\"><b>Username</b></label` y><input type=\"text\" placeholder=\"Enter Username\" name=\"username\" required>i
    //<label for=\"password\"><b>Password</b></label><input type=\"password\" placeholder=\"Enter Password\" name=\"password\" required><button type=\"submit\">Login</button></form>".to_string())
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
    request_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginFormData {
    username: String,
    password: String,
}
