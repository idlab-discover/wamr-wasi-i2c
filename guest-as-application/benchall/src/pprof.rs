use std::fs::File;

fn wamr_pingpong() {
    let (_rt, _mod, instance, f) = wamr_impl
        ::setup_runtime()
        .expect("[BENCH:pprof] WAMR runtime setup failed");

    let _guard = pprof::ProfilerGuardBuilder
        ::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    wamr_impl::run_pingpong(&instance, &f).expect("[BENCH:pprof] WAMR pingpong failed");

    if let Ok(report) = _guard.report().build() {
        let file = File::create("wamr_flame.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}

fn wasmtime_pingpong() {
    let (instance, mut store) = wasmtime_impl
        ::setup_runtime()
        .expect("[BENCH:crit] Wasmtime runtime setup failed");

    let _guard = pprof::ProfilerGuardBuilder
        ::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    wasmtime_impl
        ::run_pingpong(&instance, &mut store)
        .expect("[BENCH:pprof] Wasmtime pingpong failed");

    if let Ok(report) = _guard.report().build() {
        let file = File::create("wasmtime_flame.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}

fn native_pingpong() {
    let mut hw = native_impl::setup();

    let _guard = pprof::ProfilerGuardBuilder
        ::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    native_impl::pingpong(&mut hw);

    if let Ok(report) = _guard.report().build() {
        let file = File::create("native_flame.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}

#[cfg(feature = "pprof-runtime")]
fn wamr_setup() {
    let guard = pprof::ProfilerGuardBuilder
        ::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();
    // TODO: Bespreek: WAMR doet iets heel vreemd: Claude (Rust Conditional Feature Compilation)
    //      Strace geeft weer dat WAMR nog vanalles aan het opzetten zou zijn wanneer we de setup zouden aanroepen via let _ = ...
    //      Dit zou zijn doordat de destructor meteen wordt opgeroepen
    let (_rt, _mod, _instance, _f) = wamr_impl
        ::setup_runtime()
        .expect("[BENCH:pprof] WAMR runtime setup failed");
}

#[cfg(feature = "pprof-runtime")]
fn wasmtime_setup() {
    let guard = pprof::ProfilerGuardBuilder
        ::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();
    let (_rt, _mod) = wasmtime_impl
        ::setup_runtime()
        .expect("[BENCH:pprof] Wasmtime runtime setup failed");
}

#[cfg(feature = "pprof-runtime")]
fn native_setup() {
    let guard = pprof::ProfilerGuardBuilder
        ::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();
    let _ = native_impl::setup();
}

fn main() {
    #[cfg(feature = "pprof-runtime")]
    std::hint::black_box({
        native_setup();
        wamr_setup();
        wasmtime_setup();
    });

    std::hint::black_box({
        native_pingpong();
        wamr_pingpong();
        wasmtime_pingpong();
    });
}
