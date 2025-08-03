use std::{ ffi::c_void, path::PathBuf };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    sys::WASMModuleInstanceCommon,
    value::WasmValue,
    RuntimeError,
};

use crate::{ host_functions, permission_manager::I2C_PERMISSIONS_MANAGER };
pub struct DroppableInstance {
    pub instance: Instance,
    identifier: *const WASMModuleInstanceCommon,
}
impl DroppableInstance {
    fn new(runtime: &Runtime, module: &Module, stack_size: u32) -> Result<Self, RuntimeError> {
        let instance = Instance::new(runtime, module, stack_size)?;
        let identifier = instance.get_inner_instance() as *const WASMModuleInstanceCommon;
        Ok(DroppableInstance { instance, identifier })
    }
}

impl Drop for DroppableInstance {
    fn drop(&mut self) {
        I2C_PERMISSIONS_MANAGER.lock().unwrap().close_instance(self.identifier);
    }
}

pub fn setup_runtime() -> Result<(Runtime, Module, DroppableInstance, Function), RuntimeError> {
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
    path_buffer.push("guestp1.wasm");
    let module = Module::from_file(&runtime, path_buffer.as_path())?;
    let instance = DroppableInstance::new(&runtime, &module, 1024 * 64)?;
    let function = Function::find_export_func(&instance.instance, "_start")?;
    Ok((runtime, module, instance, function))
}

pub fn run_pingpong(
    instance: &DroppableInstance,
    function: &Function
) -> Result<WasmValue, RuntimeError> {
    let params: Vec<WasmValue> = vec![];
    let ret_val = function.call(&instance.instance, &params)?;
    Ok(ret_val)
}
