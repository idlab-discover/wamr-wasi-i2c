mod modules;
use modules::crit_tests::{
    bench_hot_pingpong_comparison,
    bench_cold_pingpong_comparison,
    bench_setup_comparison,
};
use criterion::{ criterion_group, criterion_main, Criterion };
use std::time::Duration;

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(110)).warm_up_time(Duration::from_secs(8));
    targets = bench_setup_comparison, bench_cold_pingpong_comparison, bench_hot_pingpong_comparison
}
criterion_main!(benches);
