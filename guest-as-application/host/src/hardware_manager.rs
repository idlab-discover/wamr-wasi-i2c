use std::sync::{ LazyLock, Mutex };
use linux_embedded_hal::{ i2cdev, I2cdev };

pub struct I2cHardware {
    pub bus: I2cdev,
}

pub static I2C_HARDWARE_MANAGER: LazyLock<Mutex<Option<I2cHardware>>> = LazyLock::new(|| {
    Mutex::new(None)
});

pub fn init_i2c_hardware(device_path: &str) -> Result<(), i2cdev::linux::LinuxI2CError> {
    let i2c = I2cdev::new(device_path)?;
    let mut hardware = I2C_HARDWARE_MANAGER.lock().unwrap();
    *hardware = Some(I2cHardware { bus: i2c });
    Ok(())
}
