// no_std isn't really possible here. wit-bindgen depends on std. We could write our own bindings file, but the whole point of using wit files is that bindings get generated automatically for ease of use.
#[allow(warnings)]
mod bindings;

use std::time::Instant;

use bindings::Guest;

use crate::bindings::get_i2c_bus;

struct PingPongComponent;

const ADDRESS: u16 = 0x0027;

impl Guest for PingPongComponent {
    fn setup() {
        let _ = get_i2c_bus().write(ADDRESS, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    }
    /// Say hello!
    fn run() {
        let device = get_i2c_bus();

        for _ in 0..100 {
            let now = Instant::now();
            let _ = device.write(ADDRESS, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]);
            println!("wasmtime,write,{}", now.elapsed().as_nanos());
        }

        for _ in 0..100 {
            let now = Instant::now();
            let _ = device.read(ADDRESS, 5);
            println!("wasmtime,read,{}", now.elapsed().as_nanos());
        }
    }
}

bindings::export!(PingPongComponent with_types_in bindings);
