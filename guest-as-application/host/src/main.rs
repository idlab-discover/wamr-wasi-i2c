mod manager;
mod host_functions;
mod i2c_commons;

use std::{ ffi::c_void, path::PathBuf };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    value::WasmValue,
    RuntimeError,
};

fn main() -> Result<(), RuntimeError> {
    // Setup WAMR & register host functions
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_functions::read as *mut c_void)
        .register_host_function("host_open", host_functions::open as *mut c_void)
        .register_host_function("host_write", host_functions::write as *mut c_void)
        .build()?;

    // Look for and load in the compiled guest wasm module
    let mut path_buffer = PathBuf::from(".");
    path_buffer.push("wasmodules");
    path_buffer.push("guest.wasm");
    let module = Module::from_file(&runtime, path_buffer.as_path())?;

    // Instantiate loaded wasm module
    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    // Look for and run the entrance function of the guest module
    let function = Function::find_export_func(&instance, "_start");
    let params: Vec<WasmValue> = vec![];
    match function {
        Ok(main_func) => {
            main_func.call(&instance, &params)?;
        }
        Err(e) => {
            eprintln!("No main function found: {}", e);
        }
    }
    Ok(())
}
