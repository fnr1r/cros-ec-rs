use super::{EcDevVersion, iface_prelude::*};

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
