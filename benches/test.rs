use criterion::{Criterion, criterion_group, criterion_main};

#[cfg(feature = "bench")]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dummy", |b| b.iter(|| ()));
}

#[cfg(feature = "bench")]
criterion_group!(benches, criterion_benchmark);
#[cfg(feature = "bench")]
criterion_main!(benches);
