pub use self::{
    dynamic::{Dynamic as IfaceDynamic, EcDevVersion},
    error::EcDevError,
    iface::EcDev,
    v1::{IfaceV1, ec_dev_v1_command},
    v2::{IfaceV2, ec_dev_v2_command, ec_dev_v2_readmem},
};

pub mod dynamic;
pub mod error;
pub mod iface;
mod traits;
pub mod v1;
pub mod v2;
mod version;

mod iface_prelude {
    pub(super) use std::os::fd::AsFd;

    pub(super) use rustix::io::Errno;

    pub(super) use super::{
        EcDev, EcDevError, ec_dev_v1_command, ec_dev_v2_command, ec_dev_v2_readmem, traits::*,
    };
    pub(super) use crate::{error::EcCommandError, types::EcCommandInfo};
}
