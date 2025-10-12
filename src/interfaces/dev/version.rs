use std::{
    fs::File,
    io::{Error as IoError, Read},
    path::Path,
};

use thiserror::Error;

use crate::consts::CROS_EC_DEV_VERSION;

#[derive(Debug, Error)]
pub enum DevVersionErrorKind {
    #[error("read failed: {}", _0)]
    Read(#[from] IoError),
    #[error("invalid: {}", _0)]
    Invalid(Box<str>),
}

#[derive(Debug, Error)]
#[error("failed to read version from {}: {}", path, kind)]
pub struct DevVersionError {
    path: Box<str>,
    kind: DevVersionErrorKind,
}

impl DevVersionError {
    fn new(path: impl AsRef<Path>, kind: DevVersionErrorKind) -> Self {
        let path = path
            .as_ref()
            .to_string_lossy()
            .into_owned()
            .into_boxed_str();
        Self { path, kind }
    }
}

pub fn ec_dev_read_version_check(file: &mut File, path: &Path) -> Result<(), DevVersionError> {
    let err = |kind| DevVersionError::new(path, kind);
    let mut buf = [0; 80];
    let len = file.read(&mut buf).map_err(|e| err(e.into()))?;
    let txt = str::from_utf8(&buf[..len]).unwrap();
    let mut lines = txt.lines();
    let version_str = lines.next().unwrap();
    if version_str != CROS_EC_DEV_VERSION {
        let kind = DevVersionErrorKind::Invalid(version_str.into());
        return Err(err(kind));
    }
    Ok(())
}
