//! PWM module
//!
//! Expect only fan control here
//!
//! # TODO
//!
//! - consider using `constrained_int` for fan rpm results

use arbitrary_int::u2;
use enumflags2::BitFlags;
use nonmax::NonMaxU16;
use rustix::io::Errno;

use crate::{
    traits::{EcHasReadmem, EcReadmemExt},
    types::features::{EcFeature, EcProxy},
};

type Result<T, E = Errno> = core::result::Result<T, E>;

type EcFeatures = BitFlags<EcFeature>;

#[derive(Debug)]
pub struct FanPwm;

impl EcProxy for FanPwm {
    fn get_required_features() -> impl Into<EcFeatures> {
        EcFeature::PwmFan
    }
}

pub type ProxyFanRpm<T> = crate::types::features::Proxy<T, FanPwm>;

/// Fan speeds `0x10` - `0x17`
pub const EC_MEMMAP_FAN: u16 = 0x0010;
/// Number of fans at EC_MEMMAP_FAN
pub const EC_FAN_SPEED_ENTRIES: u8 = 4;
/// Entry not present
pub const EC_FAN_SPEED_NOT_PRESENT: u16 = 0xffff;
/// Fan stalled
pub const EC_FAN_SPEED_STALLED: u16 = 0xfffe;

/// NOTE: The value range is `0..=3`
pub type FanIdx = u2;

#[inline]
pub const fn get_fan_offset(idx: FanIdx) -> u16 {
    EC_MEMMAP_FAN + 2 * idx.value() as u16
}

impl<T: EcHasReadmem> ProxyFanRpm<T> {
    #[inline]
    pub fn get_fan_rpm_unchecked(&self, idx: FanIdx) -> Result<u16> {
        self.ec_read_u16(get_fan_offset(idx) as _)
    }
    #[inline]
    pub fn get_fan_rpm(&self, idx: FanIdx) -> Result<Option<NonMaxU16>> {
        self.get_fan_rpm_unchecked(idx).map(NonMaxU16::new)
    }
}

#[inline]
pub fn iter_fans() -> impl Iterator<Item = FanIdx> {
    (0..EC_FAN_SPEED_ENTRIES).map(|e| unsafe { u2::new_unchecked(e) })
}

#[inline]
pub fn get_num_fans(iface: &ProxyFanRpm<impl EcHasReadmem>) -> Result<u8> {
    let read_speed = |idx| iface.get_fan_rpm(idx).transpose();
    let resum = |count, speed: Result<_, _>| speed.map(|_| count + 1);
    iter_fans().map_while(read_speed).try_fold(0, resum)
}
