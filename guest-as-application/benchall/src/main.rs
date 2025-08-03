#[cfg(feature = "dhat-heap")]
use dhat;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn wamr_test() {
    let (instance, f) = wamr_impl::setup_runtime().expect("[BENCH:dhat] WAMR runtime setup failed");

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name("wamr_test.json").build();

    wamr_impl::run_pingpong(&instance, &f).expect("[BENCH:dhat] WAMR pingpong failed");
}

fn wasmtime_test() {
    let (instance, mut store) = wasmtime_impl
        ::setup_runtime()
        .expect("[BENCH:crit] Wasmtime runtime setup failed");

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name("wasmtime_test.json").build();
    wasmtime_impl
        ::run_pingpong(&instance, &mut store)
        .expect("[BENCH:dhat] Wasmtime pingpong failed");
}

fn native_test() {
    let mut hw = native_impl::setup();

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name("native_test.json").build();

    native_impl::pingpong(&mut hw);
}

fn main() {
    native_test();
    wamr_test();
    wasmtime_test();
}
