pub mod i2c {
    use core::mem::MaybeUninit;
    use std::mem::transmute;
    pub type I2cAddress = u16;
    pub type I2cResourceHandle = u32;

    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        #[link_name = "host_open"]
        unsafe fn host_open() -> I2cResourceHandle;
        #[link_name = "host_read"]
        unsafe fn host_read(_: I2cResourceHandle, _: I2cAddress, _: usize, _: *mut u8) -> u8;
        #[link_name = "host_write"]
        unsafe fn host_write(_: I2cResourceHandle, _: I2cAddress, _: usize, _: *const u8) -> u8;
    }

    #[repr(u8)]
    #[derive(Debug)]
    pub enum NoAcknowledgeSource {
        Address,
        Data,
        Unknown,
    }

    impl NoAcknowledgeSource {
        pub fn lift(val: u8) -> NoAcknowledgeSource {
            match val {
                0 => NoAcknowledgeSource::Address,
                1 => NoAcknowledgeSource::Data,
                _ => NoAcknowledgeSource::Unknown,

                // TODO: decide on what to do, panic or Unknown no-ack: _ => panic!("invalid enum discriminant"),
            }
        }

        pub fn lower(&self) -> u8 {
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
        /// No error occurred. Operation was succesful.
        None,
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

    impl ErrorCode {
        pub fn lower(&self) -> u8 {
            match self {
                ErrorCode::None => 0b000_00000,
                ErrorCode::Bus => 0b001_00000,
                ErrorCode::ArbitrationLoss => 0b010_00000,
                ErrorCode::NoAcknowledge(source) => {
                    let no_ack_bits = source.lower();
                    0b011_00000 | no_ack_bits
                }
                ErrorCode::Overrun => 0b100_00000,
                _ => 0b101_00000,
            }
        }

        pub fn lift(val: u8) -> ErrorCode {
            let error_type = val >> 5; // take first 3 bits only
            let error_variant = 0b000_11111 & val; // take last 5 bits only
            match error_type {
                0 => ErrorCode::None,
                1 => ErrorCode::Bus,
                2 => ErrorCode::ArbitrationLoss,
                3 => ErrorCode::NoAcknowledge(NoAcknowledgeSource::lift(error_variant)),
                4 => ErrorCode::Overrun,
                _ => ErrorCode::Other,
            }
        }
    }

    #[repr(transparent)]
    pub struct I2cResource {
        // TODO: Resources worden eigenlijk nog niet correct behandeld
        //      - We moeten nog checken op terug vrijkomen van resources
        //      - Het doorgeven van resources is geen mogelijkheid (ownership op handle)
        //      - Nog niets van error handling
        handle: I2cResourceHandle,
    }

    impl I2cResource {
        pub fn new() -> Self {
            Self { handle: unsafe { host_open() } }
        }

        pub fn read(&self, address: I2cAddress, len: usize) -> Result<Vec<u8>, ErrorCode> {
            let mut read_buffer: Vec<MaybeUninit<u8>> = Vec::with_capacity(len as usize);

            let host_res = unsafe {
                read_buffer.set_len(len as usize);
                host_read(self.handle, address, len, read_buffer.as_mut_ptr() as *mut u8)
            };

            let output = match ErrorCode::lift(host_res) {
                ErrorCode::None => Ok(unsafe { transmute(read_buffer) }),
                e => Err(e),
            };
            output
        }

        pub fn write(&self, address: I2cAddress, data: &Vec<u8>) -> Result<(), ErrorCode> {
            let host_res = unsafe { host_write(self.handle, address, data.len(), data.as_ptr()) };

            match ErrorCode::lift(host_res) {
                ErrorCode::None => Ok(()),
                n => Err(n),
            }
        }
    }

    // TODO: Implement Drop function
}
