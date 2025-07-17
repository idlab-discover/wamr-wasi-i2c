use std::{ ffi::c_void, path::PathBuf };
use std::collections::HashMap;
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

mod manager;
use manager::{ I2C_MANAGER, I2cPermissions };

#[repr(u8)]
enum I2cErrorCode {
    Bus = 0,
    ArbitrationLoss = 1,
    NoAcknowledge = 2,
    Overrun = 3,
    Other = 4,
}

extern "C" fn host_open(exec_env: wasm_exec_env_t) -> u32 {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return 0;
    }

    let mut manager = I2C_MANAGER.lock().unwrap();
    let handle = manager.new_handle();

    let permissions = I2cPermissions {
        can_read: false,
        can_write: true,
        is_whitelisted: false,
        addresses: vec![],
    };

    let instances_handles = manager.instances.entry(module_inst).or_insert_with(HashMap::new);

    instances_handles.insert(handle, permissions);

    println!("Host: Created I2C handle {} for module instance {:p}", handle, module_inst);
    println!("Host: ACL for module instance {:p} is now: {:?}", module_inst, instances_handles);

    handle
}

extern "C" fn host_write(
    exec_env: wasm_exec_env_t,
    handle: u32,
    addr: u16,
    len: usize,
    buffer_ptr: u32
) -> u8 {
    println!("Host: i2c_write called - handle: {}, address: 0x{:04x}, len: {}", handle, addr, len);
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return I2cErrorCode::Other as u8;
    }

    let can_write = {
        let manager = I2C_MANAGER.lock().unwrap();
        match manager.get_permissions(module_inst, handle) {
            Some(permissions) => permissions.can_write,
            None => {
                eprintln!(
                    "Host: Handle {} not found for module instance {:p}",
                    handle,
                    module_inst
                );
                return I2cErrorCode::Other as u8;
            }
        }
    };

    if !can_write {
        eprintln!("Host: Access denied - no write permission for handle {}", handle);
        return I2cErrorCode::Other as u8;
    }

    let native_buffer = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_ptr as u64)
    }) as *mut u8;
    if native_buffer.is_null() {
        eprintln!("Host: Invalid buffer pointer");
        return I2cErrorCode::Other as u8;
    }

    let res = unsafe { Vec::from_raw_parts(native_buffer, len, len) };

    println!("Host: Write completed: {:?}", res);
    0b000_00000
}

extern "C" fn host_read(
    exec_env: wasm_exec_env_t,
    handle: u32,
    addr: u16,
    len: u64,
    buffer_ptr: u32
) -> u8 {
    println!("Host: i2c_read called - handle: {}, address: 0x{:04x}, len: {}", handle, addr, len);
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return I2cErrorCode::Other as u8;
    }

    let can_read = {
        let manager = I2C_MANAGER.lock().unwrap();
        match manager.get_permissions(module_inst, handle) {
            Some(permissions) => permissions.can_read,
            None => {
                eprintln!(
                    "Host: Handle {} not found for module instance {:p}",
                    handle,
                    module_inst
                );
                return I2cErrorCode::Other as u8;
            }
        }
    };

    if !can_read {
        eprintln!("Host: Access denied - no read permission for handle {}", handle);
        return I2cErrorCode::Other as u8;
    }

    let native_buffer = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_ptr as u64)
    }) as *mut u8;
    if native_buffer.is_null() {
        eprintln!("Host: Invalid buffer pointer");
        return I2cErrorCode::Other as u8;
    }

    let simulated_data = vec![0x11, 0xab, 0xcd]; // decimal: 17,171,205

    unsafe {
        std::ptr::copy_nonoverlapping::<u8>(simulated_data.as_ptr(), native_buffer, len as usize);
    }
    println!("Host: Read completed");
    0b000_00000
}

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_read as *mut c_void)
        .register_host_function("host_open", host_open as *mut c_void)
        .register_host_function("host_write", host_write as *mut c_void)
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
