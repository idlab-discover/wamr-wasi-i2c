use std::{ ffi::c_void, path::PathBuf };
use std::collections::HashMap;
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    sys::{
        wasm_runtime_malloc,
        wasm_runtime_module_free,
        wasm_runtime_module_malloc,
        wasm_exec_env_t,
        wasm_runtime_get_module_inst,
        wasm_runtime_addr_app_to_native,
    },
    value::WasmValue,
    wasi_context::WasiCtxBuilder,
    RuntimeError,
};

mod host_functions;
mod manager;
use manager::{ I2C_MANAGER, I2cPermissions };

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_functions::read as *mut c_void)
        .register_host_function("host_open", host_functions::open as *mut c_void)
        .build()?;

    let mut d = PathBuf::from(".");
    d.push("wasmodules");
    d.push("guest.wasm");
    let mut module = Module::from_file(&runtime, d.as_path())?;

    let wasi_ctx = WasiCtxBuilder::new().build();

    module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 256)?;

    /* use wamr_rust_sdk::sys::wasm_runtime_lookup_function;
    for name in ["malloc", "free", "__wbindgen_malloc", "cabi_realloc", "_start"] {
        let func = unsafe {
            let module_inst = instance.get_inner_instance();
            let c_name = std::ffi::CString::new(name).unwrap();
            wasm_runtime_lookup_function(module_inst, c_name.as_ptr())
        };
        println!("Function '{}' found: {}", name, !func.is_null());
    } */

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
