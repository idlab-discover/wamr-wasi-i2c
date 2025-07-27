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
    instances: HashMap<*const WASMModuleInstanceCommon, HashMap<u32, I2cPermissions>>,
    next_handle: u32,
}

// TODO: Resources worden eigenlijk nog niet correct behandeld
//      - We moeten nog checken op terug vrijkomen van resources
//      - Het doorgeven van resources is ook nog geen mogelijkheid (ownership op handle), maar maakt ook niet enorm veel sense in de context van i2c
//      - Nog niets van error handling
//          - Wanneer new_handle u32::MAX bereikt zitten we met overflow
//          - Mogelijk om telkens wanneer een nieuwe handle gevraagd wordt na te gaan wat de eerstvolgende vrije handle is voor die instance
//              - Enkel mogelijk wanneer resources niet doorgegeven kunnen worden, anders een lijst bijhouden van vrije handles OF een nieuwe handle creëren wanneer het overgedragen wordt.
impl I2cManager {
    pub fn new_handle(&mut self, instance: *const WASMModuleInstanceCommon) -> u32 {
        let new_handle = self.next_handle;

        let permissions = I2cPermissions {
            can_read: true,
            can_write: true,
            is_whitelisted: false,
            addresses: vec![],
        };

        let instances_handles = self.instances.entry(instance).or_insert_with(HashMap::new);

        instances_handles.insert(new_handle, permissions);

        self.next_handle += 1;
        new_handle
    }

    pub fn close_handle(&mut self, instance: *const WASMModuleInstanceCommon, handle: u32) {
        if let Some(handles) = self.instances.get_mut(&instance) {
            handles.remove(&handle);
        }
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

unsafe impl Send for I2cManager {}
unsafe impl Sync for I2cManager {}

impl fmt::Debug for I2cManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "I2cManager {{")?;
        for (module_ptr, handles) in &self.instances {
            writeln!(f, "\tHandles for Module @ {:p}:", module_ptr)?;
            for (handle, permissions) in handles {
                writeln!(f, "\t\t{} => {:?}", handle, permissions)?;
            }
        }
        writeln!(f, "}}")
    }
}
