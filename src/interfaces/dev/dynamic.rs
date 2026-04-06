use self::EcDevVersion::*;
use super::{iface_prelude::*, v1::check::ec_dev_is_v1};

#[derive(Debug, Clone, Copy)]
pub enum EcDevVersion {
    V1,
    V2,
}

#[derive(Debug, Clone)]
pub struct Dynamic {
    version: EcDevVersion,
}

impl Dynamic {
    pub const fn new(version: EcDevVersion) -> Self {
        Self { version }
    }
}

impl From<EcDevVersion> for Dynamic {
    fn from(value: EcDevVersion) -> Self {
        Self::new(value)
    }
}

impl EcDevBackendCommand for Dynamic {
    #[inline]
    unsafe fn ec_command(
        &self,
        fd: impl AsFd,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError> {
        use EcDevVersion as E;
        let f = match self.version {
            E::V1 => ec_dev_v1_command,
            E::V2 => ec_dev_v2_command,
        };
        unsafe { f(fd, command, input, output) }
    }
}

impl EcDevBackendNew for Dynamic {
    fn ec_dev_new(fd: impl AsFd) -> Result<Self, EcDevError> {
        Ok(if ec_dev_is_v1(&fd)? { V1 } else { V2 }.into())
    }
}
