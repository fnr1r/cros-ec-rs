use std::os::fd::AsFd;

use super::EcDev;
use crate::cmds::hello::{EcHelloError, ec_cmd_hello};

pub fn ec_dev_is_v1(file: impl AsFd) -> Result<bool, EcHelloError> {
    let iface = EcDev::new_v1_unchecked(file);
    let res = ec_cmd_hello(&iface);
    let Err(error) = res else {
        return Ok(true);
    };
    error.ok_if_enotty()?;
    Ok(false)
}
