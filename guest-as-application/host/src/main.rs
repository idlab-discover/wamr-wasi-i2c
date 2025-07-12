use std::{ ffi::c_void, path::PathBuf };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    sys::{ wasm_exec_env_t, wasm_runtime_get_module_inst, wasm_runtime_addr_app_to_native },
    value::WasmValue,
    wasi_context::WasiCtxBuilder,
    RuntimeError,
};

#[repr(u8)]
enum I2cErrorCode {
    Bus = 0,
    ArbitrationLoss = 1,
    NoAcknowledge = 2,
    Overrun = 3,
    Other = 4,
}

// extern "C" fn host_open_i2c(exec_env: wasm_exec_env_t) {}

extern "C" fn host_read(
    exec_env: wasm_exec_env_t,
    handle: u32,
    addr: u16,
    len: u64,
    buffer_ptr: u32
) -> u8 {
    println!("Host: i2c_read called - handle: {}, address: 0x{:04x}, len: {}", handle, addr, len);
    unsafe {
        let module_inst = wasm_runtime_get_module_inst(exec_env);

        if module_inst.is_null() {
            eprintln!("Host: Failed to get module instance");
            return I2cErrorCode::Other as u8;
        }

        let native_buffer = wasm_runtime_addr_app_to_native(
            module_inst,
            buffer_ptr as u64
        ) as *mut u8;

        if native_buffer.is_null() {
            eprintln!("Host: Invalid buffer pointer");
            return I2cErrorCode::Other as u8;
        }

        let simulated_data = vec![0x11, 0xab, 0xcd]; // decimal: 17,171,205

        std::ptr::copy_nonoverlapping(simulated_data.as_ptr(), native_buffer, len as usize);
        0b000_00001
    }
}

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_read as *mut c_void)
        .build()?;

    let mut d = PathBuf::from(".");
    d.push("wasmodules");
    d.push("guest.wasm");
    let mut module = Module::from_file(&runtime, d.as_path())?;

    let wasi_ctx = WasiCtxBuilder::new().build();

    module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    let function = Function::find_export_func(&instance, "main");
    let params: Vec<WasmValue> = vec![];
    match function {
        Ok(main_func) => {
            println!("Calling main function...");
            let results = main_func.call(&instance, &params)?;
            println!("main returned: {:?}", results);
        }
        Err(e) => {
            eprintln!("No main function found: {}", e);
        }
    }
    Ok(())
}
