use std::hint::black_box;
use criterion::{ criterion_group, criterion_main, Criterion };
use host::wamr_manager;

fn criterion_benchmark(c: &mut Criterion) {
    let fntest = black_box(wamr_manager::setup_runtime);
    c.bench_function("fib 20", |b| b.iter(|| fntest()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
