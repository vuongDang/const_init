use settings_as_constants::*;

struct FooBar {
    foo: bool,
    bar: usize,
}

fn main() {
    println!("FOO: {FOO}");
    println!("BAR: {BAR}");

    const FOO_BAR: FooBar = FooBar { foo: FOO, bar: BAR };

    let present = "I should be present in the binary";
    let absent = "I should be absent in the binary";

    if FOO {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }
    if BAR == 1 {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }

    if FOO_BAR.foo && FOO_BAR.bar == 1 {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }
}
