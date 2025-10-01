//! We want to measure how much performance gain we can
//! get by using constant initialization
use const_init_macros::ConstInit;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const FOO: bool = true;
const BAR: isize = -34;
const A: [isize; 3] = [0, 1, 2];
const B: f64 = 3.14;
const C: &'static str = "ding!";

#[derive(ConstInit)]
struct FooBar {
    #[const_init(value = FOO)]
    foo: bool,
    #[const_init(value = BAR)]
    bar: isize,
    #[const_init(value = A)]
    a: [isize; 3],
    #[const_init(value = B)]
    b: f64,
    #[const_init(value = C)]
    c: &'static str,
}

// We stimulate the fact that init values aren't
// known at compile time with `black_box`
fn runtime_init() -> FooBar {
    FooBar {
        foo: black_box(FOO),
        bar: black_box(BAR),
        a: black_box(A),
        b: black_box(B),
        c: black_box(C),
    }
}

// The branch in this example only contains conditional on constants
fn work(foo_bar: &FooBar, loop_count: u32) -> isize {
    let mut res = 0;
    if foo_bar.foo && foo_bar.bar == BAR && foo_bar.a == A && foo_bar.b == B && foo_bar.c == C
    // This condition is always true
    {
        // Spin loop to be able to control the amount of
        // time spent in the branch
        for _ in 0..loop_count {
            // black_box to avoid loop optimizations
            res += black_box(1);
        }
    }
    res
}

fn branch_optimizations(c: &mut Criterion) {
    const FOO_BAR: FooBar = FooBar::const_init();
    let foo_bar = runtime_init();
    let mut group = c.benchmark_group("Branch optimizations");
    let loop_counts = [1, 10, 100, 1000, 2000, 5000, 10000];
    for loop_count in loop_counts {
        group.bench_with_input(
            BenchmarkId::new("const_init", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work(&FOO_BAR, *loop_count)),
        );

        group.bench_with_input(
            BenchmarkId::new("runtime_init", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work(&foo_bar, *loop_count)),
        );
    }
    group.finish();
}

// Boilerplate that generates a main() for Criterion
criterion_group!(benches, branch_optimizations);
criterion_main!(benches);
