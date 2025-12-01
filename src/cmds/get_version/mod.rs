use strum::FromRepr;

pub use super::consts::{EC_CMD_GET_VERSION_V0, EC_CMD_GET_VERSION_V1};
use crate::types::cstring::SizedCString;

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
