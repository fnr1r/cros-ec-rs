use core::ffi::c_void;
use std::os::fd::AsFd;

use derive_new::new;
use rustix::{
    io::Errno,
    ioctl::{Ioctl, IoctlOutput, Opcode, ioctl},
};

use super::consts::CROS_EC_DEV_IOCRDMEM_V2;
use crate::utils::slice::slice_copy_min_len;

/// ACPI IO buffer max is 255 bytes
const EC_MEMMAP_SIZE: usize = 0xff;

#[derive(Debug, new)]
#[repr(C)]
pub struct DevReadmemV2 {
    /// within EC_LPC_ADDR_MEMMAP region
    offset: u32,
    /// number of bytes to read
    bytes: u32,
    /// where to store the result
    #[new(value = "[0; EC_MEMMAP_SIZE]")]
    buf: [u8; EC_MEMMAP_SIZE],
}

/// This is here to avoid leaking a Ioctl impl for [`DevReadmemV2`]
#[derive(Debug, new)]
struct DevReadmemV2Wrap<'a>(&'a mut DevReadmemV2);

unsafe impl Ioctl for DevReadmemV2Wrap<'_> {
    type Output = usize;
    const IS_MUTATING: bool = true;
    fn opcode(&self) -> Opcode {
        CROS_EC_DEV_IOCRDMEM_V2
    }
    fn as_ptr(&mut self) -> *mut c_void {
        self.0 as *mut DevReadmemV2 as _
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

/// Read memory of the EC though the ioctl V2 interface.
///
/// Returns the length of output data stored in `output`.
///
/// # Safety
///
/// It just wraps and ioctl. It'll error out if it fails.
///
/// It *should* be memory safe.
pub fn ec_dev_v2_readmem(fd: impl AsFd, offset: i32, output: &mut [u8]) -> Result<usize, Errno> {
    let mut cmd = DevReadmemV2::new(offset as _, output.len() as _);
    let real_data_len = unsafe { ioctl(fd, DevReadmemV2Wrap::new(&mut cmd)) }?;
    slice_copy_min_len(&cmd.buf, output);
    Ok(real_data_len)
}
