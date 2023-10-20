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

//! Test authentication behaviour like invalid user or password
//!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 3;
        let b = 1 + 1;

        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }

    #[tokio::test]
    async fn authenticate_without_user() {
        // given
        let test_state = spawn_app().await;
        let client = test_state.api_client;
        let input = serde_json::json!({"password": "secret"});

        // when
        let response = client
            .post(&format!("{}/oauth/authorize", &test_state.app_address))
            .form(&input)
            .send()
            .await
            .expect("authorization request failed!");

        // then
        assert_eq!(
            response.status().as_u16(),
            422,
            "{} returns client error status",
            "no username"
        );
    }
}
