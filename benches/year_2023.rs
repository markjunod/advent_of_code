use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code::year_2023;

fn bench_year_2023(c: &mut Criterion) {
    c.bench_function("2023_all_days", |b| b.iter(|| year_2023::run_all()));
}

fn bench_days_in_year_2023(c: &mut Criterion) {
    c.bench_function("2023_day_1", |b| b.iter(|| year_2023::run_day(black_box(1))));
    c.bench_function("2023_day_2", |b| b.iter(|| year_2023::run_day(black_box(2))));
    c.bench_function("2023_day_3", |b| b.iter(|| year_2023::run_day(black_box(3))));
    c.bench_function("2023_day_4", |b| b.iter(|| year_2023::run_day(black_box(4))));
}

criterion_group!(run_entire_year, bench_year_2023);
criterion_group!(run_individual_days, bench_days_in_year_2023);

criterion_main!(run_entire_year, run_individual_days);
