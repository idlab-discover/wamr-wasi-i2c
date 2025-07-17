#![no_std]

pub mod i2c {
    use alloc::vec::Vec;
    extern crate alloc;
    pub type I2cAddress = u16;
    pub type I2cResourceHandle = u32;

    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        #[link_name = "host_open"]
        unsafe fn host_open() -> I2cResourceHandle;
        #[link_name = "host_read"]
        unsafe fn host_read(_: I2cResourceHandle, _: I2cAddress, _: usize, _: *mut u8); // TODO: Als we wel een u8 returnen, kunnen we WAMR errors returnen?
        // #[link_name = "host_write"]
        // unsafe fn host_write(_: I2cResourceHandle, _: I2cAddress, _: usize, _: *const u8) -> u8;
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
        pub fn lower(self) -> u8 {
            match self {
                ErrorCode::NoAcknowledge(source) => {
                    let no_ack_bits = unsafe { source.lower() };
                    (2u8 << 5) | no_ack_bits
                }
                _ => ,
            }
        }
    } */

    #[repr(transparent)]
    pub struct I2cResource {
        handle: I2cResourceHandle,
    }

    impl I2cResource {
        pub fn new() -> Self {
            Self { handle: unsafe { host_open() } }
        }

        pub fn read(&self, address: I2cAddress, len: usize) -> Result<Vec<u8>, ErrorCode> {
            // Return Area is 3 pointers
            //  1: Geeft aan 0=Ok, 1=Err
            //  2: Point naar begin van de data
            //  3: Point naar lengte van de data
            //  OF 2+3 gebruikt voor Errorcode + NoAckSource
            #[cfg_attr(target_pointer_width = "64", repr(align(8)))]
            #[cfg_attr(target_pointer_width = "32", repr(align(4)))]
            struct ReturnArea(
                [::core::mem::MaybeUninit<u8>; 3 * ::core::mem::size_of::<*const u8>()],
            );
            let mut return_area = ReturnArea(
                [::core::mem::MaybeUninit::uninit(); 3 * ::core::mem::size_of::<*const u8>()]
            );
            let return_area_begin = return_area.0.as_mut_ptr().cast::<u8>();

            unsafe {
                host_read(self.handle, address, len, return_area_begin);
            }

            let return_discriminant = unsafe { *return_area_begin.add(0).cast::<u8>() }; // 0 = Ok, 1 = Err
            let output_result = match return_discriminant {
                0 => {
                    let read_data = unsafe {
                        let read_data_start = *return_area_begin
                            .add(::core::mem::size_of::<*const u8>())
                            .cast::<*mut u8>();
                        let read_data_size = *return_area_begin
                            .add(2 * ::core::mem::size_of::<*const u8>())
                            .cast::<usize>();
                        let data_length = read_data_size;

                        Vec::from_raw_parts(read_data_start.cast(), data_length, data_length)
                    };
                    Ok(read_data)
                }
                _ => {
                    let error_code = {
                        let error_discriminant = unsafe {
                            *return_area_begin.add(::core::mem::size_of::<*const u8>()).cast::<u8>()
                        };
                        let full_error_code = match error_discriminant {
                            0 => { ErrorCode::Bus }
                            1 => { ErrorCode::ArbitrationLoss }
                            2 => {
                                let no_ack_source = {
                                    let no_ack_discriminant = unsafe {
                                        *return_area_begin
                                            .add(1 + 1 * ::core::mem::size_of::<*const u8>())
                                            .cast::<u8>()
                                    };

                                    NoAcknowledgeSource::lift(no_ack_discriminant)
                                };
                                ErrorCode::NoAcknowledge(no_ack_source)
                            }
                            3 => { ErrorCode::Overrun }
                            _ => ErrorCode::Other,
                        };

                        full_error_code
                    };
                    Err(error_code)
                }
            };
            output_result
        }

        /* pub fn read(&self, address: I2cAddress, len: usize) -> Result<Vec<u8>, ErrorCode> {
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
        } */
    }

    // TODO: Implement Drop function
}
