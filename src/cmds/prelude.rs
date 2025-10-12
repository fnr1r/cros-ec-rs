pub use crate::{
    error::EcCommandError,
    traits::{EcCommandExt, EcHasCommand},
};

pub type Result<T, E = EcCommandError> = core::result::Result<T, E>;
