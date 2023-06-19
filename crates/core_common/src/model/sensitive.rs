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

//! Sensitive.
//!
//! Provides an abstraction over a vlue for sensitive data like passwords.
//! It is not printing its value to logs or tracing
//! 
use serde::{Deserialize, Serialize};
use std::{
  borrow::Borrow,
  ops::{Deref, DerefMut},
};
#[cfg(feature = "full")]
use ts_rs::TS;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct Sensitive<T>(T);

impl<T> Sensitive<T> {
  pub fn new(item: T) -> Self {
    Sensitive(item)
  }
  pub fn into_inner(self) -> T {
    self.0
  }
}

///! overrides the standard debug programmer-facing representation to prevent the value from leaking.
impl<T> std::fmt::Debug for Sensitive<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("[********]").finish()
  }
}

impl<T> AsRef<T> for Sensitive<T> {
  fn as_ref(&self) -> &T {
    &self.0
  }
}

impl AsRef<str> for Sensitive<String> {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl AsRef<[u8]> for Sensitive<String> {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl AsRef<[u8]> for Sensitive<Vec<u8>> {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl<T> AsMut<T> for Sensitive<T> {
  fn as_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl AsMut<str> for Sensitive<String> {
  fn as_mut(&mut self) -> &mut str {
    &mut self.0
  }
}

impl Deref for Sensitive<String> {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Sensitive<String> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> From<T> for Sensitive<T> {
  fn from(t: T) -> Self {
    Sensitive(t)
  }
}

impl From<&str> for Sensitive<String> {
  fn from(s: &str) -> Self {
    Sensitive(s.into())
  }
}

impl<T> Borrow<T> for Sensitive<T> {
  fn borrow(&self) -> &T {
    &self.0
  }
}

impl Borrow<str> for Sensitive<String> {
  fn borrow(&self) -> &str {
    &self.0
  }
}

#[cfg(feature = "full")]
impl TS for Sensitive<String> {
  fn name() -> String {
    "string".to_string()
  }
  fn name_with_type_args(_args: Vec<String>) -> String {
    "string".to_string()
  }
  fn dependencies() -> Vec<ts_rs::Dependency> {
    Vec::new()
  }
  fn transparent() -> bool {
    true
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn debug_representation_should_replace_value() {
    let sensitive_secret: Sensitive<String> = "secret".into();

    assert_eq!("[********]", format!("{:?}", sensitive_secret))
  }


  #[test]
  fn convert_string_into_should_succeed() {
    let sensitive_secret: Sensitive<String> = "secret".into();

    assert_eq!("secret", sensitive_secret.0)
  }
}
