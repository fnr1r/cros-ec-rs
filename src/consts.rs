use constcat::concat;

const CROS_EC_DEV_NAME: &str = "cros_ec";
pub const CROS_EC_DEV_VERSION: &str = "1.0.0";

pub const CROS_EC_DEV_PATH: &str = concat!("/dev/", CROS_EC_DEV_NAME);
