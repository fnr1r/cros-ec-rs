//!
//! # Errors
//!
//! Returns [EcResult::InvalidParam](crate::error::EcResult::InvalidParam) for
//! a bad command
pub use super::consts::{EC_CMD_GET_CMD_VERSIONS_V0, EC_CMD_GET_CMD_VERSIONS_V1};
use super::prelude::*;
use crate::types::version_mask::VersionMask;

pub fn ec_cmd_get_cmd_versions_v0(iface: &impl EcHasCommand, command: u8) -> Result<VersionMask> {
    unsafe { iface.ec_cmd_ext_rwad(&EC_CMD_GET_CMD_VERSIONS_V0, &command) }
}

pub fn ec_cmd_get_cmd_versions_v1(iface: &impl EcHasCommand, command: u16) -> Result<VersionMask> {
    unsafe { iface.ec_cmd_ext_rwad(&EC_CMD_GET_CMD_VERSIONS_V1, &command) }
}
