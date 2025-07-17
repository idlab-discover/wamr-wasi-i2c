mod manager;
mod host_functions;

use std::{ ffi::c_void, path::PathBuf };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    value::WasmValue,
    wasi_context::WasiCtxBuilder,
    RuntimeError,
};

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_functions::read as *mut c_void)
        .register_host_function("host_open", host_functions::open as *mut c_void)
        .register_host_function("host_write", host_functions::write as *mut c_void)
        .build()?;

    let mut d = PathBuf::from(".");
    d.push("wasmodules");
    d.push("guest.wasm");
    let mut module = Module::from_file(&runtime, d.as_path())?;

    let wasi_ctx = WasiCtxBuilder::new().build();

    module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    let function = Function::find_export_func(&instance, "_start");
    let params: Vec<WasmValue> = vec![];
    match function {
        Ok(main_func) => {
            println!("Calling main function...");
            main_func.call(&instance, &params)?;
            println!("main returned");
        }
        Err(e) => {
            eprintln!("No main function found: {}", e);
        }
    }
    Ok(())
}
