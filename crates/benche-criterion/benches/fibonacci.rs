use benche_criterion::{
    fibonacci_sequence, inline_fibonacci_sequence, inline_let_fibonacci, let_fibonacci,
    match_fibonacci, recursive_fibonacci,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("match_fibonacci 20", |b| {
        b.iter(|| match_fibonacci(black_box(20)))
    });
    c.bench_function("let_fibonacci 20", |b| {
        b.iter(|| let_fibonacci(black_box(20)))
    });
    c.bench_function("inline_let_fibonacci 20", |b| {
        b.iter(|| inline_let_fibonacci(black_box(20)))
    });
    c.bench_function("recursive_fibonacci 20", |b| {
        b.iter(|| recursive_fibonacci(black_box(20)))
    });
    c.bench_function("iterative_fibonacci 20", |b| {
        b.iter(|| fibonacci_sequence().take(black_box(20)))
    });
    c.bench_function("inline_fibonacci_sequence 20", |b| {
        b.iter(|| inline_fibonacci_sequence().take(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
