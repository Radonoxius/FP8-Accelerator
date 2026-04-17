use std::ptr::write_volatile;

use crate::{SPAN, U128, Vfp8Accelerator, errors::DriverError};

impl Vfp8Accelerator {
    #[inline(always)]
    pub fn read_reg_at(&self, offset: usize) -> Result<U128, DriverError> {
        if offset > SPAN {
            return Err(DriverError::OutOfBounds);
        }

        // Ensure the address is at least 4-byte aligned (required for ldm)
        let addr = (self.base_addr as usize + offset) as *const u32;

        let w0: u32;
        let w1: u32;
        //let w2: u32;
        //let w3: u32;

        unsafe {
            //dmb osh
            //ldrd r0, r1, [{addr}]
            core::arch::asm!(
                "dsb sy",
                // The {{ }} escapes the braces for the assembler
                // Using explicit registers ensures w0 is the lowest register index
                //"ldm {addr}, {{ r0, r1, r2, r3 }}",
                "ldm {addr}, {{ r0, r1 }}",
                "dsb sy",
                addr = in(reg) addr,
                // Bind the variables to the specific registers used in ldm
                out("r0") w0,
                out("r1") w1,
                //out("r2") w2,
                //out("r3") w3,
                options(nostack, preserves_flags),
            );

            //Ok(core::mem::transmute::<[u32; 4], U128>([w0, w1, w2, w3]))
            Ok(core::mem::transmute::<[u32; 4], U128>([w0, w1, 0, 0]))
        }
    }

    #[inline(always)]
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

    //pub fn write_reg_at(&mut self, offset: usize, value: U128) -> Result<(), DriverError> {
    //    if offset > SPAN {
    //        return Err(DriverError::OutOfBounds);
    //    }
    //
    //    let addr = (self.base_addr as usize + offset) as *mut u32;
    //
    //    // Destructure the 128-bit value into 32-bit chunks for the registers.
    //    // If U128 is a [u32; 4], you can access it directly. 
    //    // If it's a u128, use transmute.
    //    let [w0, w1, w2, w3] = unsafe { core::mem::transmute::<U128, [u32; 4]>(value) };
    //
    //    unsafe {
    //        core::arch::asm!(
    //            "dsb sy",               // Ensure previous memory ops are complete
    //            "stm {addr}, {{r0, r1, r2, r3}}", // Burst write 128 bits
    //            "dsb sy",               // Ensure the write hits the bridge before continuing
    //            addr = in(reg) addr,
    //            in("r0") w0,
    //            in("r1") w1,
    //            in("r2") w2,
    //            in("r3") w3,
    //            options(nostack, preserves_flags),
    //        );
    //    }
    //
    //    Ok(())
    //}
}