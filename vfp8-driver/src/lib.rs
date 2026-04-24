use std::{ffi::c_void, ptr::null_mut};

pub mod errors;
pub mod mem;
pub mod ops;
pub mod ffi;

use libc::{MAP_FAILED, MAP_POPULATE, MAP_SHARED, MAP_SYNC, O_RDWR, O_SYNC, PROT_READ, PROT_WRITE, close, mmap, munmap, open};

use crate::errors::DriverError;

const AXI_BRIDGE_BASE: usize = 0xC000_0000;
const DEVICE_OFFSET: usize = 0x00;
const DEVICE_SPAN: usize = 0xFF;

const OPERAND_REGISTER: usize = 0x00;
const OPCODE_REGISTER: usize = 0x10;
const RESULT_REGISTER: usize = 0x20;

pub type FpReg = [u8; 16];

///Represents the vfp8 accelerator device
#[derive(Debug)]
#[repr(C)]
pub struct Vfp8Accelerator {
    pub(crate) base_addr: *mut u32,
    mem_fd: i32
}

impl Vfp8Accelerator {
    ///Initialize the vfp8 accelerator and take ownership of it
    pub fn take<'a>() -> Result<Self, DriverError<'a>> {
        let path = c"/dev/mem".as_ptr();
        let mem_fd = unsafe { open(path, O_RDWR | O_SYNC) };

        return if mem_fd < 0 {
            Err(DriverError::MemFdError)
        } else {
            let v_addr = unsafe {
                mmap(
                    null_mut(),
                    DEVICE_SPAN,
                    PROT_READ | PROT_WRITE,
                    MAP_SHARED | MAP_POPULATE | MAP_SYNC,
                    mem_fd,
                    (AXI_BRIDGE_BASE + DEVICE_OFFSET) as i64
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
            munmap(self.base_addr as *mut c_void, DEVICE_SPAN);
            close(self.mem_fd);
        }
    }
}

///Represents the arithmetic operator used in mathematical expressions,
///that are supported by the vfp8 accelerator
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum Vfp8Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Inverse,

    Idle
}

impl Into<u8> for Vfp8Operator {
    fn into(self) -> u8 {
        return match self {
            Self::Add => 0b100_00000,
            Self::Subtract => 0b101_00000,
            Self::Multiply => 0b110_00000,
            Self::Divide => 0b111_00000,

            Self::Inverse => 0b001_00000,

            Self::Idle => 0b000_00000
        }
    }
}