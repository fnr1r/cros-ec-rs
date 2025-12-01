use std::iter::FusedIterator;

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
    const fn unset_bit(&mut self, n: u8) {
        self.0 &= !(1 << n);
    }
    pub fn max_version(&self) -> Option<u8> {
        self.into_iter().next_back()
    }
}

#[derive(Debug, Clone)]
pub struct VersionIter(VersionMask);

impl From<VersionMask> for VersionIter {
    fn from(value: VersionMask) -> Self {
        Self(value)
    }
}

impl Iterator for VersionIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let version = self.0.trailing_zeros() as u8;
        if version >= VersionMask::BITS {
            return None;
        }
        self.0.unset_bit(version);
        Some(version)
    }
}

impl DoubleEndedIterator for VersionIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let zeroes = self.0.leading_zeros() as u8;
        if zeroes >= VersionMask::BITS {
            return None;
        }
        let version = VersionMask::BITS - 1 - zeroes;
        self.0.unset_bit(version);
        Some(version)
    }
}

impl ExactSizeIterator for VersionIter {
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl FusedIterator for VersionIter {}

impl IntoIterator for VersionMask {
    type Item = u8;
    type IntoIter = VersionIter;
    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
