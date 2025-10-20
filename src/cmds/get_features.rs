use plain::{Plain, as_mut_bytes};

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
    let output = unsafe { as_mut_bytes(&mut buf) };
    unsafe { iface.ec_command(&EC_CMD_GET_FEATURES, None, Some(output))? };
    Ok(buf)
}
