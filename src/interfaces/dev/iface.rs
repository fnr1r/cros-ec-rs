use std::{
    fs::{File, OpenOptions},
    io::Error as IoError,
    os::fd::AsFd,
    path::Path,
};

use derive_more::Deref;
use rustix::io::Errno;

use super::{dynamic::Dynamic, error::EcDevError, traits::*, version::ec_dev_read_version_check};
use crate::{
    cmds::hello::{EcHelloError, ec_cmd_hello},
    consts::CROS_EC_DEV_PATH,
    error::EcCommandError,
    traits::{EcHasCommand, EcHasReadmem},
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

impl<F: AsFd, I: EcDevBackendNew> EcDev<F, I> {
    pub fn new(file: F) -> Result<Self> {
        let iface = I::ec_dev_new(&file)?;
        Ok(Self::new_unchecked(file, iface))
    }
}

fn rwopen(path: impl AsRef<Path>) -> Result<File, IoError> {
    OpenOptions::new().read(true).write(true).open(path)
}

impl<I: EcDevBackendNew> EcDev<File, I> {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut file = rwopen(path).map_err(EcDevError::Open)?;
        ec_dev_read_version_check(&mut file, path)?;
        EcDev::new(file)
    }
    pub fn open_by_name(name: impl AsRef<str>) -> Result<Self> {
        Self::open(Path::new("/dev").join(name.as_ref()))
    }
}

// NOTE: The above impl doesn't default I to Default for some reason.
impl EcDev {
    /// Creates a [`EcDev`] from [`CROS_EC_DEV_PATH`]
    pub fn open_cros_ec() -> Result<Self> {
        Self::open(CROS_EC_DEV_PATH)
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

impl<F: AsFd, I: EcDevBackendReadmem> EcHasReadmem for EcDev<F, I> {
    fn ec_readmem(&self, offset: i32, output: &mut [u8]) -> Result<usize, Errno> {
        self.iface.ec_readmem(&self.file, offset, output)
    }
}
