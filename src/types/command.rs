use strum::{EnumString, FromRepr, IntoStaticStr, VariantArray};

use super::CommandT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, FromRepr, IntoStaticStr, VariantArray)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u16)]
#[non_exhaustive]
pub enum EcKnownCommand {
    ProtoVersion,
    Hello,
    GetCmdVersions = 0x0008,
    GetFeatures = 0x000D,
    FwChargeLimit = 0x3E03,
}

impl EcKnownCommand {
    pub const fn as_cmd(&self) -> CommandT {
        *self as CommandT
    }
}
