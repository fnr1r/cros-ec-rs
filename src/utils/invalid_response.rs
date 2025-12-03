use crate::error::{EcCommandError, EcError, EcResult};

pub const INVALID_RESPONSE_EC_ERR: EcError = EcError::err_from_ec_result(EcResult::InvalidResponse);
pub const INVALID_RESPONSE_ERR: EcCommandError = EcCommandError::EcError(INVALID_RESPONSE_EC_ERR);
