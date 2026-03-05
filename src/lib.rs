use std::fmt::Debug;

use crate::status::Status;

pub mod status;

///Represents an 8 bit, E4M3 floating
///point number.
/// 
///**NOTE**: No denormal support.
#[derive(Clone, Copy)]
pub struct Fp8 {
    pub(crate) byte: u8
}

impl Fp8 {
    pub fn new(raw_value: u8) -> Fp8 {
        Fp8 { byte: raw_value }
    }

    pub fn get_sign_bit(&self) -> u8 {
        self.byte >> 7
    }

    pub fn get_exponent_bits(&self) -> u8 {
        (self.byte & 0b0111_1000) >> 3
    }

    pub fn get_exponent_value(&self) -> i8 {
        self.get_exponent_bits() as i8 - 8
    }

    pub fn get_mantissa_bits(&self) -> u8 {
        self.byte & 0b0000_0111
    }

    pub fn get_mantissa_value(&self) -> f32 {
        1.0 +
        0.5 * ((self.get_mantissa_bits() & 4) >> 2) as f32 +
        0.25 * ((self.get_mantissa_bits() & 2) >> 1) as f32 +
        0.125 * (self.get_mantissa_bits() & 1) as f32
    }
}

impl Debug for Fp8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} x 2 ^ {}", self.get_mantissa_value(), self.get_exponent_value())
    }
}