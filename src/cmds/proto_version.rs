pub use super::consts::EC_CMD_PROTO_VERSION;
use super::prelude::*;

pub type ProtoVersion = u32;

pub fn ec_cmd_proto_version(iface: &impl EcHasCommand) -> Result<ProtoVersion> {
    let mut res = ProtoVersion::default();
    unsafe { iface.ec_command_w(&EC_CMD_PROTO_VERSION, &mut res)? };
    Ok(res)
}
