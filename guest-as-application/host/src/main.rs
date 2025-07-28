mod permission_manager;
mod host_functions;
mod hardware_manager;

use std::{ error::Error, ffi::c_void, path::PathBuf };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    value::WasmValue,
    RuntimeError,
};
use hardware_manager::init_i2c_hardware;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup I2C hardware
    if let Err(hardware_manager_error) = init_i2c_hardware("/dev/i2c-1") {
        eprintln!("Host: Error: {}", hardware_manager_error);
        return Ok(());
    }

    // Setup WAMR & register host functions
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_functions::read as *mut c_void)
        .register_host_function("host_write", host_functions::write as *mut c_void)
        .register_host_function("host_open", host_functions::open as *mut c_void)
        .register_host_function("host_close", host_functions::close as *mut c_void)
        .build()?;

    let mut path_buffer = PathBuf::from(".");
    path_buffer.push("wasmodules");
    path_buffer.push("guest.wasm");
    let module = Module::from_file(&runtime, path_buffer.as_path())?;

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

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
