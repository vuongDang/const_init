use const_init_macros::ConstInit;

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = true)]
    foo: bool,
    #[const_init(val = 3)]
    bar: usize,
}

fn main() {}
