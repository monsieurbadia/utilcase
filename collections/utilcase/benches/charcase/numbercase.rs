use charcase::numbercase::{
  is_hex_number, is_number, is_number_continue, is_number_zero,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_number_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::numbercase::is_number", |b| {
    b.iter(|| is_number(black_box('3')))
  });
}

fn is_number_zero_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::numbercase::is_number_zero", |b| {
    b.iter(|| is_number_zero(black_box('0')))
  });
}

fn is_number_continue_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::numbercase::is_number_continue", |b| {
    b.iter(|| is_number_continue(black_box('7')))
  });
}

fn is_hex_number_benchmark(c: &mut Criterion) {
  c.bench_function("charcase::numbercase::is_hex_number", |b| {
    b.iter(|| is_hex_number(black_box('f')))
  });
}

criterion_group!(
  benches,
  is_number_benchmark,
  is_number_zero_benchmark,
  is_number_continue_benchmark,
  is_hex_number_benchmark,
);

criterion_main!(benches);
