use std::{ffi::c_void, ptr::null_mut};

pub mod errors;
pub mod mem;
pub mod ops;
pub mod ffi;

use libc::{MAP_FAILED, MAP_POPULATE, MAP_SHARED, MAP_SYNC, O_RDWR, O_SYNC, PROT_READ, PROT_WRITE, close, mmap, munmap, open};
use soft_fp8::Fp8;

use crate::errors::DriverError;

const AXI_BRIDGE_BASE: usize = 0xC000_0000;
const BRIDGE_OFFSET: usize = 0;

const OPERAND_REGISTER_OFFSET: usize = 0x00;
const OPCODE_REGISTER_OFFSET: usize = 0x10;
const RESULT_REGISTER_OFFSET: usize = 0x20;

const SPAN: usize = 0x1000;

pub type U128 = [u8; 16];

#[derive(Debug)]
#[repr(C)]
pub struct Vfp8Accelerator {
    pub(crate) base_addr: *mut u32,
    mem_fd: i32
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum Vfp8Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Inverse,
    Fma,

    Halt
}

pub type OperandPair = (Fp8, Fp8);

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
                    MAP_SHARED | MAP_POPULATE | MAP_SYNC,
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

impl Into<u8> for Vfp8Operation {
    fn into(self) -> u8 {
        return match self {
            Self::Add => 0b100_00000,
            Self::Subtract => 0b101_00000,
            Self::Multiply => 0b110_00000,
            Self::Divide => 0b111_00000,
            Self::Inverse => 0b001_00000,
            Self::Fma => 0b010_00000,

            Self::Halt => 0b000_00000
        }
    }
}