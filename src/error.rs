use std::{borrow::Cow, io::Error as IoError, num::NonZeroU32};

use rustix::io::Errno;
use strum::{FromRepr, IntoStaticStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
#[non_exhaustive]
pub enum EcResult {
    Success,
    InvalidCommand,
    Error,
    InvalidParam,
    AccessDenied,
    InvalidResponse,
    InvalidVersion,
    InvalidChecksum,
    InProgress,
    Unavailable,
    Timeout,
    Overflow,
    InvalidHeader,
    RequestTruncated,
    ResponseTooBig,
    BusError,
    Busy,
    InvalidHeaderVersion,
    InvalidHeaderCrc,
    InvalidDataCrc,
    DupUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("{}", self.as_str())]
#[repr(transparent)]
pub struct EcError(NonZeroU32);

impl EcError {
    pub fn kind(&self) -> Option<EcResult> {
        EcResult::from_repr(self.0.into())
    }
    pub fn as_str(&self) -> Cow<'static, str> {
        if let Some(res) = self.kind() {
            return Cow::Borrowed(res.into());
        };
        Cow::Owned(format!("EcError {} <unknown>", self.0))
    }
    pub fn from_ec_result(res: u32) -> Result<(), Self> {
        if let Some(err) = NonZeroU32::new(res) {
            Err(Self(err))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum EcCommandError {
    IoError(#[from] IoError),
    Errno(#[from] Errno),
    EcError(#[from] EcError),
}
