use const_init::ConstInit;

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = true)]
    foo: bool,
    bar: usize,
}

fn main() {}
