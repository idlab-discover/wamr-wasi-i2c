use std::{ collections::HashMap, fmt, sync::{ LazyLock, Mutex } };

use wamr_rust_sdk::sys::WASMModuleInstanceCommon;
use wasip1_i2c_lib::common::I2cResourceHandle;

// Can add more permissions like a white/black list of addresses
#[derive(Clone, Debug)]
pub struct I2cPermissions {
    pub can_read: bool,
    pub can_write: bool,
}

pub struct I2cPermissionsManager {
    // Instance of module => Resource Handle => I2cPermissions
    instances: HashMap<*const WASMModuleInstanceCommon, HashMap<I2cResourceHandle, I2cPermissions>>,
    next_handle: I2cResourceHandle,
}

// TODO: Resources worden eigenlijk nog niet correct behandeld
//      - Het doorgeven van resources is ook nog geen mogelijkheid (ownership op handle), maar maakt ook niet enorm veel sense in de context van i2c
//      - Nog niets van error handling
//          - Wanneer new_handle u32::MAX bereikt zitten we met overflow
//          - Mogelijk om telkens wanneer een nieuwe handle gevraagd wordt na te gaan wat de eerstvolgende vrije handle is voor die instance
//              - Enkel mogelijk wanneer resources niet doorgegeven kunnen worden, anders een lijst bijhouden van vrije handles OF een nieuwe handle creëren wanneer het overgedragen wordt.
impl I2cPermissionsManager {
    pub fn open_handle(&mut self, instance: *const WASMModuleInstanceCommon) -> I2cResourceHandle {
        let new_handle = self.next_handle;

        let permissions = I2cPermissions {
            can_read: true,
            can_write: true,
        };

        let instances_handles = self.instances.entry(instance).or_insert_with(HashMap::new);

        instances_handles.insert(new_handle, permissions);

        self.next_handle += 1;
        new_handle
    }

    pub fn close_handle(
        &mut self,
        instance: *const WASMModuleInstanceCommon,
        handle: I2cResourceHandle
    ) {
        if let Some(handles) = self.instances.get_mut(&instance) {
            handles.remove(&handle);
        }
    }

    pub fn close_instance(&mut self, instance: *const WASMModuleInstanceCommon) {
        self.instances.remove(&instance);
    }

    pub fn get_permissions(
        &self,
        instance: *const WASMModuleInstanceCommon,
        handle: I2cResourceHandle
    ) -> Option<&I2cPermissions> {
        self.instances.get(&instance)?.get(&handle)
    }
}

pub static I2C_PERMISSIONS_MANAGER: LazyLock<Mutex<I2cPermissionsManager>> = LazyLock::new(|| {
    Mutex::new(I2cPermissionsManager {
        instances: HashMap::new(),
        next_handle: 1,
    })
});

unsafe impl Send for I2cPermissionsManager {}
unsafe impl Sync for I2cPermissionsManager {}

impl fmt::Debug for I2cPermissionsManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "I2cManager {{")?;
        for (module_ptr, handles) in &self.instances {
            writeln!(f, "\tHandles for Module @ {:p}:", *module_ptr)?;
            for (handle, permissions) in handles {
                writeln!(f, "\t\t{} => {:?}", handle, permissions)?;
            }
        }
        writeln!(f, "}}")
    }
}
