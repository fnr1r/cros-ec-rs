use super::{consts::EC_CMD_PWM_GET_FAN_TARGET_RPM, prelude::*};

pub fn ec_cmd_get_fan_target_rpm_v0(iface: &impl EcHasCommand) -> Result<u32> {
    unsafe { iface.ec_cmd_ext_wad(&EC_CMD_PWM_GET_FAN_TARGET_RPM) }
}
