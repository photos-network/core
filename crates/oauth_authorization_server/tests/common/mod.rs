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

//! Create a private key file to use for OIDC.
//!
use oauth_authorization_server::{config::ServerConfig, state::ServerState, AuthorizationServerManager};
use axum::Router;
use rand::rngs::OsRng;
use rsa::pkcs1::EncodeRsaPrivateKey;
use rsa::pkcs8::LineEnding;
use rsa::RsaPrivateKey;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use testdir::testdir;

pub fn create_fake_pem(filename: &'static str) -> PathBuf {
    let path: PathBuf = testdir!();
    let keys_base_path = path.join("keys");
    // create keys directory
    fs::create_dir(&keys_base_path).unwrap();

    // create a fake private key
    let mut rng = OsRng;
    let bits = 2048;
    let key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate private key");
    let pem = key.to_pkcs1_pem(LineEnding::LF).unwrap();

    // write private key into file
    let mut file: File = File::create(keys_base_path.join(filename)).expect("no file");
    file.write_all(pem.as_bytes()).expect("write failed");

    keys_base_path
}

pub fn create_router() -> Router {
    let private_key: PathBuf = create_fake_pem("master.pem");

    // create server config with fake key
    let server_config = ServerConfig {
        listen_addr: String::from("127.0.0.1:7777"),
        domain: String::from("localhost:7777"),
        use_ssl: false,
        realm_keys_base_path: private_key,
        realms: vec![],
    };
    let server_state = ServerState::new(server_config).expect("no server config!");
    let router = AuthorizationServerManager::routes(server_state);

    router
}
