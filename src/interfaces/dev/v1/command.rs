use std::{ffi::c_void, ops::Deref, os::fd::AsFd};

use derive_new::new;
use rustix::{
    io::Errno,
    ioctl::{Ioctl, IoctlOutput, Opcode, ioctl},
};

use super::consts::CROS_EC_DEV_IOCXCMD;
use crate::{
    error::{EcCommandError, EcError},
    types::{CommandT, EcCommandInfo, VersionT},
    utils::slice::{as_raw_mut_parts, as_raw_parts},
};

#[derive(Debug, new)]
#[repr(C)]
pub struct CrosEcCommandV1 {
    // Command version number (often 0)
    pub version: VersionT,
    // Command to send (prefixed with `EC_CMD_`)
    pub command: CommandT,
    // Outgoing data to EC
    pub outdata: *const u8,
    // Outgoing length in bytes
    pub outsize: u32,
    // Where to put the incoming data from EC
    pub indata: *mut u8,
    // On call, how much we can accept. On return, how much we got.
    pub insize: u32,
    // EC's response to the command (separate from communication failure)
    #[new(value = "0xff")]
    pub result: u32,
}

impl CrosEcCommandV1 {
    fn new_sliced(
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Self {
        let EcCommandInfo { command, version } = *command;
        let (outdata, outsize) = input.map(as_raw_parts).unwrap_or_default();
        let outsize = outsize as u32;
        let (indata, insize) = output.map(as_raw_mut_parts).unwrap_or_default();
        let insize = insize as u32;
        Self::new(version, command, outdata, outsize, indata, insize)
    }
}

/// This is here to avoid leaking a Ioctl impl for [CrosEcCommandV1]
#[derive(Debug)]
#[repr(transparent)]
pub struct CrosEcCommandV1Wrap(CrosEcCommandV1);

impl Deref for CrosEcCommandV1Wrap {
    type Target = CrosEcCommandV1;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CrosEcCommandV1Wrap {
    fn new_sliced(
        command: &EcCommandInfo,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Self {
        Self(CrosEcCommandV1::new_sliced(command, input, output))
    }
    unsafe fn from_ptr_mut<'a, T>(this: *mut T) -> &'a mut Self {
        unsafe { this.cast::<Self>().as_mut() }.unwrap()
    }
}

unsafe impl Ioctl for CrosEcCommandV1Wrap {
    type Output = (u32, u32);
    const IS_MUTATING: bool = true;
    fn opcode(&self) -> Opcode {
        CROS_EC_DEV_IOCXCMD
    }
    fn as_ptr(&mut self) -> *mut c_void {
        self as *mut Self as *mut c_void
    }
    unsafe fn output_from_ptr(
        out: IoctlOutput,
        extract_output: *mut c_void,
    ) -> Result<Self::Output, Errno> {
        if out.is_negative() {
            return Err(Errno::from_raw_os_error(out));
        }
        let this = unsafe { Self::from_ptr_mut(extract_output) };
        Ok((this.result, this.insize))
    }
}

/// Send a command to the EC though the ioctl V1 interface.
///
/// Returns the length of output data stored in `output`.
///
/// # Safety
///
/// This calls abstract EC commands. It *should* be memory safe though.
pub unsafe fn ec_command_dev_v1(
    fd: impl AsFd,
    command: &EcCommandInfo,
    input: Option<&[u8]>,
    output: Option<&mut [u8]>,
) -> Result<usize, EcCommandError> {
    let cmd = CrosEcCommandV1Wrap::new_sliced(command, input, output);
    let (result, len) = unsafe { ioctl(fd, cmd) }?;
    EcError::from_ec_result(result)?;
    Ok(len as usize)
}
