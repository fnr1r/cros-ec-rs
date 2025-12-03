pub use super::consts::EC_CMD_PROTO_VERSION;
use super::prelude::*;

pub type ProtoVersion = u32;

pub fn ec_cmd_proto_version(iface: &impl EcHasCommand) -> Result<ProtoVersion> {
    unsafe { iface.ec_cmd_ext_wad(&EC_CMD_PROTO_VERSION) }
}
