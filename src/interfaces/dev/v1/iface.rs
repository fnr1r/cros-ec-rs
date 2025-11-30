use std::{fs::File, os::fd::AsFd};

use derive_more::Deref;

use super::command::ec_command_dev_v1;
use crate::{
    cmds::hello::{EcHelloError, ec_cmd_hello},
    error::EcCommandError,
    traits::EcHasCommand,
    types::EcCommandInfo,
};

#[derive(Debug, Deref)]
#[repr(transparent)]
pub struct EcDevV1<F: AsFd = File>(F);

impl<F: AsFd> EcDevV1<F> {
    /// Creates an [EcDevV1] without sending a [hello](ec_cmd_hello) command
    ///
    /// Not marked as `unsafe` since any interface should error
    pub const fn new_unchecked(file: F) -> Self {
        Self(file)
    }
    pub fn new(file: F) -> Result<Self, EcHelloError> {
        let this = Self::new_unchecked(file);
        ec_cmd_hello(&this)?;
        Ok(this)
    }
}

impl<F: AsFd> EcHasCommand for EcDevV1<F> {
    unsafe fn ec_command(
        &self,
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, EcCommandError> {
        let fd = &self.0;
        unsafe { ec_command_dev_v1(fd, command, input, output) }
    }
}
