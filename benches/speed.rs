use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use is_prime::is_prime;

pub fn criterion_benchmark(c: &mut Criterion) {
    let num: u128 = 2147483647;
    c.bench_with_input(BenchmarkId::new("input_example", num), &num, |b, &s| {
        b.iter(|| is_prime(s));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
