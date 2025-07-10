#![no_std]

pub mod i2c {
    use core::mem::MaybeUninit;
    use alloc::vec::Vec;
    extern crate alloc;
    pub type I2cAddress = u16;
    pub type I2cHandle = u32;

    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        #[link_name = "host_read"]
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
            match val {
                0 => NoAcknowledgeSource::Address,
                1 => NoAcknowledgeSource::Data,
                _ => NoAcknowledgeSource::Unknown,

                // TODO: decide on what to do, panic or Unknown no-ack: _ => panic!("invalid enum discriminant"),
            }
        }

        pub unsafe fn lower(self) -> u8 {
            match self {
                NoAcknowledgeSource::Address => 0,
                NoAcknowledgeSource::Data => 1,
                _ => 2,

                // TODO: decide on what to do, panic or Unknown no-ack: _ => panic!("invalid enum discriminant"),
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

    // TODO: ErrorCode is nu in totaal 16 bits, maar ik gebruik eigenlijk maar 8 bits, pas ik dit aan zoals de wit-bindgen c code = constante waarde voor iedere errormogelijkheid
    /* impl ErrorCode {
        pub fn unlift(self) -> u8 {
            match self {
                ErrorCode::NoAcknowledge(source) => {
                    let no_ack_bits = unsafe { source.unlift() };
                    (2u8 << 5) | no_ack_bits
                }
                _ => ,
            }
        }
    } */

    #[repr(transparent)]
    pub struct I2cResource {
        handle: I2cHandle,
    }

    impl I2cResource {
        // TODO: define how handle should be generated or handled
        pub fn new(handle: I2cHandle) -> Self {
            Self { handle: handle }
        }

        pub fn read(&self, address: I2cAddress, len: u64) -> Result<Vec<u8>, ErrorCode> {
            let mut read_buffer: Vec<MaybeUninit<u8>> = Vec::with_capacity(len as usize);

            let host_res = unsafe {
                read_buffer.set_len(len as usize);
                let res = host_read(self.handle, address, len, read_buffer.as_mut_ptr() as *mut u8);
                core::hint::black_box(res)
            };

            let error_type = host_res >> 5; // take first 3 bits only
            let error_variant = 0b000_11111 & host_res; // take last 5 bits only

            let final_result = match error_type {
                0 => Ok(unsafe { core::mem::transmute(read_buffer) }),
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
