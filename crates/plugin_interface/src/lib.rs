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

//! This is the plugin interface definition for Photos.network
//!
//! The Photos.network core will look for available plugins during start up and
//! enable and load them when the plugin identifier is present in the configuration file.
//!
//! Plugins can not be unloaded during runtime!
//!

use abi_stable::{
    declare_root_module_statics,
    external_types::crossbeam_channel::RSender,
    library::RootModule,
    package_version_strings, sabi_trait,
    sabi_types::VersionStrings,
    std_types::{RBox, RResult, RString},
    StableAbi,
};

pub type PluginType = Plugin_TO<'static, RBox<()>>;
pub type PluginId = RString;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, StableAbi)]
pub struct PluginCommand {
    pub from: RString,
    pub to: RString,
    pub command: RString,
}

/// Interface definition of the plugin
#[sabi_trait]
pub trait Plugin {
    fn on_core_init(&self) -> RResult<RString, Error>;
    fn on_core_started(&self) -> RResult<RString, Error>;
}

/// Factory to load the plugin at runtime
///
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginFactoryRef)))]
#[sabi(missing_field(panic))]
pub struct PluginFactory {
    /// Constructs the plugin.
    #[sabi(last_prefix_field)]
    pub new: extern "C" fn(RSender<PluginCommand>, PluginId) -> RResult<PluginType, Error>,
}

impl RootModule for PluginFactoryRef {
    declare_root_module_statics! {PluginFactoryRef}
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

#[repr(u8)]
#[derive(Debug, StableAbi)]
pub enum Error {
    /// A deserialization error produced when trying to deserialize json
    /// as a particular command type.
    UnsupportedCommand(RBox<RString>),
    /// A deserialization error produced when trying to deserialize json
    /// as a particular return value type.
    UnsupportedReturnValue(RBox<RString>),
}
