use paste::paste;

pub use self::pwm::*;

macro_rules! cmd {
    ($name:ident, $ver:expr) => {
        paste! {
            pub const [<EC_CMD_ $name:upper>]: $crate::types::EcCommandInfo = $crate::types::EcCommandInfo::new_known(
                $crate::types::EcKnownCommand::[<$name:camel>],
                $ver,
            );
        }
    };
    ($name:ident, $($ver:expr),*) => {
        $(paste! {
            pub const [<EC_CMD_ $name:upper _V $ver>]: $crate::types::EcCommandInfo = $crate::types::EcCommandInfo::new_known(
                $crate::types::EcKnownCommand::[<$name:camel>],
                $ver,
            );
        })*
    };
}

cmd!(proto_version, 0);
cmd!(hello, 0);
cmd!(get_version, 0, 1);
cmd!(get_cmd_versions, 0, 1);
cmd!(get_features, 0);
cmd!(flash_info, 0, 1, 2);
cmd!(fw_charge_limit, 0);

pub(super) mod pwm {
    use super::*;

    cmd!(pwm_get_fan_target_rpm, 0);
    cmd!(pwm_set_fan_target_rpm, 0, 1);
    cmd!(pwm_set_fan_duty, 0, 1);
    cmd!(pwm_set_duty, 0);
}

pub(super) mod thermal {
    use super::*;

    cmd!(thermal_auto_fan_ctrl, 0, 1);
}
