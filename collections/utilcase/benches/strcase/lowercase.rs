use criterion::{black_box, criterion_group, criterion_main, Criterion};
use strcase::lowercase::{is_lowercase, to_lowercase};

fn is_lowercase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::lowercase::is_lowercase", |b| {
    b.iter(|| is_lowercase(black_box('A')))
  });
}

fn to_lowercase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::lowercase::to_lowercase", |b| {
    b.iter(|| to_lowercase(black_box('A')))
  });
}

criterion_group!(benches, is_lowercase_benchmark, to_lowercase_benchmark);
criterion_main!(benches);
