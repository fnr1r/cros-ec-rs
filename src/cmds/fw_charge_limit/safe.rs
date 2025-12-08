use super::{consts::*, prelude::*, raw::*};

#[derive(Debug, Clone, Default)]
pub struct EcFwChargeLimitConfig {
    pub do_clear: bool,
    pub do_set: Option<u16>,
    pub do_query: bool,
    pub do_override: bool,
}

impl EcFwChargeLimitParams {
    pub const fn from_config(config: EcFwChargeLimitConfig) -> Self {
        let mut this = Self::DEFAULT;

        let EcFwChargeLimitConfig {
            do_clear,
            do_set,
            do_query,
            do_override,
        } = config;
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

impl From<EcFwChargeLimitConfig> for EcFwChargeLimitParams {
    fn from(value: EcFwChargeLimitConfig) -> Self {
        Self::from_config(value)
    }
}

impl EcFwChargeLimitConfig {
    // TODO: Move to const-default
    pub const DEFAULT: Self = Self {
        do_clear: false,
        do_set: None,
        do_query: false,
        do_override: false,
    };
    pub const CLEAR: Self = Self {
        do_clear: true,
        ..Self::DEFAULT
    };
    pub const QUERY: Self = Self {
        do_query: true,
        ..Self::DEFAULT
    };
    pub const fn do_set(limit: u16) -> Self {
        Self {
            do_set: Some(limit),
            ..Self::DEFAULT
        }
    }
    pub const fn into_params(self) -> EcFwChargeLimitParams {
        EcFwChargeLimitParams::from_config(self)
    }
    pub fn as_params(&self) -> EcFwChargeLimitParams {
        self.clone().into_params()
    }
}

/// Sends a [`FW_CHARGE_LIMIT`](super::EC_CMD_FW_CHARGE_LIMIT) command with
/// the specified config.
pub fn ec_cmd_fw_charge_limit_config(
    iface: &impl EcHasCommand,
    config: EcFwChargeLimitConfig,
) -> Result<Option<u16>> {
    Ok(ec_cmd_fw_charge_limit(iface, &config.as_params())?.map(|e| e.limit))
}
