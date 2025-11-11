use rustix::ioctl::{Opcode, opcode::read_write};

use super::command::CrosEcCommandV2Header;

const CROS_EC_DEV_IOC_V2: u8 = 0xEC;
pub const CROS_EC_DEV_IOCXCMD_V2: Opcode =
    read_write::<CrosEcCommandV2Header>(CROS_EC_DEV_IOC_V2, 0);
//pub const CROS_EC_DEV_IOCRDMEM_V2: Opcode = read_write::<()>(CROS_EC_DEV_IOC_V2, 1);
//const CROS_EC_DEV_IOCEVENTMASK_V2: u32 = _IO(CROS_EC_DEV_IOC_V2, 2);
