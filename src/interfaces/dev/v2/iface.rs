use super::super::iface_prelude::*;

#[derive(Debug, Clone)]
pub struct V2;

unsafe impl EcDevBackendEmpty for V2 {
    const INSTANCE: Self = Self;
}

impl EcDevBackendCommand for V2 {
    #[inline]
    unsafe fn ec_command(
        &self,
        fd: impl AsFd,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError> {
        unsafe { ec_dev_v2_command(fd, command, input, output) }
    }
}
