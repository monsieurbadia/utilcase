use charcase::identcase::is_ident;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_ident_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::identcase::is_ident", |b| {
    b.iter(|| is_ident(black_box('a')))
  });
}

criterion_group!(benches, is_ident_benchmark);
criterion_main!(benches);
