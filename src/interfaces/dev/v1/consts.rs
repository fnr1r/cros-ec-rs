use rustix::ioctl::{Opcode, opcode::read_write};

use super::command::CrosEcCommand;

pub const CROS_EC_DEV_IOC: u8 = b':';
pub const CROS_EC_DEV_IOCXCMD: Opcode = read_write::<CrosEcCommand>(CROS_EC_DEV_IOC, 0);
