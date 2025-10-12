use std::ffi::{c_int, c_ulong};

pub type CommandT = c_int;
pub type VersionT = c_int;
pub type MaskT = c_ulong;

/// Embedded Controller Command metadata
///
/// Comntains the command number, version and name
#[derive(Debug, Clone)]
pub struct EcCommandMeta {
    pub command: CommandT,
    pub version: VersionT,
    pub name: &'static str,
}

impl EcCommandMeta {
    pub const fn new(command: CommandT, version: VersionT, name: &'static str) -> Self {
        Self {
            command,
            version,
            name,
        }
    }
}
