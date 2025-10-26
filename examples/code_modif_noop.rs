#![allow(unused_assignments, unused_mut)]
//! The macro  `#[const_init_code_modif(noop)]`
//! is meant to be used with functions where target const_init
//! parameters are mutable owned value or mutable ref.
//!
//! When using this crate, the assumption is that the target
//! const_init type will be immutable during the whole
//! execution.
//!
//! To enhance performance we give a macro to transform
//! a function performing mutations and returning the unit
//! type into a noop functions

mod utils;
use const_init_macros::const_init_code_modif;
use utils::shared::*;

impl FooBar {
    /// The function will be transformed into noop
    #[const_init_code_modif(noop)]
    fn mut_ref(&mut self) {
        self.bar = -1000;
    }

    /// The function will be transformed into noop
    #[const_init_code_modif(noop)]
    fn mutable_owned(mut self) {
        self = FooBar::random();
    }
}

fn main() {
    let mut fb = FooBar::CONST_INIT_VAR;
    (&mut fb).mut_ref();
    code_that_should_be_optimized(&fb);
    fb.mutable_owned();
}
