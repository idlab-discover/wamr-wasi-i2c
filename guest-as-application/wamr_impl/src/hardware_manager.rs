use std::sync::{ LazyLock, Mutex };
use linux_embedded_hal::{ I2cdev };

pub struct I2cHardware {
    pub bus: I2cdev,
}

pub static I2C_HARDWARE_MANAGER: LazyLock<Mutex<I2cHardware>> = LazyLock::new(|| {
    Mutex::new(I2cHardware { bus: I2cdev::new("/dev/i2c-1").unwrap() })
});
