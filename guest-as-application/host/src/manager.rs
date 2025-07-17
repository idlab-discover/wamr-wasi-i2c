use std::{ collections::HashMap, sync::{ LazyLock, Mutex } };

use wamr_rust_sdk::sys::WASMModuleInstanceCommon;

#[derive(Clone, Debug)]
pub struct I2cPermissions {
    pub can_read: bool,
    pub can_write: bool,
    pub is_whitelisted: bool,
    pub addresses: Vec<u16>,
}

pub struct I2cManager {
    pub instances: HashMap<*const WASMModuleInstanceCommon, HashMap<u32, I2cPermissions>>,
    next_handle: u32,
}

unsafe impl Send for I2cManager {}
unsafe impl Sync for I2cManager {}

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
