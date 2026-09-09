use criterion::{Criterion, criterion_group, criterion_main};
use xml_parser::text_parser;

#[cfg(feature = "bench")]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tokenising vk.xml", |b| {
        b.iter(|| text_parser::benchmark_tokeniser())
    });
    c.bench_function("parsing vk.xml", |b| {
        b.iter(|| text_parser::benchmark_parser())
    });
}

#[cfg(feature = "bench")]
criterion_group!(benches, criterion_benchmark);
#[cfg(feature = "bench")]
criterion_main!(benches);
