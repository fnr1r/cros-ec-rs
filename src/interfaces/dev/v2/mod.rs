pub use self::{command::*, consts::*, iface::V2 as IfaceV2, readmem::*};

pub mod command;
pub mod consts;
pub mod iface;
pub mod readmem;

pub type EcDevV2<F> = super::EcDev<F, IfaceV2>;
