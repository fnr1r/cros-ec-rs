use easy_ext::ext;
use plain::{Plain, as_bytes, as_mut_bytes};

use crate::{error::EcCommandError, traits::EcHasCommand, types::EcCommandMeta};

/// TODO: specialization on this?
pub trait EcCommandSizes: EcHasCommand {
    const MAX_INSIZE: usize;
    const MAX_OUTSIZE: usize;
}

#[ext(EcCommandExt)]
pub impl<T: EcHasCommand> T {
    /// # Safety
    ///
    /// See [EcHasCommand::ec_command]
    ///
    /// Types SHOULD be `repr(C)` and match what's expected by `command`.
    unsafe fn do_command<I, O: Plain>(
        &self,
        command: &EcCommandMeta,
        input: &I,
        output: &mut O,
    ) -> Result<usize, EcCommandError> {
        // SAFETY: types are repr(C) and match what's expected
        let input = unsafe { as_bytes(input) };
        let output = unsafe { as_mut_bytes(output) };
        unsafe { T::ec_command(self, command, Some(input), Some(output)) }
    }
}
