#[cfg(feature = "crit-single")]
use criterion::{ BatchSize, Criterion };

#[cfg(feature = "crit-wamr")]
pub fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("Full");
    group.sampling_mode(criterion::SamplingMode::Linear);

    // b.iter meet ook de tijd dat het neemt om de resource te droppen!
    group.bench_function("Setup", |b| {
        b.iter(|| {
            let _test = wamr_impl::PingPongRunner::new();
        });
    });

    group.bench_function("Cold Pingpong", |b| {
        b.iter_batched(
            || { wamr_impl::PingPongRunner::new().unwrap() },
            |runner| {
                runner.pingpong();
            },
            BatchSize::SmallInput
        );
    });

    let wamr_runner = wamr_impl::PingPongRunner::new().unwrap();
    group.bench_function("Hot Pingpong", |b| {
        b.iter(|| {
            wamr_runner.pingpong();
        })
    });
    group.finish();
}

#[cfg(feature = "crit-wasmtime")]
pub fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("Full");
    group.sampling_mode(criterion::SamplingMode::Linear);

    // b.iter meet ook de tijd dat het neemt om de resource te droppen!
    group.bench_function("Setup", |b| {
        b.iter(|| {
            let _test = wasmtime_impl::PingPongRunner::new();
        });
    });

    group.bench_function("Cold Pingpong", |b| {
        b.iter_batched(
            || { wasmtime_impl::PingPongRunner::new().unwrap() },
            |mut runner| {
                runner.pingpong();
            },
            BatchSize::SmallInput
        );
    });

    let mut wasmtime_runner = wasmtime_impl::PingPongRunner::new().unwrap();
    group.bench_function("Hot Pingpong", |b| {
        b.iter(|| {
            wasmtime_runner.pingpong();
        })
    });
    group.finish();
}

#[cfg(feature = "crit-native")]
pub fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("Full");
    group.sampling_mode(criterion::SamplingMode::Linear);

    // b.iter meet ook de tijd dat het neemt om de resource te droppen!
    group.bench_function("Setup", |b| {
        b.iter(|| {
            let _test = native_impl::setup();
        });
    });

    group.bench_function("Cold Pingpong", |b| {
        b.iter_batched(
            || { native_impl::setup() },
            |mut dev| {
                native_impl::pingpong(&mut dev);
            },
            BatchSize::SmallInput
        );
    });

    let mut native_hw = native_impl::setup();
    group.bench_function("Hot Pingpong", |b| {
        b.iter(|| { native_impl::pingpong(&mut native_hw) })
    });
    group.finish();
}
