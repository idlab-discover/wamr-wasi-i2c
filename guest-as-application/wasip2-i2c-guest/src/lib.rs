#[allow(warnings)]
mod bindings;

use bindings::Guest;

use crate::bindings::{ get_i2c_bus };

struct PingPongComponent;

impl Guest for PingPongComponent {
    /// Say hello!
    fn run() {
        let dev = get_i2c_bus();
        let _ = dev.write(0x09, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]);
        let _ = dev.read(0x09, 5);
    }
}

bindings::export!(PingPongComponent with_types_in bindings);
