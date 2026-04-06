use super::super::iface_prelude::*;

#[derive(Debug, Clone)]
pub struct V1;

unsafe impl EcDevBackendEmpty for V1 {
    const INSTANCE: Self = Self;
}

impl EcDevBackendCommand for V1 {
    #[inline]
    unsafe fn ec_command(
        &self,
        fd: impl AsFd,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError> {
        unsafe { ec_dev_v1_command(fd, command, input, output) }
    }
}

crate::ec_dev_backend_new_impl_empty!(V1);
