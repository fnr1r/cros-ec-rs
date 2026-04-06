use std::os::fd::AsFd;

use super::EcDevError;
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

pub trait EcDevBackendNew: Sized {
    fn ec_dev_new(fd: impl AsFd) -> Result<Self, EcDevError>;
}

#[doc(hidden)]
#[macro_export]
macro_rules! ec_dev_backend_new_impl_empty {
    ($name:ident) => {
        impl EcDevBackendNew for $name {
            fn ec_dev_new(fd: impl AsFd) -> Result<Self, EcDevError> {
                let iface = EcDev::new_unchecked(fd, Self::INSTANCE);
                $crate::cmds::hello::ec_cmd_hello(&iface)?;
                Ok(Self::INSTANCE)
            }
        }
    };
}
