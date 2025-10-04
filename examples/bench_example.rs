//! We want to have an example that illustrate the performance gain
//! of our work

mod generated;
use const_init_macros::ConstInit;
use generated::settings::*;
use serde::Deserialize;
use std::{hint::black_box, path::PathBuf};

#[derive(ConstInit, Deserialize)]
struct FooBar {
    #[const_init(value = FOO)]
    foo: bool,
    #[const_init(value = BAR)]
    bar: isize,
    #[const_init(value = B)]
    b: [isize; 3],
    #[const_init(value = C)]
    c: f64,
    #[const_init(value = D)]
    #[serde(skip, default = "get_d")]
    d: &'static str,
}

const fn get_d() -> &'static str {
    D
}

// We stimulate the fact that init values aren't
// known at compile time with `black_box`
// We tried to do it with `black_box` hint before
// it still got optimized
fn runtime_init() -> FooBar {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // We read the settings from "settings.json" file
    let json_file: PathBuf = [&manifest_path, "settings.json"].iter().collect();
    let json = std::fs::read_to_string(json_file).unwrap();
    let res: FooBar = serde_json::from_str(&json).unwrap();
    res
}

// The branch in this example only contains conditional on constants
#[unsafe(no_mangle)]
#[inline(never)]
fn work(foo_bar: &FooBar, loop_count: u32) -> isize {
    let mut res = 0;
    // I think the testcase is too quick to have precise measurements,
    // we try to repeat the work 1000 times to smooth the imprecision
    for _ in 0..1000 {
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
    }
    res
}

// The branch in this example only contains conditional on constants
#[unsafe(no_mangle)]
#[inline(never)]
fn work_with_constant(loop_count: u32) -> isize {
    const FOO_BAR: FooBar = FooBar::const_init();
    let mut res = 0;
    // I think the testcase is too quick to have precise measurements,
    // we try to repeat the work 1000 times to smooth the imprecision
    for _ in 0..1000 {
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
    }
    res
}

fn main() {
    let foo_bar = runtime_init();
    let loop_count = 1;

    let const_init = work_with_constant(loop_count);
    let runtime_init = work(&foo_bar, loop_count);

    println!("{}", const_init + runtime_init)
}
