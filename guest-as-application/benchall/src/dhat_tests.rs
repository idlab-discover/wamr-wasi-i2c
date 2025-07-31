fn wamr_test() {
    let _profiler = dhat::Profiler::builder().file_name("wamr_impl.json").build();
    let runtime = wamr_impl::setup_runtime().expect("Oeps");
    let module = wamr_impl::setup_module(&runtime).expect("Oeps");
    {
        let instance = wamr_impl::setup_module_instance(&runtime, &module).expect("Oeps");
        wamr_impl::run_guest_function(&instance).expect("Oeps");
    }
}

pub static TESTS: [fn(); 1] = [wamr_test];
