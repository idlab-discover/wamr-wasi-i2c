use core::ffi::c_void;
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    value::WasmValue,
    RuntimeError,
};

use crate::{ host_functions, permission_manager::I2C_PERMISSIONS_MANAGER };

extern crate alloc;

// Moet in deze volgorde staan. Rust dropt variabelen in declaratievolgorde = FIFO bij structs, in een functie gebeurd dit omgekeerd: LIFO
// WAMR heeft weird behaviour wanneer de volgorde omgedraaid is
// Op te merken wanneer men een setup doet let _ = setup(), hierbij wordt de destructor te snel opgeroepen
// Probleem komt niet vaak voor, maar wanneer we het opstarten van de runtime willen profilen, zou het kunnen zijn dat we de variabelen niet zouden willen bijhouden
pub struct PingPongRunner {
    func: Function,
    instance: Instance,
    _module: Module,
    _runtime: Runtime,
}

impl PingPongRunner {
    pub fn new() -> Result<PingPongRunner, RuntimeError> {
        let runtime = Runtime::builder()
            .use_system_allocator()
            .register_host_function("host_read", host_functions::read as *mut c_void)
            .register_host_function("host_write", host_functions::write as *mut c_void)
            .register_host_function("host_open", host_functions::open as *mut c_void)
            .register_host_function("host_close", host_functions::close as *mut c_void)
            .build()?;

        const WASM_BYTES: &[u8] = include_bytes!("../wasmodules/guestp1.wasm");
        let module = Module::from_buf(&runtime, WASM_BYTES, "guestp1.wasm")?;
        let instance = Instance::new(&runtime, &module, 1024 * 64)?;
        let func = Function::find_export_func(&instance, "_start")?;
        Ok(PingPongRunner {
            _runtime: runtime,
            _module: module,
            instance,
            func,
        })
    }

    pub fn pingpong(&self) {
        let params: alloc::vec::Vec<WasmValue> = alloc::vec![];
        self.func.call(&self.instance, &params).unwrap();
    }
}

impl Drop for PingPongRunner {
    fn drop(&mut self) {
        I2C_PERMISSIONS_MANAGER.lock().close_instance(self.instance.get_inner_instance());
    }
}
