use super::super::iface_prelude::*;

#[derive(Debug, Clone)]
pub struct V2;

crate::ec_dev_backend_ver_impl!(V2);

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

impl EcDevBackendReadmem for V2 {
    #[inline]
    fn ec_readmem(&self, fd: impl AsFd, offset: i32, output: &mut [u8]) -> Result<usize, Errno> {
        ec_dev_v2_readmem(fd, offset, output)
    }
}
