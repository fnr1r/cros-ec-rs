use trait_set::trait_set;

pub use self::{
    base::{EcHasCommand, EcHasPollevent, EcHasReadmem},
    exts::{EcCommandExt, EcCommandSizes, EcReadmemExt},
};

mod base;
mod exts;

trait_set! {
    pub trait EcHasAll = EcHasCommand + EcHasReadmem + EcHasPollevent;
}
