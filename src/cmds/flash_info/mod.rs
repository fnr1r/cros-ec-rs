pub use super::consts::{EC_CMD_FLASH_INFO_V0, EC_CMD_FLASH_INFO_V1, EC_CMD_FLASH_INFO_V2};

pub mod flags;
pub mod v0;
pub mod v1;
pub mod v2;

mod prelude {
    pub(super) use plain::Plain;

    pub(super) use super::super::prelude::*;
}
