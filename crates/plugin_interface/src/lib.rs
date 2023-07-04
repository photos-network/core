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
    sabi_types::{VersionStrings},
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
#[sabi(kind(Prefix(prefix_ref = PluginFactory_Ref)))]
#[sabi(missing_field(panic))]
pub struct PluginFactory {
    /// Constructs the plugin.
    #[sabi(last_prefix_field)]
    pub new: extern "C" fn(RSender<PluginCommand>, PluginId) -> RResult<PluginType, Error>,
}

impl RootModule for PluginFactory_Ref {
    declare_root_module_statics! {PluginFactory_Ref}
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
