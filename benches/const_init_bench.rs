//! We want to measure how much performance gain we can
//! get by using constant initialization

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
mod utils;
use utils::*;

fn branch_optimizations(c: &mut Criterion) {
    use no_init::*;
    // Parsing a JSON file to prevent any compiler optimization
    let foo_bar_json = FooBar::from_json_file();
    let foo_bar_constant = &FOO_BAR;
    let mut group = c.benchmark_group("Branch optimizations");
    let loop_counts = [1, 10, 20, 50, 100];
    for loop_count in loop_counts {
        group.bench_with_input(
            BenchmarkId::new("with_runtime_init_from_json", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work(&foo_bar_json, *loop_count)),
        );
        group.bench_with_input(
            BenchmarkId::new("with_runtime_init_from_constant", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work(foo_bar_constant, *loop_count)),
        );

        group.bench_with_input(
            BenchmarkId::new("with_const_init", loop_count),
            &loop_count,
            |b, loop_count| b.iter(|| work_constant(*loop_count)),
        );
    }
    group.finish();
}

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
    branch_optimizations_with_init_time
);
criterion_main!(benches);
