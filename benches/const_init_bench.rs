//! We want to measure how much performance gain we can
//! get by using constant initialization

use criterion::{
    AxisScale, BenchmarkId, Criterion, PlotConfiguration, criterion_group, criterion_main,
};
use utils::*;
mod generated_settings;

// Benchmarks where initialization of data is omitted
fn branch_optimizations(c: &mut Criterion) {
    use no_init::*;
    // Parsing a JSON file to prevent any compiler optimization
    let foo_bar_json = FooBar::from_json_file();
    let foo_bar_constant = &FOO_BAR;

    // Y scale axis
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Branch optimizations");
    group.plot_config(plot_config);
    // let loop_counts = [1, 10, 20, 50, 100];
    // We make a handmade logarithmic scale
    let loop_counts = 0..3u32;
    for loop_count in loop_counts.into_iter() {
        group.bench_with_input(
            BenchmarkId::new("with_runtime_init_from_json", &loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work(&foo_bar_json, 10_u32.pow(*loop_count))),
        );
        group.bench_with_input(
            BenchmarkId::new("with_runtime_init_from_constant", &loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work(foo_bar_constant, 10_u32.pow(*loop_count))),
        );

        group.bench_with_input(
            BenchmarkId::new("with_const_init", &loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work_constant(10_u32.pow(*loop_count))),
        );
    }
    group.finish();
}

// Benchmarks where initialization of data is included
fn branch_optimizations_with_init_time(c: &mut Criterion) {
    use with_init::*;
    // Parsing a JSON file to prevent any compiler optimization
    let mut group = c.benchmark_group("Branch optimizations with init time");
    let loop_counts = [1, 10, 20, 50, 100];
    for loop_count in loop_counts {
        group.bench_with_input(
            BenchmarkId::new("with_runtime_init_from_json", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work_init_from_json(*loop_count)),
        );
        group.bench_with_input(
            BenchmarkId::new("with_runtime_init_from_constant", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work_init_ref_constant(*loop_count)),
        );

        group.bench_with_input(
            BenchmarkId::new("with_const_init", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| no_init::work_constant(*loop_count)),
        );
    }
    group.finish();
}

// Boilerplate that generates a main() for Criterion
criterion_group!(
    benches,
    branch_optimizations,
    // branch_optimizations_with_init_time
);
criterion_main!(benches);

mod utils {
    use super::generated_settings::*;
    use const_init_macros::ConstInit;
    use serde::Deserialize;
    use std::hint::black_box;
    use std::ops::Deref;

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
            if foo_bar.foo
                && foo_bar.bar == BAR
                && foo_bar.b == B
                && foo_bar.c == C
                && foo_bar.d == D
            {
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
            if FOO_BAR.foo
                && FOO_BAR.bar == BAR
                && FOO_BAR.b == B
                && FOO_BAR.c == C
                && FOO_BAR.d == D
            {
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

        #[unsafe(no_mangle)]
        #[inline(never)]
        pub fn work_init_from_json(loop_count: u32) -> isize {
            let foo_bar = FooBar::from_json_file();
            let mut res = 0;
            // I think the testcase is too quick to have precise measurements,
            // we try to repeat the work 1000 times to smooth the imprecision
            // for _ in 0..1000 {
            // This condition is always true and should be optimized by the compiler
            if foo_bar.foo
                && foo_bar.bar == BAR
                && foo_bar.b == B
                && foo_bar.c == C
                && foo_bar.d == D
            {
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

        #[unsafe(no_mangle)]
        #[inline(never)]
        pub fn work_init_ref_constant(loop_count: u32) -> isize {
            let foo_bar = &FOO_BAR;
            let mut res = 0;
            // I think the testcase is too quick to have precise measurements,
            // we try to repeat the work 1000 times to smooth the imprecision
            // for _ in 0..1000 {
            // This condition is always true and should be optimized by the compiler
            if foo_bar.foo
                && foo_bar.bar == BAR
                && foo_bar.b == B
                && foo_bar.c == C
                && foo_bar.d == D
            {
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
}
