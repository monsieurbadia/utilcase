use criterion::{black_box, criterion_group, criterion_main, Criterion};
use strcase::camelcase::{is_camelcase, to_camelcase};

fn is_camelcase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::camelcase::is_camelcase", |b| {
    b.iter(|| is_camelcase(black_box('A')))
  });
}

fn to_camelcase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::camelcase::to_camelcase", |b| {
    b.iter(|| to_camelcase(black_box('A')))
  });
}

criterion_group!(benches, is_camelcase_benchmark, to_camelcase_benchmark);
criterion_main!(benches);
