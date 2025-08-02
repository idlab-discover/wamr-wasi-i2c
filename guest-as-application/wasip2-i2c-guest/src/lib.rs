#[allow(warnings)]
mod bindings;

use bindings::Guest;

use crate::bindings::{ get_i2c_bus };

struct PingPongComponent;

impl Guest for PingPongComponent {
    /// Say hello!
    fn run() {
        let dev = get_i2c_bus(1);
        dev.write(0x09, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]).expect("Guest: Write error");
        let rr = dev.read(0x09, 5).expect("Guest: Failed reading data");
    }
}

bindings::export!(PingPongComponent with_types_in bindings);
