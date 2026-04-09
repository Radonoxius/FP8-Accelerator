use std::{ffi::c_void, ptr::null_mut};

pub mod errors;
pub mod mem;
pub mod ffi;

use libc::{MAP_FAILED, MAP_POPULATE, MAP_SHARED, O_RDWR, O_SYNC, PROT_READ, PROT_WRITE, close, mmap, munmap, open};

use crate::errors::DriverError;

const AXI_BRIDGE_BASE: usize = 0xC000_0000;
const BRIDGE_OFFSET: usize = 0;

const SPAN: usize = 0x1000;

pub type U128 = [u8; 16];

#[repr(C)]
pub struct Vfp8Accelerator {
    pub(crate) base_addr: *mut u32,
    mem_fd: i32
}

impl Vfp8Accelerator {
    pub fn take() -> Result<Self, DriverError> {
        let path = c"/dev/mem".as_ptr();
        let mem_fd = unsafe { open(path, O_RDWR | O_SYNC) };

        return if mem_fd < 0 {
            Err(DriverError::MemFdError)
        } else {
            let v_addr = unsafe {
                mmap(
                    null_mut(),
                    SPAN,
                    PROT_READ | PROT_WRITE,
                    MAP_SHARED | MAP_POPULATE,
                    mem_fd,
                    (AXI_BRIDGE_BASE + BRIDGE_OFFSET) as i64
                )
            };

            if v_addr == MAP_FAILED {
                unsafe { close(mem_fd) };
                Err(DriverError::MmapError)
            } else {
                Ok(
                    Self {
                        base_addr: v_addr as *mut u32,
                        mem_fd
                    }
                )
            }
        };
    }
}

impl Drop for Vfp8Accelerator {
    fn drop(&mut self) {
        unsafe {
            munmap(self.base_addr as *mut c_void, SPAN);
            close(self.mem_fd);
        }
    }
}