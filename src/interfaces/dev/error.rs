use std::io::Error as IoError;

use thiserror::Error;

use super::version::DevVersionError;
use crate::{cmds::hello::EcHelloError, error::EcCommandError};

#[derive(Debug, Error)]
pub enum EcDevError {
    #[error("failed to open: {}", _0)]
    Open(IoError),
    #[error(transparent)]
    Version(#[from] DevVersionError),
    #[error("handshake failed")]
    Handshake(#[from] EcHelloError),
    #[error(transparent)]
    Command(EcCommandError),
}
