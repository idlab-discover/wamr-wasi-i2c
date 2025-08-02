use wasmtime::component::{ Component, HasSelf, Linker, Resource };
use wasmtime::{ Config, Engine, Store };
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::{ I2c, Error };

mod bindings;
use crate::bindings::wasi;

// I2C device wrapper voor thread safety
#[derive(Clone)]
struct I2cResourceCtx {
    hardware_guard: Arc<Mutex<I2cdev>>,
    bus_path: String,
}

impl I2cResourceCtx {
    fn new(bus_path: &str) -> I2cResourceCtx {
        let device = I2cdev::new(bus_path).expect("Host: Linux embedded halt i2c bus failed");

        I2cResourceCtx {
            hardware_guard: Arc::new(Mutex::new(device)),
            bus_path: bus_path.to_string(),
        }
    }
}

// Host state
struct HostState {
    /* wasi: WasiCtx,
    table: ResourceTable, */
    i2c_devices: HashMap<u32, I2cResourceCtx>,
    next_i2c_id: u32,
}

/* impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
} */

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

        let mut buffer = vec![0u8; len as usize];

        // Lock de I2C device en doe de read
        let mut i2c = i2c_res.hardware_guard.lock().unwrap();

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
        let device = self.i2c_devices.get(&resource_id).ok_or(wasi::i2c::i2c::ErrorCode::Other)?;

        // Lock de I2C device en doe de write
        let mut i2c = device.hardware_guard.lock().unwrap();

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
    fn get_i2c_bus(&mut self, bus_number: u32) -> Resource<wasi::i2c::i2c::I2c> {
        println!("Host: Called get_i2c_bus");
        let bus_path = format!("/dev/i2c-{}", bus_number);
        let dev = I2cResourceCtx::new(&bus_path);
        let id = self.next_i2c_id;
        self.next_i2c_id += 1;
        self.i2c_devices.insert(id, dev);
        Resource::new_own(id)
    }
}

// Main functie
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup engine en config
    println!("HOST: Setting up config");
    let mut config = Config::new();
    config.wasm_component_model(true);

    println!("HOST: Setting up Engine");
    let engine = Engine::new(&config)?;

    println!("HOST: Setting up Store");
    // Maak store met host state
    let mut store = Store::new(&engine, HostState {
        /* wasi: WasiCtxBuilder::new().build(),
        table: ResourceTable::new(), */
        i2c_devices: HashMap::new(),
        next_i2c_id: 0,
    });

    println!("HOST: Setting up Component");
    // Laad de component
    let component = Component::from_file(&engine, "wasmodules/guestp2.wasm")?;

    println!("HOST: Setting up Linker");
    // Link de WASI imports
    let mut linker = Linker::new(&engine);
    // wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    // Add onze I2C implementatie
    // println!("HOST: Setting up wasi:i2c link");
    // wasi::i2c::i2c::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut HostState| state)?;

    // Add de get_i2c_bus import
    println!("HOST: Setting up pingpong link");
    bindings::Pingpong::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut HostState| state)?;

    // Instantieer de component
    println!("HOST: Setting up pingpong Instance");
    let inst = bindings::Pingpong::instantiate(&mut store, &component, &linker)?;

    // Bind de exports
    println!("HOST: Setting up func call");
    inst.call_run(&mut store)?;

    // Call de run functie
    /* println!("[HOST] Calling guest run function...");
    match exports.call_run(&mut store) {
        Ok(result) => println!("[HOST] Guest returned: {:?}", result),
        Err(e) => println!("[HOST] Guest error: {:?}", e),
    } */

    Ok(())
}
