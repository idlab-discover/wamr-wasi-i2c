use std::{ ffi::c_void, path::PathBuf };
use std::collections::HashMap;
use std::sync::{ Mutex, LazyLock };
use wamr_rust_sdk::{
    function::Function,
    instance::Instance,
    module::Module,
    runtime::Runtime,
    sys::{
        wasm_runtime_malloc,
        wasm_runtime_module_free,
        wasm_runtime_module_malloc,
        WASMModuleInstanceCommon,
        wasm_exec_env_t,
        wasm_runtime_get_module_inst,
        wasm_runtime_addr_app_to_native,
    },
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

#[derive(Clone, Debug)]
struct I2cPermissions {
    can_read: bool,
    can_write: bool,
    is_whitelisted: bool,
    addresses: Vec<u16>,
}

struct I2cManager {
    instances: HashMap<*const WASMModuleInstanceCommon, HashMap<u32, I2cPermissions>>,
    next_handle: u32,
}

unsafe impl Send for I2cManager {}
unsafe impl Sync for I2cManager {}

impl I2cManager {
    fn new_handle(&mut self) -> u32 {
        let outp = self.next_handle;
        self.next_handle += 1;
        outp
    }

    fn get_permissions(
        &self,
        instance: *const WASMModuleInstanceCommon,
        handle: u32
    ) -> Option<&I2cPermissions> {
        self.instances.get(&instance)?.get(&handle)
    }
}

static I2C_MANAGER: LazyLock<Mutex<I2cManager>> = LazyLock::new(|| {
    Mutex::new(I2cManager {
        instances: HashMap::new(),
        next_handle: 1,
    })
});

extern "C" fn host_open(exec_env: wasm_exec_env_t) -> u32 {
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return 0;
    }

    let mut manager = I2C_MANAGER.lock().unwrap();
    let handle = manager.new_handle();

    let permissions = I2cPermissions {
        can_read: true,
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

extern "C" fn host_read(
    exec_env: wasm_exec_env_t,
    handle: u32,
    addr: u16,
    len: usize,
    buffer_ptr: u32
) {
    println!("Host: i2c_read called - handle: {}, address: 0x{:04x}, len: {}", handle, addr, len);
    let module_inst = unsafe { wasm_runtime_get_module_inst(exec_env) };
    if module_inst.is_null() {
        eprintln!("Host: Failed to get module instance");
        return;
    }

    let native_return_area = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, buffer_ptr as u64)
    }) as *mut u8;

    if native_return_area.is_null() {
        eprintln!("Host: Invalid return area pointer"); // TODO: Should panic!
        return;
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
                unsafe {
                    *native_return_area.add(0) = 1; // 0=Ok,1=Err
                    *native_return_area.add(std::mem::size_of::<*const u8>()) = 4; // Errorcode::Other = 4
                }
                return;
            }
        }
    };

    if !can_read {
        eprintln!("Host: Access denied - no read permission for handle {}", handle);
        unsafe {
            *native_return_area.add(0) = 1; // 0=Ok,1=Err
            *native_return_area.add(std::mem::size_of::<*const u8>()) = 4; // Errorcode::Other = 4
        }
        return;
    }

    let simulated_data: Vec<u8> = vec![0x11, 0xab, 0xcd]; // decimal: 17,171,205

    // Option 1: Updated malloc
    let buffer: *mut *mut c_void = std::ptr::null_mut();
    let wasm_data_ptr = unsafe {
        wasm_runtime_module_malloc(module_inst, simulated_data.len() as u64, buffer)
    };

    // Option 2: Legacy malloc
    let native_data_ptr = unsafe { wasm_runtime_malloc(simulated_data.len() as u32) };

    println!("Host: WASM data ptrs zijn: {:?} en {:?}", wasm_data_ptr, native_data_ptr);

    let native_data_ptr = (unsafe {
        wasm_runtime_addr_app_to_native(module_inst, wasm_data_ptr as u64)
    }) as *mut u8;

    if native_data_ptr.is_null() {
        eprintln!("Host: Failed to convert WASM pointer to native");
        unsafe {
            // Free the allocated memory
            wasm_runtime_module_free(module_inst, wasm_data_ptr);
            // Write error
            *native_return_area.add(0) = 1;
            *native_return_area.add(std::mem::size_of::<*const u8>()) = 4;
        }
        return;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(
            simulated_data.as_ptr(),
            native_data_ptr,
            simulated_data.len()
        );
    }

    unsafe {
        // Discriminant = 0 (Ok)
        *native_return_area.add(0) = 0;

        // Write pointer to data (as WASM pointer, not native)
        let ptr_offset = std::mem::size_of::<*const u8>();
        *(native_return_area.add(ptr_offset) as *mut u64) = wasm_data_ptr;

        // Write length
        let len_offset = 2 * std::mem::size_of::<*const u8>();
        *(native_return_area.add(len_offset) as *mut usize) = simulated_data.len();
    }

    println!("Host: Read completed");
}

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_read", host_read as *mut c_void)
        .register_host_function("host_open", host_open as *mut c_void)
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
