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

//! This represents a database configuration
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,
    pub url: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum DatabaseDriver {
    MySQL,
    PostgresSQL,
    SQLite,
}

impl fmt::Display for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} database; URL: {}", self.driver, self.url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mysql_deserialization() {
        // given
        let json = r#"{
            "driver": "MySQL",
            "url": "protocol://username:password@host/database"
        }"#;

        let data = DatabaseConfig {
            driver: DatabaseDriver::MySQL,
            url: "protocol://username:password@host/database".into(),
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn test_postgres_deserialization() {
        // given
        let json = r#"{
            "driver": "MySQL",
            "url": "protocol://username:password@host/database"
        }"#;

        let data = DatabaseConfig {
            driver: DatabaseDriver::MySQL,
            url: "protocol://username:password@host/database".into(),
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn test_sqlite_deserialization() {
        // given
        let json = r#"{
            "driver": "SQLite",
            "url": "protocol://username:password@host/database"
        }"#;

        let data = DatabaseConfig {
            driver: DatabaseDriver::SQLite,
            url: "protocol://username:password@host/database".into(),
        };

        assert_eq!(data, serde_json::from_str(json).unwrap());
    }
}
