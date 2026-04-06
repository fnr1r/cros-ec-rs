use std::os::fd::AsFd;

use crate::{error::EcCommandError, types::EcCommandInfo};

/// This is an internal trait of `cros-ec`.
///
/// # Safety
///
/// This should only be implemented on unit structs, and only version interfaces
/// specifically.
pub unsafe trait EcDevBackendEmpty {
    const INSTANCE: Self;
}

pub trait EcDevBackendCommand {
    unsafe fn ec_command(
        &self,
        fd: impl AsFd,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError>;
}
