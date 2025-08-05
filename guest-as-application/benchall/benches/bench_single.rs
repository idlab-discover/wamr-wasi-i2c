mod modules;

use modules::crit_singles::bench_setup;

use criterion::{ criterion_group, criterion_main, Criterion };
use std::time::Duration;

criterion_group! {
	name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30)).warm_up_time(Duration::from_secs(8));
    targets = bench_setup,
}

criterion_main!(benches);
