#![no_main]

use wasip1_i2c::guest::I2cResource;

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let device = I2cResource::new();
    let device2 = I2cResource::new();
    let res = device.read(0xabcd, 3);
    if let Ok(data) = res {
        println!("Guest: Read data: {:?}", data);
    }
    // let _ = device2.read(0x1234, 3);
    let _ = device.write(0x5678, &vec![0x12, 0xac, 0xce]);
    /* match device.read(0xabcd, 3) {
        Ok(res) => 100,
        Err(err_code) =>
            match err_code {
                ErrorCode::Bus => 0b000_00000,
                ErrorCode::ArbitrationLoss => 0b001_00000,
                ErrorCode::NoAcknowledge(no_acknowledge_source) =>
                    match no_acknowledge_source {
                        i2c::NoAcknowledgeSource::Address => 0b010_00000,
                        i2c::NoAcknowledgeSource::Data => 0b010_00001,
                        i2c::NoAcknowledgeSource::Unknown => 0b010_00010,
                    }
                ErrorCode::Overrun => 0b011_00000,
                ErrorCode::Other => 0b100_00000,
            }
    }; */
}
