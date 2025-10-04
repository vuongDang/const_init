# Benchmarks

We want to measure the impact of using our crate on performance.

## Setup

- OS: Windows 10 Windows 10.0.26100 x86_64 (X64)
- RAM: 32G
- CPU: AMD Ryzen 5 5600X 6-Core Processor (3.70 GHz)
- rustc: 1.90.0 (1159e78c4 2025-09-14)
- cargo: 1.90.0 (840b83a10 2025-07-30)
- bench framework: Criterion

## Bench code

In the `benches/` directory

## Bench description

We are measuring the impact of being able to optimize the branch optimizations from the compiler.

For this our bench function just contains a branch which checks the fields of our test value.
The test value is obtained by 3 different manners:
- parsed from a JSON file (`with_json_parsing`)
- initialized at build time using our crate (`with_const_init`)
- is a reference to the constant obtained with our crate but the compiler can't guess it and optimize the branches (`with_runtime_init`)

The branch check is actually always true but if the test value is constant the
compiler optimizes these checks away and should be faster.
Finally we test with different inputs which just increases how much work is done
after passing the branch check.

### Measuring with initialization time

![bench_with_init_time](assets/bench_with_init_time.svg)

We can easily see that this bench is skewed by the initialization time of our struct.
Parsing from a JSON file is slow and any compiler optimizations gain/loss are negligible
compared to the parsing phase.

### Measuring without initialization time

This time we remove the initialization phase from our benchmarks

![bench_with_init_time](assets/bench_without_init_time.svg)

What we can observe:
1. `with_const_init` is slightly faster than `with_runtime_init` but not by much
2. `with_json_parsing` has constant performance regardless of the input

#### How come `with_json_parsing` is so efficient?

I don't have a definite answer but what we know is this:
- `with_const_init` is store in read-only section `.rodata` of the binary
- constant values can be stored directly in the instructions
- `with_json_parsing` is stored in RAM and freshly computed

Hypothesis:
- json parsed value is in cache so it's accessed super fast in memory
- instructions with constant values are bigger and may produce more cache spilling/misses

## Conclusion

On our small benchmarks, it seems like producing build-time values main benefits
come from the fast initialization time.
From what we've seen branch optimizations make the code faster but not by a big margin
and a lot of performance comes from how the CPU works under the hood. CPU behavior
can be hard to predict and depends on what chip you're using so it requires more in-depth
expertise to give any advice on this.
