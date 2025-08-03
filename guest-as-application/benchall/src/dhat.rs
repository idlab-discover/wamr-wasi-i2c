#[cfg(any(feature = "dhat-runtime", feature = "dhat-heap"))]
use dhat;

#[cfg(any(feature = "dhat-runtime", feature = "dhat-heap"))]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn wamr_pingpong() {
    let (_rt, _mod, instance, f) = wamr_impl
        ::setup_runtime()
        .expect("[BENCH:dhat] WAMR runtime setup failed");

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name("wamr_pingpong.json").build();

    wamr_impl::run_pingpong(&instance, &f).expect("[BENCH:dhat] WAMR pingpong failed");
}

fn wasmtime_pingpong() {
    let (instance, mut store) = wasmtime_impl
        ::setup_runtime()
        .expect("[BENCH:crit] Wasmtime runtime setup failed");

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name("wasmtime_pingpong.json").build();
    wasmtime_impl
        ::run_pingpong(&instance, &mut store)
        .expect("[BENCH:dhat] Wasmtime pingpong failed");
}

fn native_pingpong() {
    let mut hw = native_impl::setup();

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name("native_pingpong.json").build();

    native_impl::pingpong(&mut hw);
}

#[cfg(feature = "dhat-runtime")]
fn wamr_setup() {
    let _profiler = dhat::Profiler::builder().file_name("wamr_setup.json").build();
    let _ = wamr_impl::setup_runtime().expect("[BENCH:dhat] WAMR runtime setup failed");
    /* std::hint::black_box({
    }); */
}

#[cfg(feature = "dhat-runtime")]
fn wasmtime_setup() {
    let _profiler = dhat::Profiler::builder().file_name("wasmtime_setup.json").build();
    let _ = wasmtime_impl::setup_runtime().expect("[BENCH:crit] Wasmtime runtime setup failed");
    /* std::hint::black_box({
    }); */
}

#[cfg(feature = "dhat-runtime")]
fn native_setup() {
    let _profiler = dhat::Profiler::builder().file_name("native_setup.json").build();
    let _ = native_impl::setup();
    /* std::hint::black_box(); */
}

fn main() {
    #[cfg(feature = "dhat-runtime")]
    {
        native_setup();
        wamr_setup();
        wasmtime_setup();
    }

    {
        native_pingpong();
        wamr_pingpong();
        wasmtime_pingpong();
    }
}
