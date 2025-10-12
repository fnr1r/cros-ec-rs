//! # cros-ec-rs
//!
//! NOTE: This crate only aims to support the Framework laptop. Sorry.
pub mod cmds;
pub mod consts;
pub mod error;
pub mod interfaces;
pub mod traits;
pub mod types;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
