mod modules;
use criterion::{Criterion, criterion_group, criterion_main};
use modules::crit_tests::bench_without_startup_comparison;
use std::time::Duration;

criterion_group! {
name = benches;
config = Criterion::default().measurement_time(Duration::from_secs(110)).warm_up_time(Duration::from_secs(8));
targets = bench_without_startup_comparison}
criterion_main!(benches);
