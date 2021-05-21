use criterion::{black_box, criterion_group, criterion_main, Criterion};
use strcase::capitalcase::{is_capitalcase, to_capitalcase};

fn is_capitalcase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::capitalcase::is_capitalcase", |b| {
    b.iter(|| is_capitalcase(black_box('A')))
  });
}

fn to_capitalcase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::capitalcase::to_capitalcase", |b| {
    b.iter(|| to_capitalcase(black_box('A')))
  });
}

criterion_group!(benches, is_capitalcase_benchmark, to_capitalcase_benchmark);
criterion_main!(benches);
