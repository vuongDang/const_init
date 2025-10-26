#![allow(unused_assignments)]
//! The macro  `#[const_init_code_modif(target_params(...))]`
//! is meant to be used with functions where target const_init
//! parameters are immutable owned value or shared ref.
//!
//! The macro will add at the beginning of the function the
//! statement `[param_ident] = [param_type]::CONST_INIT_VAR;`
//! to set them with a constant value and enable compiler
//! optimizations

mod utils;
use const_init_macros::const_init_code_modif;
use utils::shared::*;

impl FooBar {
    /// The target_params "self" will
    /// be set at the beginning of the function with
    /// `self = Self::CONST_INIT_VAR;`
    #[const_init_code_modif(target_params(self))]
    fn input_owned(self) {
        code_that_should_be_optimized(&self);
        drop(self);
    }

    /// The target_params "self" will
    /// be set at the beginning of the function with
    /// `self = Self::CONST_INIT_VAR;`
    #[const_init_code_modif(target_params(self))]
    fn input_shared_ref(&self) {
        code_that_should_be_optimized(&self);
    }
}

fn main() {
    let fb = FooBar::random();
    fb.input_shared_ref();
    fb.input_owned();
}
