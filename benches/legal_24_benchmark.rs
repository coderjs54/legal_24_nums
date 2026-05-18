use criterion::{black_box, criterion_group, criterion_main, Criterion};
use legal_24_nums::get_all_legal_nums;

fn bench_get_all_legal_nums_1_10(c: &mut Criterion) {
    c.bench_function("get_all_legal_nums 1-10", |b| {
        b.iter(|| get_all_legal_nums(black_box(1), black_box(10)))
    });
}

fn bench_get_all_legal_nums_1_13(c: &mut Criterion) {
    c.bench_function("get_all_legal_nums 1-13", |b| {
        b.iter(|| get_all_legal_nums(black_box(1), black_box(13)))
    });
}

fn bench_get_all_legal_nums_1_15(c: &mut Criterion) {
    c.bench_function("get_all_legal_nums 1-15", |b| {
        b.iter(|| get_all_legal_nums(black_box(1), black_box(15)))
    });
}

criterion_group!(
    benches,
    bench_get_all_legal_nums_1_10,
    bench_get_all_legal_nums_1_13,
    bench_get_all_legal_nums_1_15
);
criterion_main!(benches);
