use strum::FromRepr;

pub use super::consts::{EC_CMD_GET_VERSION_V0, EC_CMD_GET_VERSION_V1};
use crate::types::cstring::SizedCString;

pub mod v0;

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
}
