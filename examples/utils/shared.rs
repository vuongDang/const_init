#![allow(dead_code)]
use crate::utils::generated_settings::*;
use const_init_macros::ConstInit;
use rand::Rng;

#[inline(always)]
pub fn code_that_should_be_optimized(foo_bar: &FooBar) {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";
    if foo_bar.foo && foo_bar.bar == BAR && foo_bar.b == B && foo_bar.c == C && foo_bar.d == D {
        // Should be kept during compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}

#[derive(ConstInit)]
pub struct FooBar {
    pub foo: bool,
    pub bar: isize,
    #[const_init(value = B)]
    pub b: [isize; 3],
    #[const_init(value = C)]
    pub c: f64,
    #[const_init(value = D)]
    pub d: &'static str,
}

impl FooBar {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        FooBar {
            foo: rng.random_bool(0.5),
            bar: rng.random::<i32>() as isize,
            b: [
                rng.random::<i32>() as isize,
                rng.random::<i32>() as isize,
                rng.random::<i32>() as isize,
            ],
            c: rng.random(),
            d: "static",
        }
    }
}
