use crate::Fp8;

///Represents the current state of an FP8 number.
/// 
///Can also by used by operations for signaling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Zero,
    NaN,

    Subnormal,

    Normal,
}

impl State {
    fn is_zero(n: &Fp8) -> bool {
        if n.byte << 1 == 0 {
            true
        }
        else {
            false
        }
    }

    fn is_subnormal(n: &Fp8) -> bool {
        if n.get_exponent_bits() == 0 &&
            n.get_mantissa_bits() != 0 {
            true
        }
        else {
            false
        }
    }

    fn is_nan(n: &Fp8) -> bool {
        if n.get_exponent_bits() == 15 &&
            n.get_mantissa_bits() == 7 {
            true
        }
        else {
            false
        }
    }

    ///Get the current state of the given FP8 number.
    pub fn get(n: &Fp8) -> State {
        if Self::is_zero(n) {
            State::Zero
        }
        else if Self::is_nan(n) {
            State::NaN
        }
        else if Self::is_subnormal(n) {
            State::Subnormal
        }
        else {
            State::Normal
        }
    }
}