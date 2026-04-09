use std::ffi::c_void;

use libc::{memcpy, memmove};

use crate::{U128, Vfp8Accelerator};

impl Vfp8Accelerator {
    pub fn read_reg_at(&self, offset: usize) -> U128 {
        let mut res = [0; 16];
        unsafe {
            if offset <= 0x1000 {
                memcpy(
                    &raw mut res as *mut c_void,
                    (self.base_addr as usize + offset) as *const c_void,
                    16
                );
            }
        }
        
        res
    }

    pub fn write_reg_at(&mut self, offset: usize, value: U128) {
        unsafe {
            if offset <= 0x1000 {
                memmove(
                    (self.base_addr as usize + offset) as *mut c_void,
                    &raw const value as *const c_void,
                    16
                );
            }
        }
    }
}