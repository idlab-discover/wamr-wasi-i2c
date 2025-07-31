fn wamr_test() {
    let _profiler = dhat::Profiler::builder().file_name("wamr_test1.json").build();
    let runtime = wamr_impl::setup_runtime().expect("Oeps");
    let module = wamr_impl::setup_module(&runtime).expect("Oeps");
    let instance = wamr_impl::setup_module_instance(&runtime, &module).expect("Oeps");
    wamr_impl::run_guest_function(&instance).expect("Oeps");
}

fn wamr_test2() {
    let runtime = wamr_impl::setup_runtime().expect("Oeps");
    let module = wamr_impl::setup_module(&runtime).expect("Oeps");

    let instance = wamr_impl::setup_module_instance(&runtime, &module).expect("Oeps");
    let _profiler = dhat::Profiler::builder().file_name("wamr_test2.json").build();
    wamr_impl::run_guest_function(&instance).expect("Oeps");
}

fn native_test() {
    let _profiler = dhat::Profiler::builder().file_name("native_test1.json").build();
    let mut hw = native_impl::setup();
    native_impl::pingpong(&mut hw);
}

fn native_test2() {
    let mut hw = native_impl::setup();
    let _profiler = dhat::Profiler::builder().file_name("native_test2.json").build();
    native_impl::pingpong(&mut hw);
}

pub static TESTS: [fn(); 4] = [wamr_test, wamr_test2, native_test, native_test2];
