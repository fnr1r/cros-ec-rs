use plain::as_mut_bytes;

pub use super::consts::EC_CMD_PROTO_VERSION;
use super::prelude::*;

pub type ProtoVersion = u32;

pub fn ec_cmd_proto_version(iface: &impl EcHasCommand) -> Result<ProtoVersion> {
    let mut res = ProtoVersion::default();
    let output = Some(unsafe { as_mut_bytes(&mut res) });
    unsafe { iface.ec_command(&EC_CMD_PROTO_VERSION, None, output)? };
    Ok(res)
}
