use criterion::{black_box, criterion_group, criterion_main, Criterion};
use strcase::uppercase::{is_uppercase, to_uppercase};

fn is_uppercase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::uppercase::is_uppercase", |b| {
    b.iter(|| is_uppercase(black_box('A')))
  });
}

fn to_uppercase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::uppercase::to_uppercase", |b| {
    b.iter(|| to_uppercase(black_box('A')))
  });
}

criterion_group!(benches, is_uppercase_benchmark, to_uppercase_benchmark);
criterion_main!(benches);
