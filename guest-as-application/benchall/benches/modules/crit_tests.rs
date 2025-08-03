use criterion::{ Criterion };

pub fn criterion_benchmark(c: &mut Criterion) {
    let (_wamr_runtime, _wamr_module, wamr_instance) = wamr_impl
        ::setup_runtime()
        .expect("[BENCH:crit] Wamr runtime setup failed");
    c.bench_function("WAMR Ping Pong", |b| {
        b.iter(|| {
            std::hint::black_box(
                wamr_impl::run_pingpong(&wamr_instance).expect("[BENCH:crit] Wamr pingpong failed")
            )
        })
    });

    let mut native_hw = native_impl::setup();
    c.bench_function("Native Ping Pong", |b| {
        b.iter(|| { std::hint::black_box(native_impl::pingpong(&mut native_hw)) })
    });

    let (wasmtime_instance, mut wasmtime_store) = wasmtime_impl
        ::setup_runtime()
        .expect("[BENCH:crit] Wasmtime runtime setup failed");
    c.bench_function("Wasmtime Ping Pong", |b| {
        b.iter(|| {
            std::hint::black_box(
                wasmtime_impl::run_pingpong(&wasmtime_instance, &mut wasmtime_store)
            )
        })
    });
}
