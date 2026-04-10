use std::ptr::{read_volatile, write_volatile};

use crate::{SPAN, U128, Vfp8Accelerator, errors::DriverError};

impl Vfp8Accelerator {
    pub fn read_reg_at(&self, offset: usize) -> Result<U128, DriverError> {
        unsafe {
            return if offset <= SPAN {
                Ok(
                    read_volatile(
                        (self.base_addr as usize + offset) as *mut U128
                    )
                )
            } else {
                Err(DriverError::OutOfBounds)
            }
        }
    }

    pub fn write_reg_at(&mut self, offset: usize, value: U128) -> Result<(), DriverError> {
        unsafe {
            if offset <= SPAN {
                write_volatile(
                    (self.base_addr as usize + offset) as *mut U128,
                    value,
                );

                Ok(())
            } else {
                Err(DriverError::OutOfBounds)
            }
        }
    }
}