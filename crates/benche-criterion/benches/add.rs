use benche_criterion::{add, inline_add};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn add_benchmark(c: &mut Criterion) {
    c.bench_function("add 2 + 3", |b| b.iter(|| add(black_box(2), black_box(3))));
    c.bench_function("inline_add 2 + 3", |b| {
        b.iter(|| inline_add(black_box(2), black_box(3)))
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
