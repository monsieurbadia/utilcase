use charcase::groupcase::is_group;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_group_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::groupcase::is_group", |b| {
    b.iter(|| is_group(black_box('(')))
  });
}

criterion_group!(benches, is_group_benchmark);
criterion_main!(benches);
