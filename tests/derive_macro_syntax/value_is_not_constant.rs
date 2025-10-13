use const_init_macros::ConstInit;

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = true)]
    foo: bool,
    #[const_init(value = Default::default())]
    bar: usize,
}

fn main() {}
