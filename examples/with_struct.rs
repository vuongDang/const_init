mod generated;
use generated::settings::*;

struct FooBar {
    foo: bool,
    bar: isize,
    b: [isize; 3],
    c: f64,
    d: &'static str,
}

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    // Note: funnily when declaring FOO_BAR with a "let" instead of "const" it
    // worked with fewer fields but at some point when the branch conditions got
    // complicated it didn't get optimized anymore
    const FOO_BAR: FooBar = FooBar {
        foo: FOO,
        bar: BAR,
        b: a::B,
        c: a::C,
        d: a::D,
    };

    if FOO_BAR.foo
        && FOO_BAR.bar == 1
        && FOO_BAR.b == [1, 2, -3]
        && FOO_BAR.c == 3.14
        && FOO_BAR.d == "ding!"
    {
        // Should be kept by compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
