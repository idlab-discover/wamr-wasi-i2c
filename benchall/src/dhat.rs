#[cfg(
    all(feature = "pprof-flamegraph", not(feature = "dhat-runtime"), not(feature = "dhat-pingpong"))
)]
use std::fs::File;

#[cfg(any(feature = "dhat-runtime", feature = "dhat-pingpong"))]
use dhat;

#[cfg(any(feature = "dhat-runtime", feature = "dhat-pingpong"))]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn wamr_pingpong() {
    let runner = wamr_impl::PingPongRunner::new().unwrap();

    #[cfg(feature = "dhat-pingpong")]
    let _profiler = dhat::Profiler::builder().file_name("wamr_pingpong.json").build();

    runner.pingpong();
}

fn wasmtime_pingpong() {
    let mut runner = wasmtime_impl::PingPongRunner::new().unwrap();

    #[cfg(feature = "dhat-pingpong")]
    let _profiler = dhat::Profiler::builder().file_name("wasmtime_pingpong.json").build();
    runner.pingpong();
}

fn native_pingpong() {
    let mut hw = native_impl::setup();

    #[cfg(feature = "dhat-pingpong")]
    let _profiler = dhat::Profiler::builder().file_name("native_pingpong.json").build();

    native_impl::pingpong(&mut hw);
}

#[cfg(feature = "dhat-runtime")]
fn wamr_setup() {
    let _profiler = dhat::Profiler::builder().file_name("wamr_setup.json").build();
    let _runner = wamr_impl::PingPongRunner::new().unwrap();
}

#[cfg(feature = "dhat-runtime")]
fn wasmtime_setup() {
    let _profiler = dhat::Profiler::builder().file_name("wasmtime_setup.json").build();
    let _runner = wasmtime_impl::PingPongRunner::new().unwrap();
}

#[cfg(feature = "dhat-runtime")]
fn native_setup() {
    let _profiler = dhat::Profiler::builder().file_name("native_setup.json").build();
    let _test = native_impl::setup();
}

fn main() {
    #[cfg(feature = "dhat-runtime")]
    std::hint::black_box({
        native_setup();
        wamr_setup();
        wasmtime_setup();
    });

    #[cfg(
        all(
            feature = "pprof-flamegraph",
            not(feature = "dhat-runtime"),
            not(feature = "dhat-pingpong")
        )
    )]
    let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).build().unwrap();

    // flamegraph zal de native_pingpong nooit opmerken omdat deze magnitudes sneller is dan zelfs de tijd om het proces te starten, waardoor dit gezien wordt als noise en dus niet wordt opgenomen
    std::hint::black_box({
        native_pingpong();
        wamr_pingpong();
        wasmtime_pingpong();
    });

    #[cfg(
        all(
            feature = "pprof-flamegraph",
            not(feature = "dhat-runtime"),
            not(feature = "dhat-pingpong")
        )
    )]
    if let Ok(report) = guard.report().build() {
        let file = File::create("all_flame.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}
