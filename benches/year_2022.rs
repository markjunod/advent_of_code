use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code::year_2022;

fn bench_year_2022(c: &mut Criterion) {
    c.bench_function("2022_all_days", |b| b.iter(|| year_2022::run_all()));
}

fn bench_days_in_year_2022(c: &mut Criterion) {
    c.bench_function("2022_day_1", |b| b.iter(|| year_2022::run_day(black_box(1))));
    c.bench_function("2022_day_2", |b| b.iter(|| year_2022::run_day(black_box(2))));
}

criterion_group!(run_entire_year, bench_year_2022);
criterion_group!(run_individual_days, bench_days_in_year_2022);

criterion_main!(run_entire_year, run_individual_days);
