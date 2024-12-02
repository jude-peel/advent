use criterion::{criterion_group, criterion_main, Criterion};
use rust::{day_1::LocationList, day_2::Reports};

#[inline]
fn day_1() {
    let list = LocationList::build().unwrap();

    let distance = list.get_total_distance();
    let similarity = list.get_similarity();

    assert_eq!(distance, 1530215);
    assert_eq!(similarity, 26800609);
}

#[inline]
fn day_2() {
    let mut reports = Reports::build().unwrap();

    assert_eq!(reports.get_safe_sum(), 334);
    assert_eq!(reports.get_fixable_sum(), 400);
}

#[allow(clippy::redundant_closure)]
fn benchmark_all_days(c: &mut Criterion) {
    //c.bench_function("Day 1:", |b| b.iter(|| day_1()));
    c.bench_function("Day 2:", |b| b.iter(|| day_2()));
}

criterion_group!(benches, benchmark_all_days);
criterion_main!(benches);
