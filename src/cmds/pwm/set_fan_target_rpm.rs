use super::{
    consts::{EC_CMD_PWM_SET_FAN_TARGET_RPM_V0, EC_CMD_PWM_SET_FAN_TARGET_RPM_V1},
    prelude::*,
};

#[derive(Debug)]
#[repr(C, packed)]
pub struct SetFanTargetRpmV1 {
    rpm: u32,
    fan_idx: u8,
}

pub fn ec_cmd_set_fan_target_rpm_v0(iface: &impl EcHasCommand, rpm: u32) -> Result<()> {
    unsafe { iface.ec_cmd_ext_ri(&EC_CMD_PWM_SET_FAN_TARGET_RPM_V0, &rpm) }
}

pub fn ec_cmd_set_fan_target_rpm_v1(
    iface: &impl EcHasCommand,
    params: &SetFanTargetRpmV1,
) -> Result<()> {
    unsafe { iface.ec_cmd_ext_ri(&EC_CMD_PWM_SET_FAN_TARGET_RPM_V1, params) }
}
