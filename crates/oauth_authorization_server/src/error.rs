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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    OpenIDUrlParseError(#[from] openidconnect::url::ParseError),

    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("{0}")]
    HyperError(String),

    #[error("{0}")]
    MappedError(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    RSAError(#[from] rsa::Error),

    #[error(transparent)]
    Pkcs1Error(#[from] rsa::pkcs1::Error),

    #[error("could not open key of realm {0}")]
    CouldNotOpenRealmKey(String),
}
