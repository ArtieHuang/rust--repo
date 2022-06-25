// criterion_demo/benches/fibonacci.rs

#[macro_use]
extern crate criterion;

extern crate criterion_demo;

use criterion::Criterion;

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i,
                               |b, i| b.iter(|| fibonacci_slow(*i)));
        group.bench_with_input(BenchmarkId::new("Iterative", i), i,
                               |b, i| b.iter(|| fibonacci_fast(*i)));
    }
    group.finish();
}

