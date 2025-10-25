//! We inspect all the use cases where function gets
//! an owned value as input or output
#![allow(unused_assignments)]

mod utils;
use const_init_macros::const_init_code_modif;
use utils::shared::*;

impl FooBar {
    // Macro should replace any usage of the value self with FooBar::const_init()
    #[const_init_code_modif(target_params(self))]
    fn input_owned(self) {
        code_that_should_be_optimized(&self);
        drop(self);
    }

    fn playground(&self) -> &Self {
        &Self::CONST_INIT_VAR
    }

    // Macro should replace the return value at the end of the code with FooBar::const_init()
    fn output_value(foo: bool, bar: isize, b: [isize; 3], c: f64, d: &'static str) -> Self {
        let foo_bar = FooBar { foo, bar, b, c, d };
        foo_bar
    }

    // Macro should replace any usage of the value self with FooBar::const_init()
    // Macro should remove any mutation of self
    // Macro should replace the return value with FooBar::const_init()
    fn input_self_output_self(mut self, with_foo: bool) -> Self {
        self.foo = with_foo;
        // code_that_should_be_optimized(&self);
        self
    }
}

fn main() {
    let mut fb = FooBar::random();
    // fb.input_shared_ref();
    // fb.input_mut_ref();
    fb.input_owned();
}
