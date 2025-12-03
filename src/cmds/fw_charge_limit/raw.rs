use plain::Plain;

use super::{EC_CMD_FW_CHARGE_LIMIT, prelude::*};

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

pub fn ec_cmd_fw_charge_limit(
    iface: &impl EcHasCommand,
    params: &EcFwChargeLimitParams,
) -> Result<EcFwChargeLimitResponse> {
    unsafe { iface.ec_cmd_ext_rwad(&EC_CMD_FW_CHARGE_LIMIT, params) }
}
