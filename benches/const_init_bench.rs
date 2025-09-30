//! We want to measure how much performance gain we can
//! get by using constant initialization
use const_init_macros::ConstInit;
use criterion::{Criterion, criterion_group, criterion_main};
mod generated;
use generated::settings::*;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::LazyLock;

#[derive(ConstInit, Deserialize)]
struct FooBar {
    #[const_init(value = FOO)]
    foo: bool,
    #[const_init(value = BAR)]
    bar: isize,
    #[const_init(value = a::B)]
    b: [isize; 3],
    #[const_init(value = a::C)]
    c: f64,
    #[const_init(value = a::D)]
    d: &'static str,
}

static JSON: LazyLock<String> = LazyLock::new(|| {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // We read the settings from "settings.json" file
    let json_path: PathBuf = [&manifest_path, "settings.json"].iter().collect();
    std::fs::read_to_string(json_path).unwrap()
});

fn runtime_init() -> FooBar {
    serde_json::from_str(&*JSON).unwrap()
}

// The branch in this example only contains conditional on constants
fn work_branch_full_constants(foo_bar: &FooBar, acc: isize) -> isize {
    if acc == 0 {
        return acc;
    }
    if foo_bar.foo
        && foo_bar.bar == 1
        && foo_bar.b == [1, 2, -3]
        && foo_bar.c == 3.14
        && foo_bar.d == "ding!"
    {
        // This is always true
        work_branch_full_constants(foo_bar, acc - 1)
    } else {
        // This should not happen
        work_branch_full_constants(foo_bar, acc + 1)
    }
}

fn bench_branches_full_constants(c: &mut Criterion) {
    const FOO_BAR: FooBar = FooBar::const_init();
    let foo_bar = runtime_init();
    let mut group = c.benchmark_group("Full constants branch");
    group.bench_function("with const init", |b| {
        b.iter(|| work_branch_full_constants(&FOO_BAR, 10))
    });
    group.bench_function("with runtime init", |b| {
        b.iter(|| work_branch_full_constants(&foo_bar, 10))
    });
    group.finish();
}

// Boilerplate that generates a main() for Criterion
criterion_group!(benches, bench_branches_full_constants);
criterion_main!(benches);
