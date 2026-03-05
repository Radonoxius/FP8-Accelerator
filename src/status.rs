use crate::Fp8;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Zero,
    Infinity,
    NaN,

    Normal,
    Denormal
}

impl Status {
    pub fn is_zero(n: &Fp8) -> bool {
        if n.byte << 1 == 0 {
            true
        }
        else {
            false
        }
    }

    pub fn is_denormal(n: &Fp8) -> bool {
        todo!()
    }

    pub fn is_inf(n: &Fp8) -> bool {
        if n.get_exponent_bits() == 15 &&
            n.get_mantissa_bits() == 0 {
            true
        }
        else {
            false
        }
    }

    pub fn is_nan(n: &Fp8) -> bool {
        if n.get_exponent_bits() == 15 &&
            n.get_mantissa_bits() != 0 {
            true
        }
        else {
            false
        }
    }
}