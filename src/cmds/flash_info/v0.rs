use super::{EC_CMD_FLASH_INFO_V0, prelude::*};

/// Response to the flash info command. (Version 0)
#[derive(Debug, Default)]
#[repr(C, align(4))]
pub struct FlashInfoV0 {
    /// Usable flash size in bytes.
    pub flash_size: u32,
    /// Write block size. Write offset and size must be a multiple of this.
    pub write_block_size: u32,
    /// Erase block size. Erase offset and size must be a multiple of this.
    pub erase_block_size: u32,
    /// Protection block size. Protection offset and size must be a multiple of
    /// this.
    pub protect_block_size: u32,
}

unsafe impl Plain for FlashInfoV0 {}

pub fn ec_cmd_flash_info_v0(iface: &impl EcHasCommand) -> Result<FlashInfoV0> {
    unsafe { iface.ec_cmd_ext_wad(&EC_CMD_FLASH_INFO_V0) }
}
