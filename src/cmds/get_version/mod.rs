use strum::FromRepr;

pub use super::consts::{EC_CMD_GET_VERSION_V0, EC_CMD_GET_VERSION_V1};
use super::prelude::*;
use crate::{
    cmds::get_cmd_versions::ec_cmd_get_cmd_versions_v0,
    error::{EcError, EcResult},
    types::{EcKnownCommand, cstring::SizedCString},
};

mod prelude;
pub mod v0;
pub mod v1;

pub type VersionStr = SizedCString<32>;

#[derive(Debug, Default, FromRepr)]
#[repr(u32)]
pub enum EcImageType {
    #[default]
    Unknown,
    ReadOnlyA,
    ReadWriteA,
    ReadOnlyB,
    ReadWriteB,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct EcVersion {
    pub version_string_ro: VersionStr,
    pub version_string_rw: VersionStr,
    pub current_image: EcImageType,
    pub cros_fwid_ro: Option<VersionStr>,
    pub cros_fwid_rw: Option<VersionStr>,
}

impl EcVersion {
    // TODO: Move to const-default
    const DEFAULT: Self = Self {
        version_string_ro: VersionStr::DEFAULT,
        version_string_rw: VersionStr::DEFAULT,
        current_image: EcImageType::Unknown,
        cros_fwid_ro: None,
        cros_fwid_rw: None,
    };
}

const GET_VERSION_UNSUPPORTED_ERROR: EcError =
    EcError::err_from_ec_result(EcResult::InvalidCommand);

pub fn ec_cmd_get_version(iface: &impl EcHasCommand) -> Result<EcVersion> {
    let version_mask = ec_cmd_get_cmd_versions_v0(iface, EcKnownCommand::GetVersion as u8)?;
    let Some(version) = version_mask.max_version() else {
        return Err(GET_VERSION_UNSUPPORTED_ERROR)?;
    };
    Ok(match version {
        0 => v0::ec_cmd_get_version_v0(iface)?.into(),
        _ => v1::ec_cmd_get_version_v1(iface)?.into(),
    })
}
