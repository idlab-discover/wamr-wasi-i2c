use criterion::{ BatchSize, Criterion };

pub fn bench_setup_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Runtime Setup");

    group.bench_function("Native", |b| {
        b.iter(|| {
            let _test = native_impl::setup();
        });
    });

    group.bench_function("WAMR", |b| {
        b.iter(|| {
            let _test = wamr_impl::PingPongRunner::new();
        });
    });

    group.bench_function("Wasmtime", |b| {
        b.iter(|| {
            let _test = wasmtime_impl::PingPongRunner::new();
        });
    });

    group.finish();
}

pub fn bench_cold_pingpong_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cold Ping Pong Execution");

    group.bench_function("Native", |b| {
        b.iter_batched(
            || { native_impl::setup() },
            |mut dev| {
                native_impl::pingpong(&mut dev);
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("WAMR", |b| {
        b.iter_batched(
            || { wamr_impl::PingPongRunner::new().unwrap() },
            |runner| {
                runner.pingpong();
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("Wasmtime", |b| {
        b.iter_batched(
            || { wasmtime_impl::PingPongRunner::new().unwrap() },
            |mut runner| {
                runner.pingpong();
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

pub fn bench_hot_pingpong_comparison(c: &mut Criterion) {
    let mut native_hw = native_impl::setup();
    let wamr_runner = wamr_impl::PingPongRunner::new().unwrap();
    let mut wasmtime_runner = wasmtime_impl::PingPongRunner::new().unwrap();

    let mut group = c.benchmark_group("Hot Ping Pong Execution");

    group.bench_function("Native", |b| { b.iter(|| { native_impl::pingpong(&mut native_hw) }) });
    group.bench_function("WAMR", |b| {
        b.iter(|| {
            wamr_runner.pingpong();
        })
    });
    group.bench_function("Wasmtime", |b| {
        b.iter(|| {
            wasmtime_runner.pingpong();
        })
    });

    group.finish();
}
