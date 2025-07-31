use std::{ ffi::c_void, path::PathBuf };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    value::WasmValue,
    RuntimeError,
};

use crate::{ host_functions, permission_manager::I2C_PERMISSIONS_MANAGER };

pub fn setup_runtime() -> Result<Runtime, RuntimeError> {
    // Setup WAMR & register host functions
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_functions::read as *mut c_void)
        .register_host_function("host_write", host_functions::write as *mut c_void)
        .register_host_function("host_open", host_functions::open as *mut c_void)
        .register_host_function("host_close", host_functions::close as *mut c_void)
        .build()?;
    Ok(runtime)
}

pub fn setup_module(runtime: &Runtime) -> Result<Module, RuntimeError> {
    let mut path_buffer = PathBuf::from(".");
    path_buffer.push("wasmodules");
    path_buffer.push("guest.wasm");
    let module = Module::from_file(runtime, path_buffer.as_path())?;
    Ok(module)
}

pub struct DroppableInstance {
    pub instance: Instance,
}

impl Drop for DroppableInstance {
    fn drop(&mut self) {
        I2C_PERMISSIONS_MANAGER.lock().unwrap().close_instance(self.instance.get_inner_instance());
    }
}

pub fn setup_module_instance(
    runtime: &Runtime,
    module: &Module
) -> Result<DroppableInstance, RuntimeError> {
    let instance = DroppableInstance { instance: Instance::new(runtime, module, 1024 * 64)? };
    Ok(instance)
}

pub fn run_guest_function(instance: &DroppableInstance) -> Result<WasmValue, RuntimeError> {
    let function = Function::find_export_func(&instance.instance, "_start")?;
    let params: Vec<WasmValue> = vec![];
    let ret_val = function.call(&instance.instance, &params)?;
    Ok(ret_val)
}
