use embedded_hal::i2c::I2c;
use linux_embedded_hal::{ I2cdev };
pub fn setup() -> I2cdev {
    I2cdev::new("/dev/i2c-1").unwrap()
}

pub fn pingpong(bus: &mut I2cdev) {
    let bfr = [0x68u8, 0x65, 0x6c, 0x6c, 0x6f];
    let slave_addr = 0x0009 as u8;
    bus.write(slave_addr, &bfr).unwrap();
}
