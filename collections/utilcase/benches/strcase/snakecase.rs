use strcase::snakecase::{is_snakecase, to_snakecase};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_snakecase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::snakecase::to_snakecase", |b| {
    b.iter(|| to_snakecase(black_box('3')))
  });
}

fn is_snakecase_benchmark(c: &mut Criterion) {
  c.bench_function("strcase::snakecase::is_snakecase", |b| {
    b.iter(|| is_snakecase(black_box('f')))
  });
}

criterion_group!(benches, to_snakecase_benchmark, is_snakecase_benchmark);
criterion_main!(benches);
