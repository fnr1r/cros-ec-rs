use strum::{EnumString, FromRepr, IntoStaticStr, VariantArray};

use super::CommandT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, FromRepr, IntoStaticStr, VariantArray)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u16)]
#[non_exhaustive]
pub enum EcKnownCommand {
    ProtoVersion,
    Hello,
    GetVersion,
    GetCmdVersions = 0x0008,
    GetFeatures = 0x000D,
    FlashInfo = 0x0010,
    PwmGetFanTargetRpm = 0x0020,
    PwmSetFanTargetRpm = 0x0021,
    /// OBSOLETE - Use [`PwmGetDuty`](Self::PwmGetDuty)
    PwmGetKeyboardBacklight = 0x0022,
    /// OBSOLETE - Use [`PwmSetDuty`](Self::PwmSetDuty)
    PwmSetKeyboardBacklight = 0x0023,
    PwmSetFanDuty = 0x0024,
    PwmSetDuty = 0x0025,
    PwmGetDuty = 0x0026,
    ThermalAutoFanCtrl = 0x0052,
    FwChargeLimit = 0x3E03,
}

impl EcKnownCommand {
    pub const fn as_cmd(&self) -> CommandT {
        *self as CommandT
    }
}
