use std::ffi::{c_int, c_ulong};

pub use self::{command::EcKnownCommand, command_info::EcCommandInfo};

mod command;
mod command_info;
pub mod cstring;
pub mod features;
pub mod version_mask;

pub type CommandT = c_int;
pub type VersionT = c_int;
pub type MaskT = c_ulong;
