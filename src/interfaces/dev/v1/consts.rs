use rustix::ioctl::{Opcode, opcode::read_write};

use super::command::CrosEcCommandV1;

pub const CROS_EC_DEV_IOC: u8 = b':';
pub const CROS_EC_DEV_IOCXCMD: Opcode = read_write::<CrosEcCommandV1>(CROS_EC_DEV_IOC, 0);
