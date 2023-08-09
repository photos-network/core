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

use core::start_server;
use std::process;


/// the `#[tokio::main]` macro initializes a runtime instance and executes the main in it.
/// See: https://tokio.rs/tokio/tutorial/hello-tokio#async-main-function
#[tokio::main]
async fn main() {
    if let Err(e) = start_server().await {
        eprintln!("error: {:#}", e);
        process::exit(1);
    }
}
