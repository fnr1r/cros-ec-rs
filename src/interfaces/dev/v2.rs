use std::{
    cmp::{max, min},
    ffi::c_void,
    iter::repeat_n,
    os::fd::AsFd,
};

use derive_new::new;
use easy_ext::ext;
use rustix::{
    io::Errno,
    ioctl::{Ioctl, IoctlOutput, Opcode, ioctl, opcode::read_write},
};
use slice_dst::SliceWithHeader;

use crate::{
    error::{EcCommandError, EcError},
    types::EcCommandMeta,
    types::{CommandT, VersionT},
};

const CROS_EC_DEV_IOC_V2: u8 = 0xEC;
pub const CROS_EC_DEV_IOCXCMD_V2: Opcode =
    read_write::<CrosEcCommandV2Header>(CROS_EC_DEV_IOC_V2, 0);
//pub const CROS_EC_DEV_IOCRDMEM_V2: Opcode = read_write::<()>(CROS_EC_DEV_IOC_V2, 1);
//const CROS_EC_DEV_IOCEVENTMASK_V2: u32 = _IO(CROS_EC_DEV_IOC_V2, 2);

#[derive(Debug, new)]
#[repr(C)]
struct CrosEcCommandV2Header {
    // Command version number (often 0)
    version: VersionT,
    // Command to send (prefixed with `EC_CMD_`)
    command: CommandT,
    // Outgoing length in bytes
    outsize: u32,
    // Max number of bytes to accept from EC
    insize: u32,
    // EC's response to the command (separate from communication failure)
    #[new(value = "0xff")]
    result: u32,
}

impl CrosEcCommandV2Header {
    fn data_len(&self) -> usize {
        max(self.insize, self.outsize) as usize
    }
}

type CrosEcCommandV2Inner = SliceWithHeader<CrosEcCommandV2Header, u8>;

#[ext]
impl CrosEcCommandV2Inner {
    type Header = CrosEcCommandV2Header;
    fn header_as_ptr_mut(&mut self) -> *mut Self::Header {
        &raw mut self.header
    }
}

/// blame
/// https://gitlab.howett.net/DHowett/ectool/-/blob/main/src/cros_ec_dev.h#L65
/// and
/// https://gitlab.howett.net/DHowett/ectool/-/blob/main/src/comm-dev.cc#L137
/// for this design
#[derive(Debug)]
#[repr(C)]
struct CrosEcCommandV2(Box<CrosEcCommandV2Inner>);

impl CrosEcCommandV2 {
    fn new(version: VersionT, command: CommandT, outsize: u32, insize: u32) -> Self {
        let header = CrosEcCommandV2Header::new(version, command, outsize, insize);
        let data_len = header.data_len();
        let this = SliceWithHeader::new(header, repeat_n(0, data_len));
        Self(this)
    }
}

unsafe impl Ioctl for &mut CrosEcCommandV2 {
    type Output = usize;
    const IS_MUTATING: bool = true;
    fn opcode(&self) -> Opcode {
        CROS_EC_DEV_IOCXCMD_V2
    }
    fn as_ptr(&mut self) -> *mut c_void {
        self.0.header_as_ptr_mut().cast()
    }
    unsafe fn output_from_ptr(
        out: IoctlOutput,
        _extract_output: *mut c_void,
    ) -> Result<Self::Output, Errno> {
        if out.is_negative() {
            return Err(Errno::from_raw_os_error(out));
        }
        Ok(out as Self::Output)
    }
}

fn slice_copy_min_len<T: Copy>(input: &[T], output: &mut [T]) -> usize {
    let len = min(input.len(), output.len());
    let input = &input[..len];
    let output = &mut output[..len];
    output.copy_from_slice(input);
    len
}

pub unsafe fn ec_command_dev_v2(
    fd: impl AsFd,
    command: &EcCommandMeta,
    input: Option<&[u8]>,
    output: Option<&mut [u8]>,
) -> Result<usize, EcCommandError> {
    let EcCommandMeta {
        command, version, ..
    } = *command;
    let outsize = input.map(|e| e.len() as u32).unwrap_or_default();
    let insize = output.as_ref().map(|e| e.len() as u32).unwrap_or_default();
    let mut cmd = CrosEcCommandV2::new(version, command, outsize, insize);
    if let Some(input) = input {
        slice_copy_min_len(input, &mut cmd.0.slice);
    }
    let real_data_len = unsafe { ioctl(fd, &mut cmd) }?;
    EcError::from_ec_result(cmd.0.header.result)?;
    if let Some(output) = output {
        slice_copy_min_len(&cmd.0.slice, output);
    }
    Ok(real_data_len)
}
