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

//! Thest the OIDC authorization code flow with PKCE
//!
//! 1st - [OpenID Connect Discovery](https://server.com/.well-known/openid-configuration)
//! 2nd - [Authorization](https://server.com/oidc/authorize?)
//! 3rd - [Token](https://server.com/oidc/token)
//! 4th - [User info](https://server.com/oidc/userinfo)
//! 5th - [End session](https://server.com/oidc/logout)
//! 6th - [Revokation](https://server.com/oidc/revoke)
//!
use ::axum_test::TestServer;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    // Test if OIDC discovery responds with 200 OK and contains mandatory fields
    // e.g. issuer, authorization_endpoint and token_endpoint
    //#[tokio::test]
    #[allow(dead_code)]
    async fn oidc_discovery_succesful() {
        // given
        let router = common::create_router();
        let server = TestServer::new(router.into_make_service()).unwrap();

        // when
        let response = server.get(".well-known/openid-configuration").await;

        // then
        assert_eq!(
            response.status_code().as_u16(),
            200,
            "HTTP 200 OK success status response code expected"
        );

        // TODO: verify body
    }

    //#[tokio::test]
    #[allow(dead_code)]
    async fn oidc_authorization_code_flow_with_pkce_succesful() {
        // given
        let router = common::create_router();
        let server = TestServer::new(router.into_make_service()).unwrap();

        // when
        let response = server.get(".well-known/openid-configuration").await;

        // then
        assert_eq!(
            response.status_code().as_u16(),
            200,
            "HTTP 200 OK success status response code expected"
        );

        // TODO: verify Location header
    }
}
