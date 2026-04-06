use std::os::fd::AsFd;

use const_default::ConstDefault;
use rustix::io::Errno;

use super::EcDevError;
use crate::{error::EcCommandError, types::EcCommandInfo};

/// This is an internal trait of `cros-ec`.
///
/// # Safety
///
/// This should only be implemented on unit structs, and only version interfaces
/// specifically.
pub unsafe trait EcDevBackendEmpty: ConstDefault {}

pub trait EcDevBackendCommand {
    unsafe fn ec_command(
        &self,
        fd: impl AsFd,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError>;
}

pub trait EcDevBackendReadmem {
    fn ec_readmem(&self, fd: impl AsFd, offset: i32, output: &mut [u8]) -> Result<usize, Errno>;
}

pub trait EcDevBackendNew: Sized {
    fn ec_dev_new(fd: impl AsFd) -> Result<Self, EcDevError>;
}

#[doc(hidden)]
#[macro_export]
macro_rules! ec_dev_backend_ver_impl {
    ($name:path) => {
        impl ConstDefault for $name {
            const DEFAULT: Self = Self;
        }

        unsafe impl EcDevBackendEmpty for $name {}

        impl EcDevBackendNew for $name {
            fn ec_dev_new(fd: impl AsFd) -> Result<Self, EcDevError> {
                let iface = EcDev::<_, Self>::ver_new_unchecked(fd);
                $crate::cmds::hello::ec_cmd_hello(&iface)?;
                Ok(Self::DEFAULT)
            }
        }
    };
}
