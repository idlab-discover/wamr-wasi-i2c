#[allow(warnings)]
mod bindings;

use bindings::Guest;

use crate::bindings::wasi::i2c::i2c;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn run() {
        let dev = unsafe { i2c::I2c::from_handle(0x1u32) };
        dev.write(0x09, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]).expect("Guest: Write error");
        let rr = dev.read(0x09, 5);
        println!("Guest: Read Result: {:?}", rr);
    }
}

bindings::export!(Component with_types_in bindings);
