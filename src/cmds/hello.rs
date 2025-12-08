use constcat::concat;
use easy_ext::ext;
use libc::ENOTTY;
use rustix::io::Errno;
use thiserror::Error;

pub use super::consts::EC_CMD_HELLO;
use super::prelude::*;

type HelloT = u32;

pub const EC_CMD_HELLO_INPUT: HelloT = 0xa0b0c0d0;
pub const EC_CMD_HELLO_RESP: HelloT = 0x01020304;
pub const EC_CMD_HELLO_OUTPUT: HelloT = EC_CMD_HELLO_INPUT | EC_CMD_HELLO_RESP;

#[ext]
impl Errno {
    fn ok_if_enotty(self) -> Result<(), Errno> {
        let errno = self.raw_os_error();
        if errno == ENOTTY {
            Ok(())
        } else {
            Err(Errno::from_raw_os_error(errno))
        }
    }
}

impl EcCommandError {
    fn ok_if_enotty(self) -> Result<(), EcCommandError> {
        use EcCommandError as E;
        match self {
            E::Errno(err) => err.ok_if_enotty().map_err(EcCommandError::from),
            e => Err(e),
        }
    }
}

#[derive(Debug, Error)]
#[error("expected response 0x{:x} got 0x{:x}", EC_CMD_HELLO_OUTPUT, magic)]
pub struct EcHelloMagicError {
    magic: HelloT,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum EcHelloError {
    IoctlFailed(#[from] EcCommandError),
    InvalidMagic(#[from] EcHelloMagicError),
}

impl EcHelloError {
    fn try_into_ec_err(self) -> Result<EcCommandError, Self> {
        match self {
            Self::IoctlFailed(e) => Ok(e),
            e => Err(e),
        }
    }
    pub(crate) fn ok_if_enotty(self) -> Result<(), Self> {
        self.try_into_ec_err()?
            .ok_if_enotty()
            .map_err(Self::IoctlFailed)
    }
}

const PROGRAMMER_IS_AN_IDIOT_ERROR: &str = concat!(
    "your abstraction is ass! session terminated!\n",
    "you forgot to set the input, idiot!",
);

/// Sends a [`HELLO`](EC_CMD_HELLO) command to the EC and checks the result.
pub fn ec_cmd_hello(iface: &impl EcHasCommand) -> Result<(), EcHelloError> {
    let output = unsafe { iface.ec_cmd_ext_rwad(&EC_CMD_HELLO, &EC_CMD_HELLO_INPUT) }?;
    if output != EC_CMD_HELLO_OUTPUT {
        debug_assert_eq!(
            output, EC_CMD_HELLO_RESP,
            "{}",
            PROGRAMMER_IS_AN_IDIOT_ERROR
        );
        Err(EcHelloMagicError { magic: output })?;
    }
    Ok(())
}
