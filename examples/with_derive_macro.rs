mod generated;
use const_init_macros::ConstInit;

/* Content of the generated file "generated::settings.rs":
pub const FOO: bool = true;
pub const BAR: isize = 1;
pub mod a {
    pub const B: [isize; 3] = [1,2,-3];
    pub const C: f64 = 3.14;
    pub const D: &str = "ding!";
}
*/

#[derive(ConstInit)]
// Use this attribute if you want to import the global variables only
//  for the init function and not pollute the module namespace
// Otherwise you need to import the constants at the module level,
// here it would be "use generated::settings::*"
#[const_init(import_path = generated::settings)]
struct FooBar {
    // Without attribute, looking for matching uppercase field name, here "FOO"
    foo: bool,
    bar: isize,
    // With attribute, it specifies a constant expr that will be assigned
    #[const_init(value = a::B)]
    b: [isize; 3],
    #[const_init(value = 3.14)]
    c: f64,
    #[const_init(value = a::D)]
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

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

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
