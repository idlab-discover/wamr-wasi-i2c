use wamr_impl::{ run_guest_function, setup_module, setup_module_instance, setup_runtime };

fn main() {
    let runtime = setup_runtime().expect("Oeps");
    let module = setup_module(&runtime).expect("Oeps");
    let instance = setup_module_instance(&runtime, &module).expect("Oeps");
    run_guest_function(&instance).expect("Oeps");
}
