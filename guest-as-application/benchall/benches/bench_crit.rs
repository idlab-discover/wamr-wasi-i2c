mod crit_tests;

use crit_tests::criterion_benchmark;
use criterion::{ criterion_group, criterion_main };

/* fn full_run() {
    let runtime = wamr_manager::setup_runtime().expect("Bench: Full: Runtime Failed");
    let module = wamr_manager::setup_module(&runtime).expect("Bench: Full: Module Failed");
    let instance = wamr_manager
        ::setup_module_instance(&runtime, &module)
        .expect("Bench: Full: Instance Failed");
    wamr_manager::run_guest_function(&instance).expect("Bench: Full: Function Failed");
}

fn criterion_benchmark(c: &mut Criterion) {
    let func = black_box(full_run);
    c.bench_function("I2C Ping Pong", |b| b.iter(func));
} */

criterion_group!(benches, criterion_benchmark);
/* criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler();
    targets = criterion_benchmark,
} */
criterion_main!(benches);
