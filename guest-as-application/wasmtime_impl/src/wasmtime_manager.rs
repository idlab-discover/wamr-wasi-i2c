use embedded_hal::i2c::{ I2c, Error };
use std::collections::HashMap;

use wasmtime::component::Resource;

use crate::{ bindings, bindings::wasi, I2C_BUS };

#[derive(Clone)]
struct I2cPermissions {
    can_read: bool,
    can_write: bool,
}

#[derive(Clone)]
struct I2cResourceCtx {
    acl: I2cPermissions,
}

impl I2cResourceCtx {
    fn new() -> I2cResourceCtx {
        I2cResourceCtx {
            acl: I2cPermissions { can_read: true, can_write: true },
        }
    }
}

// Host state
pub struct HostState {
    i2c_devices: HashMap<u32, I2cResourceCtx>,
    next_i2c_id: u32,
}

impl HostState {
    pub fn new() -> HostState {
        HostState { i2c_devices: HashMap::new(), next_i2c_id: 0 }
    }
}

// Implementeer de imports van de I2C Interface
impl wasi::i2c::i2c::Host for HostState {}

// Implementeer de imports van de I2C Resource
impl wasi::i2c::i2c::HostI2c for HostState {
    fn read(
        &mut self,
        res: Resource<wasi::i2c::i2c::I2c>,
        address: wasi::i2c::i2c::Address,
        len: u64
    ) -> Result<Vec<u8>, wasi::i2c::i2c::ErrorCode> {
        let resource_id = res.rep();
        let i2c_res = self.i2c_devices.get(&resource_id).ok_or(wasi::i2c::i2c::ErrorCode::Other)?;

        if !i2c_res.acl.can_read {
            return Err(wasi::i2c::i2c::ErrorCode::Other);
        }

        let mut buffer = vec![0u8; len as usize];

        // Lock de I2C device en doe de read
        let mut i2c = I2C_BUS.lock().unwrap();

        // Linux I2C gebruikt 7-bit adressen
        let addr_7bit = (address & 0x7f) as u8;

        match i2c.read(addr_7bit, &mut buffer) {
            Ok(_) => { Ok(buffer) }
            Err(e) => Err(e.into()),
        }
    }

    fn write(
        &mut self,
        res: Resource<wasi::i2c::i2c::I2c>,
        address: wasi::i2c::i2c::Address,
        data: Vec<u8>
    ) -> Result<(), wasi::i2c::i2c::ErrorCode> {
        let resource_id = res.rep();
        let i2c_res = self.i2c_devices.get(&resource_id).ok_or(wasi::i2c::i2c::ErrorCode::Other)?;

        if !i2c_res.acl.can_write {
            return Err(wasi::i2c::i2c::ErrorCode::Other);
        }

        // Lock de I2C device en doe de write
        let mut i2c = I2C_BUS.lock().unwrap();

        // Linux I2C gebruikt 7-bit adressen
        let addr_7bit = (address & 0x7f) as u8;

        match i2c.write(addr_7bit, &data) {
            Ok(_) => { Ok(()) }
            Err(e) => Err(e.into()),
        }
    }

    fn drop(
        &mut self,
        rep: Resource<wasi::i2c::i2c::I2c>
    ) -> std::result::Result<(), wasmtime::Error> {
        self.i2c_devices.remove(&rep.rep());
        Ok(())
    }
}

// Implementeer de imports van de pingpong world
impl bindings::PingpongImports for HostState {
    fn get_i2c_bus(&mut self) -> Resource<wasi::i2c::i2c::I2c> {
        let dev = I2cResourceCtx::new();
        let id = self.next_i2c_id;
        self.next_i2c_id += 1;
        self.i2c_devices.insert(id, dev);
        Resource::new_own(id)
    }
}

// Helper om embedded_hal error te converteren naar de Wasi I2c Error codes
impl From<linux_embedded_hal::I2CError> for wasi::i2c::i2c::ErrorCode {
    fn from(err: linux_embedded_hal::I2CError) -> Self {
        use wasi::i2c::i2c::{ ErrorCode, NoAcknowledgeSource };
        use embedded_hal::i2c::{ ErrorKind as HalCode, NoAcknowledgeSource as HalNoAckS };
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
}
