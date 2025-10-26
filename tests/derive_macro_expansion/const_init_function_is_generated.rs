use const_init_macros::ConstInit;

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = true)]
    foo: bool,
    #[const_init(value = 3)]
    bar: usize,
}

fn main() {
    println!("{}", FooBar::const_init().foo)
}
