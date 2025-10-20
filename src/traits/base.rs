use crate::{
    error::EcCommandError,
    types::{EcCommandMeta, MaskT},
};

pub trait EcHasCommand {
    /// Send a command to the EC.
    ///
    /// Returns the length of output data stored in `output`.
    ///
    /// # Safety
    ///
    /// This calls abstract EC commands. It *should* be memory safe though.
    unsafe fn ec_command(
        &self,
        command: &EcCommandMeta,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError>;
}

pub trait EcHasReadmem {
    type Error;
    /// Return the content of the EC information area mapped as "memory".
    /// The offsets are defined by the `EC_MEMMAP_` constants.
    /// Returns the number of bytes read.
    ///
    /// COMPATIBILITY NOTE: Reading strings is a TODO.
    fn ec_readmem(&self, offset: i32, output: &mut [u8]) -> Result<usize, Self::Error>;
}

pub trait EcHasPollevent {
    type Error;
    fn ec_pollevent(&self, mask: MaskT, buf: &mut [u8], timeout: i32)
    -> Result<usize, Self::Error>;
}
