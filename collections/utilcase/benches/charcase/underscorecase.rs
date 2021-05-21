use charcase::uppercase::is_uppercase;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_uppercase_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::uppercase::is_uppercase", |b| {
    b.iter(|| is_uppercase(black_box('_')))
  });
}

criterion_group!(benches, is_uppercase_benchmark);
criterion_main!(benches);
