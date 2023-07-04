#[repr(C)]
#[derive(Debug, Clone, PartialEq, StableAbi)]
pub struct PluginCommand {
    pub from: PluginId,
    pub to: PluginId,
    pub command: RString,
}
