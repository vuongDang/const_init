use const_init::ConstInit;
mod generated_settings;
use generated_settings::*;

fn main() {
    with_const_variables();
    with_struct();
    init_with_macro();
}

fn with_const_variables() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";
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
}

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = FOO)]
    foo: bool,
    #[const_init(value = BAR)]
    bar: usize,
}

fn with_struct() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    let foo_bar: FooBar = FooBar { foo: FOO, bar: BAR };

    if foo_bar.foo && foo_bar.bar == 1 {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }
}

fn init_with_macro() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    const FOO_BAR: FooBar = FooBar::const_init();
    if FOO_BAR.foo && FOO_BAR.bar == 1 {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }
}
