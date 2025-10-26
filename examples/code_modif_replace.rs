#![allow(unused_assignments, unused_mut)]
//! The macro  `#[const_init_code_modif(replace)]`
//! is meant to be used with functions which return
//! an owned value of a target const_init type.
//!
//! When using this crate, the assumption is that the target
//! const_init type will be immutable during the whole
//! execution.
//!
//! To enhance performance we give a macro to replace
//! a function constructing a const_init type
//! to returning a constant value of that type

mod utils;
use const_init_macros::const_init_code_modif;
use utils::shared::*;

impl FooBar {
    /// The function will be transformed into a constant function
    /// returning a constant value
    #[const_init_code_modif(replace_with_const_init)]
    fn new() -> Self {
        FooBar::random()
    }
}

fn main() {
    let fb = FooBar::new();
    code_that_should_be_optimized(&fb);
}
