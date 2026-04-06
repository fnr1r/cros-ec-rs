use std::{
    fs::{File, OpenOptions},
    io::Error as IoError,
    os::fd::AsFd,
    path::Path,
};

use derive_more::Deref;

use super::{
    EcDevVersion::*,
    dynamic::Dynamic,
    error::EcDevError,
    traits::{EcDevBackendCommand, EcDevBackendEmpty},
    v1::ec_dev_is_v1,
    version::ec_dev_read_version_check,
};
use crate::{
    cmds::hello::{EcHelloError, ec_cmd_hello},
    consts::CROS_EC_DEV_PATH,
    error::EcCommandError,
    traits::EcHasCommand,
    types::EcCommandInfo,
};

type Result<T, E = EcDevError> = core::result::Result<T, E>;

/// Embedded Controller Device Interface
///
/// Handles connections with the Chromium Embedded Controller via the `/dev`
/// ioctl interface. Supports both V1 and V2.
#[derive(Debug, Deref)]
pub struct EcDev<F: AsFd = File, I = Dynamic> {
    #[deref]
    file: F,
    iface: I,
}

impl<F: AsFd, I> EcDev<F, I> {
    /// Creates an [`EcDev`] without sending a [`hello`](ec_cmd_hello) command
    ///
    /// Not marked as `unsafe` since any interface should error
    pub const fn new_unchecked(file: F, iface: I) -> Self {
        Self { file, iface }
    }
    pub fn into_file(self) -> F {
        self.file
    }
}

impl<F: AsFd, I: EcDevBackendEmpty> EcDev<F, I> {
    pub const fn ver_new_unchecked(file: F) -> Self {
        Self::new_unchecked(file, I::INSTANCE)
    }
    /// Create a new [`EcDev`] with a known version
    pub fn ver_new(file: F) -> Result<Self, EcHelloError>
    where
        I: EcDevBackendCommand + EcDevBackendEmpty,
    {
        let this = Self::ver_new_unchecked(file);
        ec_cmd_hello(&this)?;
        Ok(this)
    }
}

impl<F: AsFd> EcDev<F> {
    pub fn dyn_new(file: F) -> Result<Self> {
        let version = if ec_dev_is_v1(&file)? { V1 } else { V2 };
        Ok(Self::new_unchecked(file, version.into()))
    }
}

fn rwopen(path: impl AsRef<Path>) -> Result<File, IoError> {
    OpenOptions::new().read(true).write(true).open(path)
}

impl EcDev {
    pub fn dyn_open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut file = rwopen(path).map_err(EcDevError::Open)?;
        ec_dev_read_version_check(&mut file, path)?;
        Self::dyn_new(file)
    }
    pub fn open_by_name(name: impl AsRef<str>) -> Result<Self> {
        Self::dyn_open(Path::new("/dev").join(name.as_ref()))
    }
    /// Creates a [`EcDev`] from [`CROS_EC_DEV_PATH`]
    pub fn open_cros_ec() -> Result<Self> {
        Self::dyn_open(CROS_EC_DEV_PATH)
    }
}

impl<F: AsFd, I: EcDevBackendCommand> EcHasCommand for EcDev<F, I> {
    unsafe fn ec_command(
        &self,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError> {
        unsafe { self.iface.ec_command(&self.file, command, input, output) }
    }
}
