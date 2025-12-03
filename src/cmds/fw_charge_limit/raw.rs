use plain::Plain;

use super::{EC_CMD_FW_CHARGE_LIMIT, prelude::*};
use crate::utils::invalid_response::INVALID_RESPONSE_ERR;

#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct EcFwChargeLimitParams {
    pub flags: u8,
    pub limit: u16,
}

impl EcFwChargeLimitParams {
    // TODO: Move to const-default
    pub const DEFAULT: Self = Self { flags: 0, limit: 0 };
}

#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct EcFwChargeLimitResponse {
    pub limit: u16,
}

unsafe impl Plain for EcFwChargeLimitResponse {}

impl EcFwChargeLimitResponse {
    pub const DEFAULT: Self = Self { limit: 0 };
}

const RESPONSE_SIZE: usize = size_of::<EcFwChargeLimitResponse>();

pub fn ec_cmd_fw_charge_limit(
    iface: &impl EcHasCommand,
    params: &EcFwChargeLimitParams,
) -> Result<Option<EcFwChargeLimitResponse>> {
    let mut res = EcFwChargeLimitResponse::DEFAULT;
    let len = unsafe { iface.ec_cmd_ext_rw(&EC_CMD_FW_CHARGE_LIMIT, params, &mut res) }?;
    Ok(match len {
        0 => None,
        RESPONSE_SIZE => Some(res),
        _ => return Err(INVALID_RESPONSE_ERR),
    })
}
