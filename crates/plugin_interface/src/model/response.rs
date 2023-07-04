#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, StableAbi)]
pub struct PluginResponse {
    pub from: PluginId,
    pub to: PluginId,
    pub response: RString,
}
