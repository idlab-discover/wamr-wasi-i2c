use crate::common::*;

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    #[link_name = "host_open"]
    unsafe fn host_open() -> I2cResourceHandle;
    #[link_name = "host_read"]
    unsafe fn host_read(_: I2cResourceHandle, _: I2cAddress, _: usize, _: *mut u8) -> u8;
    #[link_name = "host_write"]
    unsafe fn host_write(_: I2cResourceHandle, _: I2cAddress, _: usize, _: *const u8) -> u8;
    #[link_name = "host_close"]
    unsafe fn host_close(_: I2cResourceHandle);
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

    pub fn read(
        &self,
        address: I2cAddress,
        len: usize
    ) -> Result<alloc_crate::vec::Vec<u8>, ErrorCode> {
        pub use alloc_crate::vec::Vec;
        let mut read_buffer: Vec<core::mem::MaybeUninit<u8>> = Vec::with_capacity(len as usize);

        let host_res = unsafe {
            read_buffer.set_len(len as usize);
            host_read(self.handle, address, len, read_buffer.as_mut_ptr() as *mut u8)
        };

        let output = match ErrorCode::lift(host_res) {
            ErrorCode::None => Ok(unsafe { core::mem::transmute(read_buffer) }),
            e => Err(e),
        };
        output
    }

    pub fn write(&self, address: I2cAddress, data: &[u8]) -> Result<(), ErrorCode> {
        let host_res = unsafe { host_write(self.handle, address, data.len(), data.as_ptr()) };

        match ErrorCode::lift(host_res) {
            ErrorCode::None => Ok(()),
            n => Err(n),
        }
    }
}

impl Drop for I2cResource {
    fn drop(&mut self) {
        unsafe {
            host_close(self.handle);
        }
    }
}

extern crate alloc as alloc_crate;
