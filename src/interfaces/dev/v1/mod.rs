pub use self::{check::ec_dev_is_v1, command::*, consts::*, iface::V1 as IfaceV1};

pub mod check;
pub mod command;
pub mod consts;
pub mod iface;

pub type EcDevV1<F> = super::EcDev<F, IfaceV1>;
