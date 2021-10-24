use criterion::{criterion_group, criterion_main, Criterion};
use lucky_minesweeper::*;

fn bench_all(c: &mut Criterion) {
    c.bench_function("Beginner", |b| {
        b.iter(|| test_single_threaded::<9, 9, 10, 10_000, 0>())
    });
    c.bench_function("Intermediate", |b| {
        b.iter(|| test_single_threaded::<16, 16, 40, 10_000, 0>())
    });
    c.bench_function("Advanced", |b| {
        b.iter(|| test_single_threaded::<30, 16, 99, 5_000, 0>())
    });
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
