use derive_more::Deref;
use plain::Plain;

#[derive(Debug, Clone, Copy, Default, Deref)]
#[repr(transparent)]
pub struct VersionMask(pub u32);

unsafe impl Plain for VersionMask {}
