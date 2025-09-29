mod generated;
use generated::settings::*;

struct FooBar {
    foo: bool,
    bar: usize,
}

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    let foo_bar: FooBar = FooBar { foo: FOO, bar: BAR };

    if foo_bar.foo && foo_bar.bar == 1 {
        // Should be kept by compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
