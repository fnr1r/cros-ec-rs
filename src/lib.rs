//! # cros-ec-rs
//!
//! NOTE: This crate only aims to support the Framework laptop. Sorry.
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::std_instead_of_core)]

pub mod cmds;
pub mod consts;
pub mod error;
pub mod interfaces;
pub mod traits;
pub mod types;
mod utils;
