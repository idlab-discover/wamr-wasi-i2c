use wasmtime::component::{ Component, HasSelf, Linker };
use wasmtime::{ Config, Engine, Store };
use std::sync::{ LazyLock, Mutex };
use linux_embedded_hal::I2cdev;

use anyhow::Result;

mod bindings;
mod wasmtime_manager;

use wasmtime_manager::HostState;

static I2C_BUS: LazyLock<Mutex<I2cdev>> = LazyLock::new(|| {
    Mutex::new(I2cdev::new("/dev/i2c-1").unwrap())
});

pub struct PingPongRunner {
    _config: Config,
    _engine: Engine,
    store: Store<HostState>,
    _component: Component,
    _linker: Linker<HostState>,
    instance: bindings::Pingpong,
}

impl PingPongRunner {
    pub fn new() -> Result<PingPongRunner> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        let mut store = Store::new(&engine, HostState::new());

        let component = Component::from_file(&engine, "wasmodules/guestp2.wasm")?;

        let mut linker = Linker::new(&engine);

        bindings::Pingpong::add_to_linker::<_, HasSelf<_>>(
            &mut linker,
            |state: &mut HostState| state
        )?;

        let instance = bindings::Pingpong::instantiate(&mut store, &component, &linker)?;
        Ok(PingPongRunner {
            _config: config,
            _engine: engine,
            store,
            _component: component,
            _linker: linker,
            instance,
        })
    }

    pub fn pingpong(&mut self) {
        self.instance.call_run(&mut self.store).unwrap();
    }
}
