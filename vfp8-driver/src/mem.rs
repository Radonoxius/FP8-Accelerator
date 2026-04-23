use crate::{FpReg, Vfp8Accelerator};

impl Vfp8Accelerator {
    #[inline(always)]
    pub(crate) unsafe fn read_from(&self, offset: usize) -> FpReg {
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

            core::mem::transmute::<[u32; 4], FpReg>([w0, w1, w2, w3])
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn write_to(&mut self, offset: usize, value: FpReg) {
        let addr = (self.base_addr as usize + offset) as *mut u32;
        let [w0, w1, w2, w3] = unsafe {
            core::mem::transmute::<FpReg, [u32; 4]>(value)
        };
    
        unsafe {
            core::arch::asm!(
                "stm {addr}, {{r0, r1, r2, r3}}",
                "dsb sy",
                addr = in(reg) addr,
                in("r0") w0,
                in("r1") w1,
                in("r2") w2,
                in("r3") w3,
                options(nostack, preserves_flags)
            );
        }
    }
}