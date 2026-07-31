use criterion::{criterion_group, criterion_main, Criterion};
use rendering_project::xml_parser;

#[cfg(feature = "bench")]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tokenising vk.xml", |b| b.iter(|| xml_parser::benchmark_tokeniser()));
}

#[cfg(feature = "bench")]
criterion_group!(benches, criterion_benchmark);
#[cfg(feature = "bench")]
criterion_main!(benches);