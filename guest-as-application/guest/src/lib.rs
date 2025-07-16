#![no_std]
#![no_main]

use wasip1_i2c::i2c;

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

/* use core::alloc::{ GlobalAlloc, Layout };
#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return core::ptr::null_mut();
    }
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 1);
        ALLOCATOR.alloc(layout)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, size: usize) {
    if !ptr.is_null() && size > 0 {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, 1);
            ALLOCATOR.dealloc(ptr, layout);
        }
    }
} */

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let device = i2c::I2cResource::new();
    let _ = device.read(0xabcd, 3);
    // let device2 = i2c::I2cResource::new();
    // let _ = device2.read(0x1234, 3);
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
