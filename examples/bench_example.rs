//! Our bench translated into an example to help us debug the bench

mod utils;
use std::hint::black_box;
use utils::generated_settings::*;
use utils::shared::*;

// The branch in this example only contains conditional on constants
#[unsafe(no_mangle)]
#[inline(never)]
fn work(foo_bar: &FooBar, loop_count: u32) -> isize {
    let mut res = 0;
    // I think the testcase is too quick to have precise measurements,
    // we try to repeat the work 1000 times to smooth the imprecision
    if foo_bar.foo && foo_bar.bar == BAR && foo_bar.b == B && foo_bar.c == C && foo_bar.d == D
    // This condition is always true
    {
        // Spin loop to be able to control the amount of
        // time spent in the branch
        for _ in 0..loop_count {
            // black_box to avoid loop optimizations
            res = black_box(foo_bar.bar + res);
        }
    }
    res
}

// The branch in this example only contains conditional on constants
#[unsafe(no_mangle)]
#[inline(never)]
fn work_with_constant(loop_count: u32) -> isize {
    const FOO_BAR: FooBar = FooBar::const_init();
    let mut res = 0;
    if FOO_BAR.foo && FOO_BAR.bar == BAR && FOO_BAR.b == B && FOO_BAR.c == C && FOO_BAR.d == D
    // This condition is always true
    {
        // Spin loop to be able to control the amount of
        // time spent in the branch
        for _ in 0..loop_count {
            // black_box to avoid loop optimizations
            res = black_box(FOO_BAR.bar + res);
        }
    }
    res
}

fn main() {
    let foo_bar = FooBar::random();
    let loop_count = 1;

    let const_init = work_with_constant(loop_count);
    let runtime_init = work(&foo_bar, loop_count);

    println!("{}", const_init + runtime_init)
}
