use super::{EC_CMD_FLASH_INFO_V1, prelude::*, v0::FlashInfoV0};

/// Fields added in the V1 version
#[derive(Debug, Default)]
#[repr(C, align(4))]
pub struct FlashInfoV1 {
    /// Ideal write size in bytes. Writes will be fastest if size is exactly
    /// this and offset is a multiple of this.
    ///
    /// For example, an EC may have a write buffer which can do half-page
    /// operations if data is aligned, and a slower word-at-a-time write mode.
    pub write_ideal_size: u32,
    /// Flags; see `EC_FLASH_INFO_*`
    pub flags: u32,
}

unsafe impl Plain for FlashInfoV1 {}

/// Response to the flash info v1 command.
#[derive(Debug, Default)]
#[repr(C, align(4))]
pub struct FlashInfoV1Res {
    pub v0: FlashInfoV0,
    pub v1: FlashInfoV1,
}

unsafe impl Plain for FlashInfoV1Res {}

pub fn ec_cmd_flash_info_v1(iface: &impl EcHasCommand) -> Result<FlashInfoV1Res> {
    unsafe { iface.ec_cmd_ext_wad(&EC_CMD_FLASH_INFO_V1) }
}
