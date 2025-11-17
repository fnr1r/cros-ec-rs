use plain::Plain;

pub use super::consts::EC_CMD_GET_FEATURES;
use super::prelude::*;

#[derive(Debug, Default)]
#[repr(C, align(4))]
pub struct GetFeaturesResponse {
    pub flags: [u32; 2],
}

unsafe impl Plain for GetFeaturesResponse {}

pub fn ec_cmd_get_features(iface: &impl EcHasCommand) -> Result<GetFeaturesResponse> {
    let mut buf = GetFeaturesResponse::default();
    unsafe { iface.ec_command_w(&EC_CMD_GET_FEATURES, &mut buf)? };
    Ok(buf)
}
