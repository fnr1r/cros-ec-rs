use core::iter::repeat_n;

use plain::{as_bytes, as_mut_bytes};
use unslice_dst::SliceWithHeader;

use super::{EC_CMD_FLASH_INFO_V2, prelude::*};

#[derive(Debug, Clone)]
#[repr(C, align(4))]
pub struct FlashInfoV2Params {
    /// Number of banks to describe
    pub num_banks_desc: u16,
    pub reserved: [u8; 2],
}

unsafe impl Plain for FlashInfoV2Params {}

impl FlashInfoV2Params {
    const DEFAULT: Self = Self {
        num_banks_desc: 1,
        reserved: [0; 2],
    };
}

impl Default for FlashInfoV2Params {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C, align(4))]
pub struct FlashInfoV2Header {
    /// Total flash in the EC
    pub flash_size: u32,
    /// Flags; see [`EcFlashInfoFlags`](super::flags::EcFlashInfoFlags)
    pub flags: u32,
    /// Maximum size to use to send data to write to the EC
    pub write_ideal_size: u32,
    /// Number of banks present in the EC
    pub num_banks_total: u16,
    /// Number of banks described in banks array
    num_banks_desc: u16,
}

unsafe impl Plain for FlashInfoV2Header {}

impl FlashInfoV2Header {
    const DEFAULT: Self = Self {
        flash_size: 0,
        flags: 0,
        write_ideal_size: 0,
        num_banks_total: 0,
        num_banks_desc: 0,
    };
    const fn with_len(len: usize) -> Self {
        Self {
            num_banks_desc: len as u16,
            ..Self::DEFAULT
        }
    }
    pub const fn num_banks_desc_get(&self) -> u16 {
        self.num_banks_desc
    }
    pub const fn num_banks_desc_ref(&self) -> &u16 {
        &self.num_banks_desc
    }
    /// # Safety
    ///
    /// Modifying `num_banks_desc` without reallocating the owning
    /// [`FlashInfoV2`] (return type of [`ec_cmd_flash_info_v2`]) may lead to a
    /// segmentation fault.
    pub const unsafe fn num_banks_desc_mut(&mut self) -> &mut u16 {
        &mut self.num_banks_desc
    }
}

/// fields ending with `_exp` are exponents, as in powers of 2
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct EcFlashBank {
    /// Number of sector is in this bank
    pub count: u16,
    pub size_exp: u8,
    /// Minimal write size for the sectors in this bank
    pub write_size_exp: u8,
    /// Erase size for the sectors in this bank
    pub erase_size_exp: u8,
    /// Size for write protection, usually identical to erase size
    pub protect_size_exp: u8,
    pub reserved: [u8; 2],
}

unsafe impl Plain for EcFlashBank {}

pub type FlashInfoV2 = SliceWithHeader<FlashInfoV2Header, EcFlashBank>;

fn new_flash_info_v2_with_len(len: usize) -> Box<FlashInfoV2> {
    let header = FlashInfoV2Header::with_len(len);
    let items = repeat_n(EcFlashBank::default(), len);
    FlashInfoV2::from_iter(header, items)
}

pub fn ec_cmd_flash_info_v2(
    iface: &impl EcHasCommand,
    params: Option<FlashInfoV2Params>,
) -> Result<Box<FlashInfoV2>> {
    let len = params.as_ref().map(|e| e.num_banks_desc).unwrap_or(1);
    let mut res = new_flash_info_v2_with_len(len as usize);
    let input = params.as_ref().map(|e| unsafe { as_bytes(e) });
    let output = unsafe { as_mut_bytes(res.as_mut()) };
    unsafe { iface.ec_cmd_wrap_into(&EC_CMD_FLASH_INFO_V2, input, output) }?;
    Ok(res)
}
