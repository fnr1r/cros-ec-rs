use rustix::ioctl::{Opcode, opcode::read_write};

use super::command::DevCommandV1;

pub const CROS_EC_DEV_IOC: u8 = b':';
pub const CROS_EC_DEV_IOCXCMD: Opcode = read_write::<DevCommandV1>(CROS_EC_DEV_IOC, 0);
