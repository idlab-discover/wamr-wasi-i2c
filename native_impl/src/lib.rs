use std::time::Instant;

use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;

const BFR: [u8; 5] = [0x68u8, 0x65, 0x6c, 0x6c, 0x6f];
const ADDR: u8 = 0x0027;

pub fn setup() -> I2cdev {
    I2cdev::new("/dev/i2c-1").unwrap()
}

pub fn pingpong(bus: &mut I2cdev) {
    for _ in 0..100 {
        let now = Instant::now();
        bus.write(ADDR, &BFR).unwrap();
        println!("native,write,{}", now.elapsed().as_nanos());
    }

    for _ in 0..100 {
        let now = Instant::now();
        let mut buf = [0; 5];
        bus.read(ADDR, &mut buf).unwrap();
        println!("native,read,{}", now.elapsed().as_nanos());
    }
}
