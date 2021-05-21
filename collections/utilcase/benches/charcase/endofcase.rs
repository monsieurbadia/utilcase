use charcase::endofcase::{is_end_of_file, is_end_of_line};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_end_of_file_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::endofcase::is_end_of_file", |b| {
    b.iter(|| is_end_of_file(black_box('\0')))
  });
}

fn is_end_of_line_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::endofcase::is_end_of_line", |b| {
    b.iter(|| is_end_of_line(black_box('\n')))
  });
}

criterion_group!(benches, is_end_of_file_benchmark, is_end_of_line_benchmark);
criterion_main!(benches);
