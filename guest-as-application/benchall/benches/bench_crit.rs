mod modules;
use modules::crit_tests::criterion_benchmark;
use criterion::{ criterion_group, criterion_main, Criterion };
use std::time::Duration;

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark,
}
criterion_main!(benches);
