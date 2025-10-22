use super::{CommandT, EcKnownCommand, VersionT};

/// Embedded Controller Command info
///
/// Comntains the command number, version and name
#[derive(Debug, Clone)]
pub struct EcCommandInfo {
    pub command: CommandT,
    pub version: VersionT,
}

impl EcCommandInfo {
    pub const fn new(command: CommandT, version: VersionT) -> Self {
        Self { command, version }
    }
    pub const fn new_known(command: EcKnownCommand, version: VersionT) -> Self {
        Self::new(command.as_cmd(), version)
    }
}
