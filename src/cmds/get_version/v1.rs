use super::{EC_CMD_GET_VERSION_V1, prelude::*};

/// NOTE: This is untested
#[derive(Debug, Default)]
#[repr(C, align(4))]
pub struct GetVersionResponseV1 {
    pub version_string_ro: VersionStr,
    pub version_string_rw: VersionStr,
    pub cros_fwid_ro: VersionStr,
    pub current_image: u32,
    pub cros_fwid_rw: VersionStr,
}

unsafe impl Plain for GetVersionResponseV1 {}

impl From<GetVersionResponseV1> for EcVersion {
    fn from(value: GetVersionResponseV1) -> Self {
        let GetVersionResponseV1 {
            version_string_ro,
            version_string_rw,
            cros_fwid_ro,
            current_image,
            cros_fwid_rw,
        } = value;
        Self {
            version_string_ro,
            version_string_rw,
            current_image: EcImageType::from_repr(current_image).unwrap_or_default(),
            cros_fwid_ro: Some(cros_fwid_ro),
            cros_fwid_rw: Some(cros_fwid_rw),
        }
    }
}

pub fn ec_cmd_get_version_v1(iface: &impl EcHasCommand) -> Result<GetVersionResponseV1> {
    let mut res = GetVersionResponseV1::default();
    unsafe { iface.ec_command_w(&EC_CMD_GET_VERSION_V1, &mut res)? };
    Ok(res)
}
