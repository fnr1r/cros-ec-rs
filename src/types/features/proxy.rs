use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    marker::PhantomData,
    mem::transmute,
};

use derive_more::Deref;
use rustix::io::Errno;

use super::{EcFeatures, ec_features_from_u32s};
use crate::{
    cmds::get_features::{GetFeaturesResponse, ec_cmd_get_features},
    error::EcCommandError,
    traits::{EcHasCommand, EcHasReadmem},
    types::EcCommandInfo,
};

type Result<T, E = EcCommandError> = core::result::Result<T, E>;

const fn ec_features_from_resp(v: GetFeaturesResponse) -> EcFeatures {
    ec_features_from_u32s(v.flags)
}

pub trait EcProxy {
    fn get_required_features() -> impl Into<EcFeatures>;
}

/// Wrapper that asserts that the EC has features required by `P`
#[derive(Deref)]
#[repr(transparent)]
pub struct Proxy<T, P: EcProxy>(#[deref] T, PhantomData<P>);

impl<T, P: EcProxy> Proxy<T, P> {
    /// # Safety
    ///
    /// If the EC doesn't support said features, wild things may happen.
    pub const unsafe fn new_unchecked(iface: &T) -> &Self {
        // SAFETY: Self is repr(transparent)
        unsafe { transmute::<&T, &Self>(iface) }
    }
    pub fn new(iface: &T) -> Result<Option<&Self>>
    where
        T: EcHasCommand,
    {
        let features = ec_features_from_resp(ec_cmd_get_features(iface)?);
        Ok(if features.contains(P::get_required_features()) {
            Some(unsafe { Self::new_unchecked(iface) })
        } else {
            None
        })
    }
}

impl<T: Debug, P: EcProxy> Debug for Proxy<T, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Proxy")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<T: EcHasCommand, P: EcProxy> EcHasCommand for Proxy<T, P> {
    unsafe fn ec_command(
        &self,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize> {
        unsafe { self.0.ec_command(command, input, output) }
    }
}

impl<T: EcHasReadmem, P: EcProxy> EcHasReadmem for Proxy<T, P> {
    fn ec_readmem(&self, offset: i32, output: &mut [u8]) -> Result<usize, Errno> {
        self.0.ec_readmem(offset, output)
    }
}
