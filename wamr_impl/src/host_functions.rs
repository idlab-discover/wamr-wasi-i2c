use crate::permission_manager::I2C_PERMISSIONS_MANAGER;
use crate::hardware_manager::I2C_HARDWARE_MANAGER;
use embedded_hal::i2c::I2c;
use wamr_rust_sdk::sys::{
    wasm_exec_env_t,
    wasm_runtime_addr_app_to_native,
    wasm_runtime_get_module_inst,
};
use wasip1_i2c_lib::common::{ ErrorCode, I2cAddress, I2cResourceHandle };

extern crate alloc;

pub extern "C" fn close(exec_env: wasm_exec_env_t, handle: I2cResourceHandle) {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        return;
    }

    let mut perm_manager = I2C_PERMISSIONS_MANAGER.lock();
    perm_manager.close_handle(module_inst, handle);
}

pub extern "C" fn open(exec_env: wasm_exec_env_t) -> I2cResourceHandle {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        return 0;
    }

    let mut perm_manager = I2C_PERMISSIONS_MANAGER.lock();
    let handle = perm_manager.open_handle(module_inst);

    handle
}

pub extern "C" fn write(
    exec_env: wasm_exec_env_t,
    handle: I2cResourceHandle,
    addr: I2cAddress,
    len: usize,
    buffer_offset: usize
) -> u8 {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        return ErrorCode::Other.lower();
    }

    let can_write = {
        let manager = I2C_PERMISSIONS_MANAGER.lock();

        match manager.get_permissions(module_inst, handle) {
            Some(permissions) => permissions.can_write,
            None => {
                return ErrorCode::Other.lower();
            }
        }
    };

    if !can_write {
        return ErrorCode::Other.lower();
    }

    let native_buffer = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_offset as u64)
    }) as *mut u8;
    if native_buffer.is_null() {
        return ErrorCode::Other.lower();
    }

    let res = unsafe { core::slice::from_raw_parts(native_buffer, len) };
    let mut hardware_guard = I2C_HARDWARE_MANAGER.lock();
    if let Err(write_err) = hardware_guard.bus.write(addr as u8, res) {
        return convert_error(write_err).lower();
    }

    ErrorCode::None.lower()
}

pub extern "C" fn read(
    exec_env: wasm_exec_env_t,
    handle: I2cResourceHandle,
    addr: I2cAddress,
    len: usize,
    buffer_ptr: u32
) -> u8 {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        return ErrorCode::Other.lower();
    }

    let can_read = {
        let manager = I2C_PERMISSIONS_MANAGER.lock();

        match manager.get_permissions(module_inst, handle) {
            Some(permissions) => permissions.can_read,
            None => {
                return ErrorCode::Other.lower();
            }
        }
    };

    if !can_read {
        return ErrorCode::Other.lower();
    }

    let native_buffer = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_ptr as u64)
    }) as *mut u8;
    if native_buffer.is_null() {
        return ErrorCode::Other.lower();
    }

    let mut hardware_guard = I2C_HARDWARE_MANAGER.lock();

    let mut read_buffer = alloc::vec![0u8; len as usize];

    match hardware_guard.bus.read(addr as u8, &mut read_buffer) {
        Ok(_) => {
            unsafe {
                core::ptr::copy_nonoverlapping::<u8>(read_buffer.as_ptr(), native_buffer, len);
            }
        }
        Err(e) => {
            return convert_error(e).lower();
        }
    }
    ErrorCode::None.lower()
}

fn convert_error(err: linux_embedded_hal::I2CError) -> ErrorCode {
    use wasip1_i2c_lib::common::NoAcknowledgeSource;
    use embedded_hal::i2c::{ Error, ErrorKind as HalCode, NoAcknowledgeSource as HalNoAckS };
    match err.kind() {
        HalCode::NoAcknowledge(no_ack_src) =>
            match no_ack_src {
                HalNoAckS::Address => ErrorCode::NoAcknowledge(NoAcknowledgeSource::Address),
                HalNoAckS::Data => ErrorCode::NoAcknowledge(NoAcknowledgeSource::Data),
                HalNoAckS::Unknown => ErrorCode::NoAcknowledge(NoAcknowledgeSource::Unknown),
            }
        HalCode::ArbitrationLoss => ErrorCode::ArbitrationLoss,
        HalCode::Bus => ErrorCode::Bus,
        HalCode::Overrun => ErrorCode::Overrun,
        _ => ErrorCode::Other,
    }
}
