use std::{
    ffi::CStr,
    fmt::{Debug, Formatter},
};

use bstr::ByteSlice;
use derive_more::Deref;
use plain::Plain;

#[derive(Deref)]
#[repr(transparent)]
pub struct SizedCString<const N: usize>(pub [u8; N]);

impl<const N: usize> AsRef<[u8]> for SizedCString<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> SizedCString<N> {
    fn as_cstr(&self) -> Option<&CStr> {
        CStr::from_bytes_until_nul(self.as_ref()).ok()
    }
}

unsafe impl<const N: usize> Plain for SizedCString<N> {}

impl<const N: usize> Debug for SizedCString<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.as_cstr() {
            Debug::fmt(s, f)
        } else {
            Debug::fmt(self.as_bstr(), f)
        }
    }
}

impl<const N: usize> SizedCString<N> {
    // TODO: Move to const-default
    pub const DEFAULT: Self = Self([0; N]);
}

impl<const N: usize> Default for SizedCString<N> {
    fn default() -> Self {
        Self::DEFAULT
    }
}
