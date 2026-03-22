use std::{ffi::c_void, ptr::null_mut};

use libc::{MAP_FAILED, MAP_SHARED, O_RDWR, O_SYNC, PROT_READ, PROT_WRITE, close, mmap, munmap, open};

use crate::{LW_BRIDGE_BASE, SPAN, Vfp8Accelerator};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() -> Vfp8Accelerator {
    let device = Vfp8Accelerator {
        base_addr: null_mut(),
        mem_fd: 0
    };

    let path = c"/dev/mem".as_ptr();
    let mem_fd = unsafe { open(path, O_RDWR | O_SYNC) };

    if mem_fd >= 0 {
        let v_addr = unsafe {
            mmap(
                null_mut(),
                SPAN,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                mem_fd,
                LW_BRIDGE_BASE as i64
            )
        };

        if v_addr != MAP_FAILED {
            return Vfp8Accelerator {
                base_addr: v_addr as *mut u32,
                mem_fd
            };
        } else {
            unsafe {
                close(mem_fd);
            }
            println!("An error occured while obtaining virtual address of the bus. Try running the program as root.");
        }
    }

    println!("An error occured while opening /dev/mem as read-write. Try running the program as root.");
    device
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy(device: Vfp8Accelerator) {
    unsafe {
        munmap(device.base_addr as *mut c_void, SPAN);
        close(device.mem_fd);
    }
}