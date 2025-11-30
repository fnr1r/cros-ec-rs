use std::{
    fs::{File, OpenOptions},
    io::Error as IoError,
    os::fd::AsFd,
    path::Path,
};

use derive_more::Deref;
use thiserror::Error;

use self::{
    EcDevVersion::*,
    chk_v1::ec_dev_is_v1,
    version::{DevVersionError, ec_dev_read_version_check},
};
use crate::{
    cmds::hello::{EcHelloError, ec_cmd_hello},
    consts::CROS_EC_DEV_PATH,
    error::EcCommandError,
    traits::EcHasCommand,
    types::EcCommandInfo,
};

mod chk_v1;
pub mod v1;
pub mod v2;
mod version;

type Result<T, E = EcDevError> = core::result::Result<T, E>;

#[derive(Debug, Clone, Copy)]
pub enum EcDevVersion {
    V1,
    V2,
}

#[derive(Debug, Error)]
pub enum EcDevError {
    #[error("failed to open: {}", _0)]
    Open(IoError),
    #[error(transparent)]
    Version(#[from] DevVersionError),
    #[error("handshake failed")]
    Handshake(#[from] EcHelloError),
    #[error(transparent)]
    Command(EcCommandError),
}

/// Embedded Controller Device Interface
///
/// Handles connections with the Chromium Embedded Controller via the `/dev`
/// ioctl interface. Supports both V1 and V2.
#[derive(Debug, Deref)]
pub struct EcDev<F: AsFd = File> {
    #[deref]
    file: F,
    version: EcDevVersion,
}

impl<F: AsFd> EcDev<F> {
    /// Creates an [EcDev] without sending a [hello](ec_cmd_hello) command
    ///
    /// Not marked as `unsafe` since any interface should error
    pub const fn new_unchecked(file: F, version: EcDevVersion) -> Self {
        Self { file, version }
    }
    pub fn new(file: F, version: EcDevVersion) -> Result<Self, EcHelloError> {
        let this = Self::new_unchecked(file, version);
        ec_cmd_hello(&this)?;
        Ok(this)
    }
}

impl EcDev {
    fn _open(path: impl AsRef<Path>) -> Result<File, IoError> {
        OpenOptions::new().read(true).write(true).open(path)
    }
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut file = Self::_open(path).map_err(EcDevError::Open)?;
        ec_dev_read_version_check(&mut file, path)?;
        let version = if ec_dev_is_v1(&file)? { V1 } else { V2 };
        Ok(Self::new_unchecked(file, version))
    }
    pub fn open_by_name(name: impl AsRef<str>) -> Result<Self> {
        Self::open(Path::new("/dev").join(name.as_ref()))
    }
    /// Creates a [EcDev] from [CROS_EC_DEV_PATH]
    pub fn open_cros_ec() -> Result<Self> {
        Self::open(CROS_EC_DEV_PATH)
    }
}

impl<F: AsFd> EcHasCommand for EcDev<F> {
    unsafe fn ec_command(
        &self,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError> {
        let fd = &self.file;
        let f = match self.version {
            V1 => v1::command::ec_command_dev_v1,
            V2 => v2::command::ec_command_dev_v2,
        };
        unsafe { f(fd, command, input, output) }
    }
}
