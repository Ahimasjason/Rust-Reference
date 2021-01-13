extern crate criterion_ex;
use criterion::{criterion_group, criterion_main, Criterion};
use criterion_ex::*;
fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 7", |b| {
        b.iter(|| recursive_process_fibancci(7, 0, 1))
    });
}

criterion_group!(bench, fibonacci_benchmark);
criterion_main!(bench);
