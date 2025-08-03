use wasmtime::component::{ Component, HasSelf, Linker, Resource };
use wasmtime::{ Config, Engine, Store };
use std::sync::{ LazyLock, Mutex };
use std::collections::HashMap;
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::{ I2c, Error };

use anyhow::Result;

mod bindings;
use bindings::wasi;

static I2C_BUS: LazyLock<Mutex<I2cdev>> = LazyLock::new(|| {
    Mutex::new(I2cdev::new("/dev/i2c-1").expect("[HOST] Hardware not available"))
});

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

// Implementeer de I2C interface
impl wasi::i2c::i2c::Host for HostState {}

impl wasi::i2c::i2c::HostI2c for HostState {
    fn read(
        &mut self,
        res: Resource<wasi::i2c::i2c::I2c>,
        address: wasi::i2c::i2c::Address,
        len: u64
    ) -> Result<Vec<u8>, wasi::i2c::i2c::ErrorCode> {
        println!("[HOST] I2C read from address 0x{:04x}, len: {}", address, len);

        let resource_id = res.rep();
        let i2c_res = self.i2c_devices.get(&resource_id).ok_or(wasi::i2c::i2c::ErrorCode::Other)?;

        if !i2c_res.acl.can_read {
            return Err(wasi::i2c::i2c::ErrorCode::Other);
        }

        let mut buffer = vec![0u8; len as usize];

        // Lock de I2C device en doe de read
        let mut i2c = I2C_BUS.lock().unwrap();

        // Linux I2C gebruikt 7-bit adressen (shift indien nodig)
        let addr_7bit = (address & 0x7f) as u8;

        match i2c.read(addr_7bit, &mut buffer) {
            Ok(_) => {
                println!("[HOST] Read successful: {:?}", buffer);
                Ok(buffer)
            }
            Err(e) => {
                let i2c_err = match e.kind() {
                    embedded_hal::i2c::ErrorKind::NoAcknowledge(no_ack_src) =>
                        match no_ack_src {
                            embedded_hal::i2c::NoAcknowledgeSource::Address =>
                                wasi::i2c::i2c::ErrorCode::NoAcknowledge(
                                    wasi::i2c::i2c::NoAcknowledgeSource::Address
                                ),
                            embedded_hal::i2c::NoAcknowledgeSource::Data =>
                                wasi::i2c::i2c::ErrorCode::NoAcknowledge(
                                    wasi::i2c::i2c::NoAcknowledgeSource::Data
                                ),
                            embedded_hal::i2c::NoAcknowledgeSource::Unknown =>
                                wasi::i2c::i2c::ErrorCode::NoAcknowledge(
                                    wasi::i2c::i2c::NoAcknowledgeSource::Unknown
                                ),
                        }
                    embedded_hal::i2c::ErrorKind::ArbitrationLoss =>
                        wasi::i2c::i2c::ErrorCode::ArbitrationLoss,
                    embedded_hal::i2c::ErrorKind::Bus => wasi::i2c::i2c::ErrorCode::Bus,
                    embedded_hal::i2c::ErrorKind::Overrun => wasi::i2c::i2c::ErrorCode::Overrun,
                    _ => wasi::i2c::i2c::ErrorCode::Other,
                };
                println!("[HOST] I2C read error: {:?}", i2c_err);
                Err(i2c_err)
            }
        }
    }

    fn write(
        &mut self,
        res: Resource<wasi::i2c::i2c::I2c>,
        address: wasi::i2c::i2c::Address,
        data: Vec<u8>
    ) -> Result<(), wasi::i2c::i2c::ErrorCode> {
        println!("[HOST] I2C write to address 0x{:04x}, data: {:?}", address, data);

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
            Ok(_) => {
                println!("[HOST] Write successful");
                Ok(())
            }
            Err(e) => {
                let i2c_err = match e.kind() {
                    embedded_hal::i2c::ErrorKind::NoAcknowledge(no_ack_src) =>
                        match no_ack_src {
                            embedded_hal::i2c::NoAcknowledgeSource::Address =>
                                wasi::i2c::i2c::ErrorCode::NoAcknowledge(
                                    wasi::i2c::i2c::NoAcknowledgeSource::Address
                                ),
                            embedded_hal::i2c::NoAcknowledgeSource::Data =>
                                wasi::i2c::i2c::ErrorCode::NoAcknowledge(
                                    wasi::i2c::i2c::NoAcknowledgeSource::Data
                                ),
                            embedded_hal::i2c::NoAcknowledgeSource::Unknown =>
                                wasi::i2c::i2c::ErrorCode::NoAcknowledge(
                                    wasi::i2c::i2c::NoAcknowledgeSource::Unknown
                                ),
                        }
                    embedded_hal::i2c::ErrorKind::ArbitrationLoss =>
                        wasi::i2c::i2c::ErrorCode::ArbitrationLoss,
                    embedded_hal::i2c::ErrorKind::Bus => wasi::i2c::i2c::ErrorCode::Bus,
                    embedded_hal::i2c::ErrorKind::Overrun => wasi::i2c::i2c::ErrorCode::Overrun,
                    _ => wasi::i2c::i2c::ErrorCode::Other,
                };
                println!("[HOST] I2C write error: {:?}", i2c_err);
                Err(i2c_err)
            }
        }
    }

    fn drop(
        &mut self,
        rep: Resource<wasi::i2c::i2c::I2c>
    ) -> std::result::Result<(), wasmtime::Error> {
        println!("[HOST] I2C resource dropped");
        self.i2c_devices.remove(&rep.rep());
        Ok(())
    }
}

// Implementeer de get_i2c_bus import
impl bindings::PingpongImports for HostState {
    fn get_i2c_bus(&mut self) -> Resource<wasi::i2c::i2c::I2c> {
        println!("[HOST] Called get_i2c_bus");
        let dev = I2cResourceCtx::new();
        let id = self.next_i2c_id;
        self.next_i2c_id += 1;
        self.i2c_devices.insert(id, dev);
        Resource::new_own(id)
    }
}

pub fn setup_runtime() -> Result<(bindings::Pingpong, Store<HostState>)> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;

    let mut store = Store::new(&engine, HostState {
        i2c_devices: HashMap::new(),
        next_i2c_id: 0,
    });

    let component = Component::from_file(&engine, "wasmodules/guestp2.wasm")?;

    let mut linker = Linker::new(&engine);

    bindings::Pingpong::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut HostState| state)?;

    let inst = bindings::Pingpong::instantiate(&mut store, &component, &linker)?;
    Ok((inst, store))
}

pub fn run_pingpong(inst: &bindings::Pingpong, store: &mut Store<HostState>) -> Result<()> {
    inst.call_run(store)?;
    Ok(())
}
