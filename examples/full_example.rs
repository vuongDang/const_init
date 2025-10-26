#![allow(unused_variables, dead_code, unused_assignments)]
mod utils;
use const_init_macros::{ConstInit, const_init_code_modif};

// Content of the generated file "utils::generated_settings.rs"
// produced by `const_init_build`:
// pub const FOO: bool = true;
// pub const BAR: isize = 1;
// pub const B: [isize; 3] = [1,2,-3];
// pub const C: f64 = 3.14;
// pub const D: &str = "ding!";

#[derive(ConstInit)]
// Use this attribute if you want to import the global variables only
//  for the init function and not pollute the module namespace
// Otherwise you need to import the constants at the module level,
// here it would be `use utils::generated_settings::*;`
#[const_init(import_path = utils::generated_settings)]
struct FooBar {
    // Without attribute, looking for matching uppercase field name, here "FOO"
    foo: bool,
    bar: isize,
    // With attribute, it specifies a constant expr that will be assigned
    #[const_init(value = B)]
    b: [isize; 3],
    #[const_init(value = 3.14)]
    c: f64,
    #[const_init(value = D)]
    d: &'static str,
}

/* The code produced by the macro looks like this:
   impl FooBar {
       pub const fn const_init() -> Self {
           use generated::settings::*;
           FooBar {
               foo: FOO,
               bar: BAR,
               b: a::B,
               c: 3.14,
               d: a::D
           }
       }
   }
*/

// We show macros that can be used to modify your code easily
impl FooBar {
    #[const_init_code_modif(replace_with_const_init)]
    fn new(foo: bool, bar: isize, b: [isize; 3], c: f64, d: &'static str) -> Self {
        // With the macro the function code will be replaced with
        // "FooBar::CONST_INIT_VAR"
        FooBar { foo, bar, b, c, d }
    }

    #[const_init_code_modif(noop)]
    fn set_foo(&mut self, foo: bool) {
        // With the macro the function code will be replaced with noop
        self.foo = foo;
    }

    #[const_init_code_modif(target_params(self))]
    fn read_foo_bar(&self) {
        // With the macro this code will be added at the beginning
        // "self = FooBar:CONST_INIT_VAR;"
        println!("{}{}", self.foo, self.bar);
    }
}

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    // Constant initialization
    const FOO_BAR: FooBar = FooBar::const_init();
    if FOO_BAR.foo
        && FOO_BAR.bar == 1
        && FOO_BAR.b == [1, 2, -3]
        && FOO_BAR.c == 3.14
        && FOO_BAR.d == "ding!"
    {
        // Should be kept during compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
