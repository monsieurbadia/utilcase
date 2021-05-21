use charcase::quotecase::{is_double_quote, is_quote, is_single_quote};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_quote_single_quote_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::quotecase::is_quote", |b| {
    b.iter(|| is_quote(black_box('\'')))
  });
}

fn is_quote_double_quote_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::quotecase::is_quote", |b| {
    b.iter(|| is_quote(black_box('"')))
  });
}

fn is_single_quote_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::quotecase::is_single_quote", |b| {
    b.iter(|| is_single_quote(black_box('\'')))
  });
}

fn is_double_quote_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::quotecase::is_double_quote", |b| {
    b.iter(|| is_double_quote(black_box('"')))
  });
}

criterion_group!(
  benches,
  is_quote_single_quote_benchmark,
  is_quote_double_quote_benchmark,
  is_single_quote_benchmark,
  is_double_quote_benchmark,
);

criterion_main!(benches);
