use super::{
    consts::{EC_CMD_THERMAL_AUTO_FAN_CTRL_V0, EC_CMD_THERMAL_AUTO_FAN_CTRL_V1},
    prelude::*,
};

pub fn ec_cmd_thermal_auto_fan_ctrl_v0(iface: &impl EcHasCommand) -> Result<()> {
    unsafe { iface.ec_cmd_send(&EC_CMD_THERMAL_AUTO_FAN_CTRL_V0) }
}

pub fn ec_cmd_thermal_auto_fan_ctrl_v1(iface: &impl EcHasCommand, fan_idx: u8) -> Result<()> {
    unsafe { iface.ec_cmd_ext_ri(&EC_CMD_THERMAL_AUTO_FAN_CTRL_V1, &fan_idx) }
}
