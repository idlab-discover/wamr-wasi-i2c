use std::{ collections::HashMap, fmt, sync::{ LazyLock, Mutex } };

use wamr_rust_sdk::sys::WASMModuleInstanceCommon;

#[derive(Clone, Debug)]
pub struct I2cPermissions {
    pub can_read: bool,
    pub can_write: bool,
    pub is_whitelisted: bool,
    pub addresses: Vec<u16>,
}

pub struct I2cManager {
    // Instance of module => Resource Handle => I2cPermissions
    pub instances: HashMap<*const WASMModuleInstanceCommon, HashMap<u32, I2cPermissions>>,
    next_handle: u32,
}

unsafe impl Send for I2cManager {}
unsafe impl Sync for I2cManager {}

impl fmt::Debug for I2cManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "I2cManager {{")?;
        for (module_ptr, handles) in &self.instances {
            writeln!(f, "\tModule @ {:p}:", module_ptr)?;
            for (handle, permissions) in handles {
                writeln!(f, "\t\t{} => {:?}", handle, permissions)?;
            }
        }
        writeln!(f, "}}")
    }
}

// TODO: Resources worden eigenlijk nog niet correct behandeld
//      - We moeten nog checken op terug vrijkomen van resources
//      - Het doorgeven van resources is geen mogelijkheid (ownership op handle)
//      - Nog niets van error handling
impl I2cManager {
    pub fn new_handle(&mut self) -> u32 {
        let outp = self.next_handle;
        self.next_handle += 1;
        outp
    }

    pub fn get_permissions(
        &self,
        instance: *const WASMModuleInstanceCommon,
        handle: u32
    ) -> Option<&I2cPermissions> {
        self.instances.get(&instance)?.get(&handle)
    }
}

pub static I2C_MANAGER: LazyLock<Mutex<I2cManager>> = LazyLock::new(|| {
    Mutex::new(I2cManager {
        instances: HashMap::new(),
        next_handle: 1,
    })
});
