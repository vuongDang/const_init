mod utils;
use utils::generated_settings::*;
use utils::shared::FooBar;

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

    if FOO_BAR.foo && FOO_BAR.bar == BAR && FOO_BAR.b == B && FOO_BAR.c == C && FOO_BAR.d == D {
        // Should be kept by compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
