use charcase::spacecase::is_whitespace;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_whitespace_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::spacecase::is_whitespace", |b| {
    b.iter(|| is_whitespace(black_box(' ')))
  });
}

criterion_group!(benches, is_whitespace_benchmark);
criterion_main!(benches);
