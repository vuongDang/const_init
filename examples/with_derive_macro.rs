mod generated;
use const_init_macros::ConstInit;
use generated::settings::*;

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = FOO)]
    foo: bool,
    #[const_init(value = BAR)]
    bar: isize,
    #[const_init(value = a::B)]
    b: [isize; 3],
    #[const_init(value = a::C)]
    c: f64,
    #[const_init(value = a::D)]
    d: &'static str,
}

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
