pub(crate) use crate::{
    error::EcCommandError,
    traits::{EcCommandExt, EcHasCommand},
};

pub(crate) type Result<T, E = EcCommandError> = core::result::Result<T, E>;
