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

 use crate::sensitive::Sensitive;

//! Login request
//!
//! Provides an abstraction over a vlue for sensitive data like passwords.
//! It is not printing its value to logs or tracing
//! 
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Login {
  pub username_or_email: Sensitive<String>,
  pub password: Sensitive<String>,
  pub totp_2fa_token: Option<String>,
}

//! Login response
//! 
//!  * `jwt` - None if email verification is enabled.
//!  * `verify_email_sent` - Indicates if an email verification is needed.
//!  
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
  pub jwt: Option<Sensitive<String>>,
  pub registration_created: bool,
  pub verify_email_sent: bool,
}
