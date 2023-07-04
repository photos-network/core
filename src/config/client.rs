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

//! This represents an oauth client configuration
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct OAuthClientConfig {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

impl fmt::Display for OAuthClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, redirect: {:?}", self.name, self.redirect_uris)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_deserialization() {
        // given
        let json = r#"{
            "name": "Client",
            "client_id": "clientId",
            "client_secret": "clientSecret",
            "redirect_uris": [
                "https://demo.photos.network/callback",
                "http://127.0.0.1:7777/callback",
                "photosapp://authenticate"
            ]
        }"#;

        let data = OAuthClientConfig {
            name: "Client".into(),
            client_id: "clientId".into(),
            client_secret: "clientSecret".into(),
            redirect_uris: vec![
                "https://demo.photos.network/callback".into(),
                "http://127.0.0.1:7777/callback".into(),
                "photosapp://authenticate".into(),
            ],
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn test_minimal_deserialization() {
        // given
        let json = r#"{
            "name": "Client",
            "client_id": "clientId",
            "client_secret": "clientSecret",
            "redirect_uris": []
        }"#;

        let data = OAuthClientConfig {
            name: "Client".into(),
            client_id: "clientId".into(),
            client_secret: "clientSecret".into(),
            redirect_uris: vec![],
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }
}
