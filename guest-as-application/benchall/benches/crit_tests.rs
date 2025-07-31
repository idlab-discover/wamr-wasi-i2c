use criterion::{ Criterion };

use wamr_rust_sdk::{ module::Module, runtime::Runtime };

fn setup_once() -> (Runtime, Module, wamr_impl::DroppableInstance) {
    // Doe setup eenmalig buiten de benchmark
    let runtime = wamr_impl::setup_runtime().unwrap();
    let module = wamr_impl::setup_module(&runtime).unwrap();
    let instance = wamr_impl::setup_module_instance(&runtime, &module).unwrap();
    (runtime, module, instance)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let (_runtime, _module, instance) = setup_once();
    let mut native_hw = native_impl::setup();

    c.bench_function("WAMR Ping Pong", |b| {
        b.iter(|| { std::hint::black_box(wamr_impl::run_guest_function(&instance).unwrap()) })
    });
    c.bench_function("Native Ping Pong", |b| {
        b.iter(|| { std::hint::black_box(native_impl::pingpong(&mut native_hw)) })
    });
}
