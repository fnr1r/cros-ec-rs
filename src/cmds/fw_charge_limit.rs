use plain::Plain;

pub use super::consts::EC_CMD_FW_CHARGE_LIMIT;
use super::prelude::*;

pub const FW_CHARGE_LIMIT_CLEAR: u8 = 1 << 0;
pub const FW_CHARGE_LIMIT_SET: u8 = 1 << 1;
pub const FW_CHARGE_LIMIT_QUERY: u8 = 1 << 3;
pub const FW_CHARGE_LIMIT_OVERRIDE: u8 = 1 << 7;

#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct EcFwChargeLimitParams {
    flags: u8,
    limit: u16,
}

#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct EcFwChargeLimitResponse {
    limit: u16,
}

unsafe impl Plain for EcFwChargeLimitResponse {}

pub fn ec_cmd_fw_charge_limit_raw(
    iface: &impl EcHasCommand,
    params: &EcFwChargeLimitParams,
) -> Result<EcFwChargeLimitResponse> {
    unsafe { iface.ec_cmd_ext_rwad(&EC_CMD_FW_CHARGE_LIMIT, params) }
}

#[derive(Debug, Default)]
pub struct EcFwChargeLimitConfig {
    pub do_clear: bool,
    pub do_set: Option<u16>,
    pub do_query: bool,
    pub do_override: bool,
}

impl EcFwChargeLimitConfig {
    pub fn clear() -> Self {
        Self {
            do_clear: true,
            ..Default::default()
        }
    }
    pub fn set(limit: impl Into<u16>) -> Self {
        Self {
            do_set: Some(limit.into()),
            ..Default::default()
        }
    }
    pub fn query() -> Self {
        Self {
            do_query: true,
            ..Default::default()
        }
    }
}

impl From<EcFwChargeLimitConfig> for EcFwChargeLimitParams {
    fn from(value: EcFwChargeLimitConfig) -> Self {
        let mut this = Self::default();

        let EcFwChargeLimitConfig {
            do_clear,
            do_query,
            do_set,
            do_override,
        } = value;
        if do_clear {
            this.flags |= FW_CHARGE_LIMIT_CLEAR;
        }
        if let Some(limit) = do_set {
            this.flags |= FW_CHARGE_LIMIT_SET;
            this.limit = limit;
        }
        if do_query {
            this.flags |= FW_CHARGE_LIMIT_QUERY;
        }
        if do_override {
            this.flags |= FW_CHARGE_LIMIT_OVERRIDE;
        }

        this
    }
}

/// Sends a [FW_CHARGE_LIMIT](EC_CMD_FW_CHARGE_LIMIT) command with the
/// specified config.
pub fn ec_cmd_fw_charge_limit(
    iface: &impl EcHasCommand,
    config: EcFwChargeLimitConfig,
) -> Result<Option<u16>> {
    let cmd = EcFwChargeLimitParams::from(config);
    let buf = ec_cmd_fw_charge_limit_raw(iface, &cmd)?;
    Ok(if cmd.flags & FW_CHARGE_LIMIT_QUERY != 0 {
        Some(buf.limit)
    } else {
        None
    })
}
