// #![no_std]
#![no_main]

use std::time::Instant;

use wasip1_i2c_lib::{common::I2cAddress, guest::I2cResource};

// extern crate alloc;
// use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};
// #[global_allocator]
// static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
//     unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

// use core::panic::PanicInfo;
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

const DATA: [u8; 5] = [0x68u8, 0x65, 0x6c, 0x6c, 0x6f]; // hello
const SLAVE_ADDR: I2cAddress = 0x0027;

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let device = I2cResource::new();

    for _ in 0..100 {
        let now = Instant::now();
        let _ = device.write(SLAVE_ADDR, &DATA);
        println!("wamr,write,{}", now.elapsed().as_nanos());
    }

    for _ in 0..100 {
        let now = Instant::now();
        let _ = device.read(SLAVE_ADDR, DATA.len());
        println!("wamr,read,{}", now.elapsed().as_nanos());
    }
}

// #[unsafe(no_mangle)]
// /// Write to the device to initialize it
// pub extern "C" fn setup() {
//     let device = I2cResource::new();
//     let _ = device.write(SLAVE_ADDR, &DATA);
// }
//
// #[unsafe(no_mangle)]
// pub extern "C" fn execute() {
//     let device = I2cResource::new();
//     let _ = device.write(SLAVE_ADDR, &DATA);
//     let _ = device.read(SLAVE_ADDR, DATA.len());
// }
