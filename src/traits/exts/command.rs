use plain::{Plain, as_bytes, as_mut_bytes};

use crate::{error::EcCommandError, traits::EcHasCommand, types::EcCommandInfo};

pub trait EcCommandExt: EcHasCommand {
    /// # Safety
    ///
    /// See [EcHasCommand::ec_command]
    #[inline]
    unsafe fn ec_command_wrap<'a>(
        &self,
        command: &EcCommandInfo,
        input: impl Into<Option<&'a [u8]>>,
        output: impl Into<Option<&'a mut [u8]>>,
    ) -> Result<usize, EcCommandError> {
        let input = input.into();
        let output = output.into();
        unsafe { self.ec_command(command, input, output) }
    }
    /// # Safety
    ///
    /// See [EcHasCommand::ec_command]
    ///
    /// Types SHOULD be `repr(C)` and match what's expected by `command`.
    unsafe fn ec_command_r<I>(
        &self,
        command: &EcCommandInfo,
        input: &I,
    ) -> Result<usize, EcCommandError> {
        // SAFETY: types are repr(C) and match what's expected
        let input = unsafe { as_bytes(input) };
        unsafe { self.ec_command_wrap(command, input, None) }
    }
    /// # Safety
    ///
    /// See [EcHasCommand::ec_command]
    ///
    /// Types SHOULD be `repr(C)` and match what's expected by `command`.
    unsafe fn ec_command_w<O: Plain>(
        &self,
        command: &EcCommandInfo,
        output: &mut O,
    ) -> Result<usize, EcCommandError> {
        // SAFETY: types are repr(C) and match what's expected
        let output = unsafe { as_mut_bytes(output) };
        unsafe { self.ec_command_wrap(command, None, output) }
    }
    /// # Safety
    ///
    /// See [EcHasCommand::ec_command]
    ///
    /// Types SHOULD be `repr(C)` and match what's expected by `command`.
    unsafe fn ec_command_rw<I, O: Plain>(
        &self,
        command: &EcCommandInfo,
        input: &I,
        output: &mut O,
    ) -> Result<usize, EcCommandError> {
        // SAFETY: types are repr(C) and match what's expected
        let input = unsafe { as_bytes(input) };
        let output = unsafe { as_mut_bytes(output) };
        unsafe { self.ec_command_wrap(command, input, output) }
    }
}

impl<T: EcHasCommand> EcCommandExt for T {}
