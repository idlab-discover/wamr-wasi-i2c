#![no_std]
#![no_main]

use wasip1_i2c_lib::{ common::I2cAddress, guest::I2cResource };

extern crate alloc;
use lol_alloc::{ AssumeSingleThreaded, FreeListAllocator };
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> = unsafe {
    AssumeSingleThreaded::new(FreeListAllocator::new())
};

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let data = [0x68u8, 0x65, 0x6c, 0x6c, 0x6f]; // Says "hello"
    let slave_addr: I2cAddress = 0x0009;
    let device = I2cResource::new();
    let _ = device.write(slave_addr, &data);
    let _ = device.read(slave_addr, data.len());
}
