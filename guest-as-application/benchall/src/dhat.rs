use dhat;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn wamr_test() {
    let runtime = wamr_impl::setup_runtime().expect("Oeps");
    let module = wamr_impl::setup_module(&runtime).expect("Oeps");
    let instance = wamr_impl::setup_module_instance(&runtime, &module).expect("Oeps");
    let _profiler = dhat::Profiler::builder().file_name("wamr_test1.json").build();
    wamr_impl::run_guest_function(&instance).expect("Oeps");
}

fn native_test() {
    let mut hw = native_impl::setup();
    let _profiler = dhat::Profiler::builder().file_name("native_test1.json").build();
    native_impl::pingpong(&mut hw);
}

fn main() {
    let tests = [wamr_test, native_test];
    for test in tests {
        test();
    }
}
