use plain::{Plain, as_bytes, as_mut_bytes};

use super::super::EcHasCommand;
use crate::utils::invalid_response::INVALID_RESPONSE_ERR;

type Error = crate::error::EcCommandError;
type Info = crate::types::EcCommandInfo;
type Result<T, E = Error> = core::result::Result<T, E>;

#[inline]
fn output_length_check<T>(len: usize) -> Result<()> {
    if len == size_of::<T>() {
        return Ok(());
    }
    Err(INVALID_RESPONSE_ERR)
}

/// An extension trait for [`EcHasCommand`] that provides wrapper methods for
/// sending EC commands.
///
/// This trait simplifies sending commands by handling the `unsafe` casting of data
/// structures to and from the raw byte slices required for EC communication.
///
/// ## Method Naming Convention
///
/// Method suffixes indicate their behavior:
/// - `_r`: Sends data to the EC (**r**eads from input).
/// - `_w`: Receives data from the EC (**w**rites to output).
/// - `_a`: **A**sserts that the received data size matches the output type's size.
/// - `_d`: Uses a **d**efault-initialized struct for the output.
///
/// For example, `ec_cmd_ext_rwad` combines all of these: it sends data (`r`),
/// receives data (`w`), asserts the size (`a`), and uses a default struct (`d`).
///
/// ## Safety
///
/// All methods in this trait are `unsafe` because they perform low-level EC
/// communication. The caller **must** ensure that any types used for input (`I`)
/// or output (`O`) satisfy the following requirements:
///
/// 1.  They must have a well-defined and predictable memory layout (e.g., using
///     `#[repr(C)]` or `#[repr(transparent)]`).
/// 2.  Their memory layout must exactly match the structure expected by the
///     specific EC command being called.
///
/// Failure to meet these requirements can lead to memory corruption, incorrect
/// behavior, or other undefined behavior.
pub trait EcCommandExt: EcHasCommand {
    /// A low-level wrapper for `ec_command` that automatically wraps byte slices
    /// in `Option`. This is the foundation for other methods in this trait.
    ///
    /// # Safety
    ///
    /// See [`EcHasCommand::ec_command`].
    #[inline]
    unsafe fn ec_cmd_wrap_into<'a>(
        &self,
        command: &Info,
        input: impl Into<Option<&'a [u8]>>,
        output: impl Into<Option<&'a mut [u8]>>,
    ) -> Result<usize> {
        unsafe { self.ec_command(command, input.into(), output.into()) }
    }
    /// Sends a command with input data but no output. (A "setter").
    ///
    /// # Safety
    ///
    /// The input type `I` must have a predictable memory layout and match the EC
    /// command's expectation. See the trait-level safety documentation for more
    /// information.
    #[inline]
    unsafe fn ec_cmd_ext_r<I>(&self, command: &Info, input: &I) -> Result<usize> {
        // SAFETY: types are repr(C) and match what's expected
        let input = unsafe { as_bytes(input) };
        unsafe { self.ec_cmd_wrap_into(command, input, None) }
    }
    /// Sends a command with no input but receives output. (A "getter").
    ///
    /// **Note:** This method does not verify the returned data size. For a safer
    /// alternative, see [`EcCommandExt::ec_cmd_ext_wa`].
    ///
    /// # Safety
    ///
    /// The output type `O` must have a predictable memory layout and match the EC
    /// command's expectation. The caller must also check the returned `usize`.
    /// See the trait-level safety documentation for more information.
    #[inline]
    unsafe fn ec_cmd_ext_w<O: Plain>(&self, command: &Info, output: &mut O) -> Result<usize> {
        // SAFETY: types are repr(C) and match what's expected
        let output = unsafe { as_mut_bytes(output) };
        unsafe { self.ec_cmd_wrap_into(command, None, output) }
    }
    /// Sends a command with both input and output data.
    ///
    /// **Note:** This method does not verify the returned data size. For a safer
    /// alternative, see [`EcCommandExt::ec_cmd_ext_rwa`].
    ///
    /// # Safety
    ///
    /// The input `I` and output `O` types must have a predictable memory layout
    /// and match the EC command's expectation. The caller must check the
    /// returned `usize`. See the trait-level safety documentation for more
    /// information.
    #[inline]
    unsafe fn ec_cmd_ext_rw<I, O: Plain>(
        &self,
        command: &Info,
        input: &I,
        output: &mut O,
    ) -> Result<usize> {
        // SAFETY: types are repr(C) and match what's expected
        let input = unsafe { as_bytes(input) };
        let output = unsafe { as_mut_bytes(output) };
        unsafe { self.ec_cmd_wrap_into(command, input, output) }
    }
    /// Sends a command with no input, receives output, and verifies the output size.
    /// This is a safer version of [`EcCommandExt::ec_cmd_ext_w`].
    ///
    /// # Safety
    ///
    /// The output type `O` must have a predictable memory layout and match the EC
    /// command's expectation. See the trait-level safety documentation for more
    /// information.
    #[inline]
    unsafe fn ec_cmd_ext_wa<O: Plain>(&self, command: &Info, output: &mut O) -> Result<()> {
        unsafe { self.ec_cmd_ext_w(command, output) }.and_then(output_length_check::<O>)
    }
    /// Sends a command with input, receives output, and verifies the output size.
    /// This is a safer version of [`EcCommandExt::ec_cmd_ext_rw`].
    ///
    /// # Safety
    ///
    /// The input `I` and output `O` types must have a predictable memory layout
    /// and match the EC command's expectation. See the trait-level safety
    /// documentation for more information.
    #[inline]
    unsafe fn ec_cmd_ext_rwa<I, O: Plain>(
        &self,
        command: &Info,
        input: &I,
        output: &mut O,
    ) -> Result<()> {
        unsafe { self.ec_cmd_ext_rw(command, input, output) }.and_then(output_length_check::<O>)
    }
    /// Sends a command with no input, receives output into a default-initialized
    /// struct, and verifies the output size. The safest "getter" method.
    ///
    /// # Safety
    ///
    /// The output type `O` must have a predictable memory layout and match the EC
    /// command's expectation. See the trait-level safety documentation for more
    /// information.
    #[inline]
    unsafe fn ec_cmd_ext_wad<O: Default + Plain>(&self, command: &Info) -> Result<O> {
        let mut res = O::default();
        unsafe { self.ec_cmd_ext_wa(command, &mut res) }?;
        Ok(res)
    }
    /// Sends a command with input, receives output into a default-initialized
    /// struct, and verifies the output size. The safest method for read-write commands.
    ///
    /// # Safety
    ///
    /// The input `I` and output `O` types must have a predictable memory layout
    /// and match the EC command's expectation. See the trait-level safety
    /// documentation for more information.
    #[inline]
    unsafe fn ec_cmd_ext_rwad<I, O: Default + Plain>(
        &self,
        command: &Info,
        input: &I,
    ) -> Result<O> {
        let mut res = O::default();
        unsafe { self.ec_cmd_ext_rwa(command, input, &mut res) }?;
        Ok(res)
    }
}

impl<T: EcHasCommand> EcCommandExt for T {}
