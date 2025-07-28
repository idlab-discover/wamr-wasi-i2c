#![no_main]

use wasip1_i2c::{ common::I2cAddress, guest::I2cResource };

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let data = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f]; // Says "hello"
    let slave_addr: I2cAddress = 0x0009;
    let device = I2cResource::new();
    match device.write(slave_addr, &data) {
        Ok(()) => {
            match device.read(slave_addr, data.len()) {
                Ok(read_data) => println!("Guest: Read data: {:?}", read_data),
                Err(code) => eprintln!("Guest: Read: {:?}", code),
            }
        }
        Err(code) => {
            eprintln!("Guest: Error: Write: {:?}", code);
        }
    }
}
