use strum::IntoStaticStr;

use super::CommandT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u16)]
#[non_exhaustive]
pub enum EcKnownCommand {
    Hello = 0x0001,
    FwChargeLimit = 0x3E03,
}

impl EcKnownCommand {
    pub const fn as_cmd(&self) -> CommandT {
        *self as CommandT
    }
}
