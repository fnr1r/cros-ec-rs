use super::{
    consts::{EC_CMD_PWM_SET_FAN_DUTY_V0, EC_CMD_PWM_SET_FAN_DUTY_V1},
    prelude::*,
};

type Percent = u32;

#[derive(Debug)]
#[repr(C, packed)]
pub struct PwmSetFanDutyV1Params {
    percent: Percent,
    fan_idx: u8,
}

pub fn ec_cmd_pwm_set_fan_duty_v0(iface: &impl EcHasCommand, percent: Percent) -> Result<()> {
    unsafe { iface.ec_cmd_ext_ri(&EC_CMD_PWM_SET_FAN_DUTY_V0, &percent) }
}

pub fn ec_cmd_pwm_set_fan_duty_v1(
    iface: &impl EcHasCommand,
    params: &PwmSetFanDutyV1Params,
) -> Result<()> {
    unsafe { iface.ec_cmd_ext_ri(&EC_CMD_PWM_SET_FAN_DUTY_V1, params) }
}
