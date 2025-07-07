#![no_std]

pub mod i2c {
    //use core::sync::atomic::{ AtomicU32 };
    use alloc::vec::Vec;
    extern crate alloc;
    pub type I2cAddress = u16;
    pub type I2cHandle = u32;

    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        #[link_name = "read"]
        unsafe fn host_read(_: I2cHandle, _: I2cAddress, _: u64, _: *mut u8) -> u8;
    }

    #[repr(u8)]
    #[derive(Debug)]
    pub enum NoAcknowledgeSource {
        Address,
        Data,
        Unknown,
    }

    impl NoAcknowledgeSource {
        pub unsafe fn lift(val: u8) -> NoAcknowledgeSource {
            // TODO: get this out, revert to only unsafe transmute
            if !cfg!(debug_assertions) {
                return unsafe { ::core::mem::transmute(val) };
            }

            match val {
                0 => NoAcknowledgeSource::Address,
                1 => NoAcknowledgeSource::Data,
                2 => NoAcknowledgeSource::Unknown,

                _ => panic!("invalid enum discriminant"),
            }
        }
    }

    #[derive(Debug)]
    pub enum ErrorCode {
        /// Bus error occurred. e.g. A START or a STOP condition is detected and
        /// is not located after a multiple of 9 SCL clock pulses.
        Bus,
        /// The arbitration was lost, e.g. electrical problems with the clock signal.
        ArbitrationLoss,
        /// A bus operation was not acknowledged, e.g. due to the addressed
        /// device not being available on the bus or the device not being ready
        /// to process requests at the moment.
        NoAcknowledge(NoAcknowledgeSource),
        /// The peripheral receive buffer was overrun.
        Overrun,
        /// A different error occurred.
        Other,
    }

    #[repr(transparent)]
    pub struct I2cResource {
        handle: I2cHandle,
    }

    impl I2cResource {
        pub fn new(handle: I2cHandle) -> Self {
            Self { handle: handle }
        }

        pub fn read(&self, address: I2cAddress, len: u64) -> Result<Vec<u8>, ErrorCode> {
            let mut read_buffer = Vec::<u8>::new();
            read_buffer.resize(len as usize, 0);
            let ptr = read_buffer.as_mut_ptr();

            let host_res = unsafe { host_read(self.handle, address, len, ptr) };

            let error_type = host_res >> 5; // take first 3 bits
            let error_variant = 31u8 & host_res; // take last 5 bits

            let final_result = match error_type {
                0 => Ok(read_buffer),
                1 => Err(ErrorCode::Bus),
                2 => Err(ErrorCode::ArbitrationLoss),
                3 =>
                    Err(
                        ErrorCode::NoAcknowledge(unsafe {
                            NoAcknowledgeSource::lift(error_variant)
                        })
                    ),
                4 => Err(ErrorCode::Overrun),
                _ => Err(ErrorCode::Other),
            };
            final_result
        }
    }
}
