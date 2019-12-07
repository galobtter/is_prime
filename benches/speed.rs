use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use is_prime::is_prime;

pub fn criterion_benchmark(c: &mut Criterion) {
    let num: u64 = 274876858369;
    c.bench_with_input(BenchmarkId::new("test_2", num), &num, |b, &s| {
        b.iter(|| is_prime(s));
    });
    let num: u64 = 1000123457689;
    c.bench_with_input(BenchmarkId::new("test_3", num), &num, |b, &s| {
        b.iter(|| is_prime(s));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
