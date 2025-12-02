use derive_more::Deref;
use plain::Plain;

#[derive(Debug, Clone, Copy, Default, Deref)]
#[repr(transparent)]
pub struct VersionMask(pub u32);

unsafe impl Plain for VersionMask {}

impl VersionMask {
    pub const BITS: u8 = u32::BITS as u8;
    pub const fn new(mask: u32) -> Self {
        Self(mask)
    }
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn is_supported(&self, version: u8) -> bool {
        self.0 & 1 << version != 0
    }
}
