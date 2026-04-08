use super::{consts::EC_CMD_PWM_SET_DUTY, prelude::*};

#[derive(Debug)]
#[repr(C)]
pub struct PwmSetDutyParams {
    duty: u16,
    pwm_type: u8,
    index: u8,
}

pub fn ec_cmd_pwm_set_duty_v0(iface: &impl EcHasCommand, params: PwmSetDutyParams) -> Result<()> {
    unsafe { iface.ec_cmd_ext_ri(&EC_CMD_PWM_SET_DUTY, &params) }
}
