pub mod consts {
    pub use super::super::consts::pwm::*;
}
mod prelude {
    pub(super) use super::super::prelude::*;
}

pub mod set_duty;
pub mod set_fan_duty;
