use plain::Plain;

use super::{super::prelude::*, EC_CMD_GET_VERSION_V0, EcImageType, EcVersion, VersionStr};

#[derive(Debug, Default)]
#[repr(C, align(4))]
pub struct GetVersionResponseV0 {
    pub version_string_ro: VersionStr,
    pub version_string_rw: VersionStr,
    pub reserved: [u8; 32],
    pub current_image: u32,
}

unsafe impl Plain for GetVersionResponseV0 {}

impl From<GetVersionResponseV0> for EcVersion {
    fn from(value: GetVersionResponseV0) -> Self {
        let GetVersionResponseV0 {
            version_string_ro,
            version_string_rw,
            reserved: _,
            current_image,
        } = value;
        Self {
            version_string_ro,
            version_string_rw,
            current_image: EcImageType::from_repr(current_image).unwrap_or_default(),
        }
    }
}

pub fn ec_cmd_get_version_v0(iface: &impl EcHasCommand) -> Result<GetVersionResponseV0> {
    let mut res = GetVersionResponseV0::default();
    unsafe { iface.ec_command_w(&EC_CMD_GET_VERSION_V0, &mut res)? };
    Ok(res)
}
