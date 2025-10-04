use const_init_macros::ConstInit;
use serde::Deserialize;
use std::hint::black_box;

pub const FOO: bool = true;
pub const BAR: isize = -34;
pub const B: [isize; 3] = [0, 1, 2];
pub const C: f64 = 3.14;
pub const D: &'static str = "ding!";

const fn get_d() -> &'static str {
    D
}

#[derive(ConstInit, Deserialize)]
pub struct FooBar {
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

pub const FOO_BAR: FooBar = FooBar::const_init();

impl FooBar {
    pub fn from_json_file() -> FooBar {
        let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        // We read the settings from "settings.json" file
        let json_file: std::path::PathBuf = [&manifest_path, "settings.json"].iter().collect();
        let json = std::fs::read_to_string(json_file).unwrap();
        let res: FooBar = serde_json::from_str(&json).unwrap();
        res
    }
}

pub mod no_init {
    use super::*;
    // We stimulate the fact that init values aren't
    // known at compile time with `black_box`
    // We tried to do it with `black_box` hint before
    // it still got optimized

    // Here `foo_bar` is initialized at runtime by parsing a json file, can't be optimized by the compiler
    #[unsafe(no_mangle)]
    #[inline(never)]
    pub fn work(foo_bar: &FooBar, loop_count: u32) -> isize {
        let mut res = 0;
        // I think the testcase is too quick to have precise measurements,
        // we try to repeat the work 1000 times to smooth the imprecision
        // for _ in 0..1000 {
        // This condition is always true and can get optimized by CPU branch prediction
        if foo_bar.foo && foo_bar.bar == BAR && foo_bar.b == B && foo_bar.c == C && foo_bar.d == D {
            // Spin loop to be able to control the amount of
            // time spent in the branch
            for _ in 0..loop_count {
                // black_box to avoid loop optimizations
                res = black_box(res + foo_bar.bar);
            }
        }
        // }
        res
    }

    // This version of `work` uses a constant for value of `foo_bar`
    #[unsafe(no_mangle)]
    #[inline(never)]
    pub fn work_constant(loop_count: u32) -> isize {
        let mut res = 0;
        // I think the testcase is too quick to have precise measurements,
        // we try to repeat the work 1000 times to smooth the imprecision
        // for _ in 0..1000 {
        // This condition is always true and should be optimized by the compiler
        if FOO_BAR.foo && FOO_BAR.bar == BAR && FOO_BAR.b == B && FOO_BAR.c == C && FOO_BAR.d == D {
            // Spin loop to be able to control the amount of
            // time spent in the branch
            for _ in 0..loop_count {
                // black_box to avoid loop optimizations
                res = black_box(res + FOO_BAR.bar);
            }
        }
        // }
        res
    }
}

pub mod with_init {
    use super::*;

    pub fn work_init_from_json(loop_count: u32) -> isize {
        let foo_bar = FooBar::from_json_file();
        let mut res = 0;
        // I think the testcase is too quick to have precise measurements,
        // we try to repeat the work 1000 times to smooth the imprecision
        // for _ in 0..1000 {
        // This condition is always true and should be optimized by the compiler
        if foo_bar.foo && foo_bar.bar == BAR && foo_bar.b == B && foo_bar.c == C && foo_bar.d == D {
            // Spin loop to be able to control the amount of
            // time spent in the branch
            for _ in 0..loop_count {
                // black_box to avoid loop optimizations
                res = black_box(res + foo_bar.bar);
            }
        }
        // }
        res
    }

    pub fn work_init_ref_constant(loop_count: u32) -> isize {
        let foo_bar = &FOO_BAR;
        let mut res = 0;
        // I think the testcase is too quick to have precise measurements,
        // we try to repeat the work 1000 times to smooth the imprecision
        // for _ in 0..1000 {
        // This condition is always true and should be optimized by the compiler
        if foo_bar.foo && foo_bar.bar == BAR && foo_bar.b == B && foo_bar.c == C && foo_bar.d == D {
            // Spin loop to be able to control the amount of
            // time spent in the branch
            for _ in 0..loop_count {
                // black_box to avoid loop optimizations
                res = black_box(res + foo_bar.bar);
            }
        }
        // }
        res
    }
}
