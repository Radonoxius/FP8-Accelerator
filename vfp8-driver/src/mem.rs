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
        let w2: u32;
        let w3: u32;

        unsafe {
            core::arch::asm!(
                "ldm {addr}, {{ r0, r1, r2, r3 }}",
                "dsb sy",
                addr = in(reg) addr,
                out("r0") w0,
                out("r1") w1,
                out("r2") w2,
                out("r3") w3,
                options(nostack, preserves_flags)
            );

            Ok(core::mem::transmute::<[u32; 4], U128>([w0, w1, w2, w3]))
        }
    }

    pub fn write_reg_at(&mut self, offset: usize, value: U128) -> Result<(), DriverError> {
        if offset > SPAN {
            return Err(DriverError::OutOfBounds);
        }
    
        let addr = (self.base_addr as usize + offset) as *mut u32;
        let [w0, w1, w2, w3] =
            unsafe { core::mem::transmute::<U128, [u32; 4]>(value) };
    
        unsafe {
            core::arch::asm!(
                "stm {addr}, {{r0, r1, r2, r3}}",
                "dsb sy",
                addr = in(reg) addr,
                in("r0") w0,
                in("r1") w1,
                in("r2") w2,
                in("r3") w3,
                options(nostack, preserves_flags),
            );
        }
    
        Ok(())
    }
}