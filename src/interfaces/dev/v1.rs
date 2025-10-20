use std::{ffi::c_void, os::fd::AsFd};

use derive_new::new;
use rustix::{
    io::Errno,
    ioctl::{Ioctl, IoctlOutput, Opcode, ioctl, opcode::read_write},
};

use crate::{
    error::{EcCommandError, EcError},
    types::{CommandT, EcCommandMeta, VersionT},
    utils::slice::{as_raw_mut_parts, as_raw_parts},
};

const CROS_EC_DEV_IOC: u8 = b':';
const CROS_EC_DEV_IOCXCMD: Opcode = read_write::<CrosEcCommand>(CROS_EC_DEV_IOC, 0);

#[derive(Debug, new)]
#[repr(C)]
struct CrosEcCommand {
    // Command version number (often 0)
    version: VersionT,
    // Command to send (prefixed with `EC_CMD_`)
    command: CommandT,
    // Outgoing data to EC
    outdata: *const u8,
    // Outgoing length in bytes
    outsize: u32,
    // Where to put the incoming data from EC
    indata: *mut u8,
    // On call, how much we can accept. On return, how much we got.
    insize: u32,
    // EC's response to the command (separate from communication failure)
    #[new(value = "0xff")]
    result: u32,
}

impl CrosEcCommand {
    fn new_sliced(
        command: &EcCommandMeta,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Self {
        let EcCommandMeta {
            command, version, ..
        } = *command;
        let (outdata, outsize) = input.map(as_raw_parts).unwrap_or_default();
        let outsize = outsize as u32;
        let (indata, insize) = output.map(as_raw_mut_parts).unwrap_or_default();
        let insize = insize as u32;
        Self::new(version, command, outdata, outsize, indata, insize)
    }
}

impl CrosEcCommand {
    unsafe fn from_ptr_mut<'a, T>(this: *mut T) -> &'a mut Self {
        unsafe { this.cast::<Self>().as_mut() }.unwrap()
    }
}

unsafe impl Ioctl for CrosEcCommand {
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

pub unsafe fn ec_command_dev_v1(
    fd: impl AsFd,
    command: &EcCommandMeta,
    input: Option<&[u8]>,
    output: Option<&mut [u8]>,
) -> Result<usize, EcCommandError> {
    let cmd = CrosEcCommand::new_sliced(command, input, output);
    let (result, len) = unsafe { ioctl(fd, cmd) }?;
    EcError::from_ec_result(result)?;
    Ok(len as usize)
}
