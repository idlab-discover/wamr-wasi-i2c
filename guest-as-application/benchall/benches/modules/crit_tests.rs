use criterion::{ BatchSize, Criterion };

pub fn bench_setup_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Runtime Setup");

    group.bench_function("Native", |b| {
        b.iter(native_impl::setup);
    });

    group.bench_function("WAMR", |b| {
        b.iter(|| {
            let (_rt, _mod, _instance, _f) = wamr_impl
                ::setup_runtime()
                .expect("[BENCH:crit] Wamr Setup Failed");
        });
    });

    group.bench_function("Wasmtime", |b| {
        b.iter(wasmtime_impl::setup_runtime);
    });

    group.finish();
}

pub fn bench_cold_pingpong_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Ping Pong Execution");

    group.bench_function("Native", |b| {
        b.iter_batched(
            native_impl::setup,
            |mut dev| {
                native_impl::pingpong(&mut dev);
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("WAMR", |b| {
        b.iter_batched(
            || wamr_impl::setup_runtime().expect("[BENCH:crit] Wamr Setup Failed"),
            |(_rt, _mod, inst, f)| {
                let _ = wamr_impl::run_pingpong(&inst, &f);
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("Wasmtime", |b| {
        b.iter_batched(
            || wasmtime_impl::setup_runtime().expect("[BENCH:crit] Wasmtime Setup Failed"),
            |(inst, mut store)| {
                let _ = wasmtime_impl::run_pingpong(&inst, &mut store);
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

pub fn bench_hot_pingpong_comparison(c: &mut Criterion) {
    let mut native_hw = native_impl::setup();
    c.bench_function("Native Ping Pong", |b| {
        b.iter(|| { std::hint::black_box(native_impl::pingpong(&mut native_hw)) })
    });

    let (_rt, _mod, wamr_instance, func) = wamr_impl
        ::setup_runtime()
        .expect("[BENCH:crit] Wamr runtime setup failed");
    c.bench_function("WAMR Ping Pong", |b| {
        b.iter(|| {
            std::hint::black_box(
                wamr_impl
                    ::run_pingpong(&wamr_instance, &func)
                    .expect("[BENCH:crit] Wamr pingpong failed")
            )
        })
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
