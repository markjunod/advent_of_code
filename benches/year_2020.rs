use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code::year_2020;

fn bench_year_2020(c: &mut Criterion) {
    c.bench_function("2020_all_days", |b| b.iter(|| year_2020::run_all()));
}

fn bench_days_in_year_2020(c: &mut Criterion) {
    c.bench_function("2020_day_1", |b| b.iter(|| year_2020::run_day(black_box(1))));
    c.bench_function("2020_day_2", |b| b.iter(|| year_2020::run_day(black_box(2))));
    c.bench_function("2020_day_3", |b| b.iter(|| year_2020::run_day(black_box(3))));
    c.bench_function("2020_day_4", |b| b.iter(|| year_2020::run_day(black_box(4))));
    c.bench_function("2020_day_5", |b| b.iter(|| year_2020::run_day(black_box(5))));
    c.bench_function("2020_day_6", |b| b.iter(|| year_2020::run_day(black_box(6))));
}

criterion_group!(run_entire_year, bench_year_2020);
criterion_group!(run_individual_days, bench_days_in_year_2020);

criterion_main!(run_entire_year, run_individual_days);
