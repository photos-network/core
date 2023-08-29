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

//! This describes a plugin with a key-value pair configuration
use std::fmt;

use serde::Deserialize;
use serde_json::Map;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub config: Option<Map<String, serde_json::Value>>,
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_deserialization() {
        // given
        let json = r#"{
            "name": "Plugin",
            "config": {
                "property1": null,
                "property2": true,
                "property3": "aBc",
                "property4": 42
            }
        }"#;

        let mut config = Map::new();
        config.insert("property1".to_string(), serde_json::Value::Null);
        config.insert("property2".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "property3".to_string(),
            serde_json::Value::String("aBc".into()),
        );
        config.insert(
            "property4".to_string(),
            serde_json::Value::Number(42.into()),
        );

        let data = Plugin {
            name: "Plugin".into(),
            config: Some(config),
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn test_minimal_deserialization() {
        // given
        let json = r#"{
            "name": "Plugin"
        }"#;

        let data = Plugin {
            name: "Plugin".into(),
            config: None,
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }
}
