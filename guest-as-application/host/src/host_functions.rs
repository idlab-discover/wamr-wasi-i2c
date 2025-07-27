use crate::manager::{ I2C_MANAGER };
use wamr_rust_sdk::sys::{
    wasm_exec_env_t,
    wasm_runtime_addr_app_to_native,
    wasm_runtime_get_module_inst,
};
use wasip1_i2c::common::{ ErrorCode, I2cResourceHandle, I2cAddress };

pub extern "C" fn close(exec_env: wasm_exec_env_t, handle: I2cResourceHandle) {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return;
    }

    let mut manager = I2C_MANAGER.lock().unwrap();
    manager.close_handle(module_inst, handle);

    println!("Host: Closed I2C handle {} for module instance {:p}", handle, module_inst);
    println!("{:?}", manager);
}

pub extern "C" fn open(exec_env: wasm_exec_env_t) -> I2cResourceHandle {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return 0;
    }

    let mut manager = I2C_MANAGER.lock().unwrap();
    let handle = manager.new_handle(module_inst);

    println!("Host: Created I2C handle {} for module instance {:p}", handle, module_inst);
    println!("{:?}", manager);

    handle
}

pub extern "C" fn write(
    exec_env: wasm_exec_env_t,
    handle: I2cResourceHandle,
    addr: I2cAddress,
    len: usize,
    buffer_offset: usize
) -> u8 {
    println!(
        "Host: i2c_write called - handle: {}, address: 0x{:04x}, len: {}, buffer_ptr: {:?}",
        handle,
        addr,
        len,
        buffer_offset
    );
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return ErrorCode::Other.into();
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
                return ErrorCode::Other.into();
            }
        }
    };

    if !can_write {
        eprintln!("Host: Access denied - no write permission for handle {}", handle);
        return ErrorCode::Other.into();
    }

    let native_buffer = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_offset as u64)
    }) as *mut u8;
    if native_buffer.is_null() {
        eprintln!("Host: Invalid buffer pointer");
        return ErrorCode::Other.into();
    }

    let res = unsafe { std::slice::from_raw_parts(native_buffer, len) };

    println!("Host: Write completed: {:?}", res);
    0b000_00000
}

pub extern "C" fn read(
    exec_env: wasm_exec_env_t,
    handle: I2cResourceHandle,
    addr: I2cAddress,
    len: u64,
    buffer_ptr: u32
) -> u8 {
    println!("Host: i2c_read called - handle: {}, address: 0x{:04x}, len: {}", handle, addr, len);
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return ErrorCode::Other.into();
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
                return ErrorCode::Other.into();
            }
        }
    };

    if !can_read {
        eprintln!("Host: Access denied - no read permission for handle {}", handle);
        return ErrorCode::Other.into();
    }

    let native_buffer = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_ptr as u64)
    }) as *mut u8;
    if native_buffer.is_null() {
        eprintln!("Host: Invalid buffer pointer");
        return ErrorCode::Other.into();
    }

    let simulated_data = vec![0x11, 0xab, 0xcd]; // decimal: 17,171,205

    unsafe {
        std::ptr::copy_nonoverlapping::<u8>(simulated_data.as_ptr(), native_buffer, len as usize);
    }
    println!("Host: Read completed");
    ErrorCode::None.into()
}
