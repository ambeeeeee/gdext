use crate::{
    builtin::Dictionary,
    classes::{multiplayer_api::RpcMode, multiplayer_peer::TransferMode},
    dict,
};

#[doc(hidden)]
pub struct RpcArgs {
    pub mode: RpcMode,
    pub transfer_mode: TransferMode,
    pub call_local: bool,
    pub transfer_channel: u32,
}

impl RpcArgs {
    /// Returns a [`Dictionary`] populated with the values required for a call to [`Node::rpc_config`].
    pub fn into_dictionary(self) -> Dictionary {
        dict! {
            "mode": self.mode,
            "transfer_mode": self.transfer_mode,
            "call_local": self.call_local,
            "transfer_channel": self.transfer_channel
        }
    }
}

impl Default for RpcArgs {
    fn default() -> Self {
        Self {
            mode: RpcMode::AUTHORITY,
            transfer_mode: TransferMode::UNRELIABLE,
            call_local: false,
            transfer_channel: 0,
        }
    }
}
