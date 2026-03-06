use std::fmt::Debug;

use crate::state::State;

pub mod state;

///Represents an 8 bit, E4M3 floating
///point number.
///
///This format is defined by Nvidia, Arm and Intel for ML use.
///
///**NOTE**: No subnormal support.
#[derive(Clone, Copy)]
pub struct Fp8 {
    pub(crate) byte: u8
}

impl Fp8 {
    ///Create a new FP8 number from its components.
    pub fn new(
        sign_bit: u8,
        exponent_bits: u8,
        mantissa_bits: u8
    ) -> Self {
        Self {
            byte: (sign_bit << 7) |
                (exponent_bits << 3) |
                mantissa_bits
        }
    }

    ///Create 0.0.
    pub fn zero() -> Self {
        Self { byte: 0 }
    }

    ///Create NaN.
    pub fn nan() -> Self {
        Self { byte: 0b0111_1111 }
    }

    ///Flip the sign bit of the FP8 number.
    /// 
    ///Valid for all states, including NaN and zero.
    pub fn flip_sign(&mut self) {
        self.byte ^= 0b1000_0000;
    }

    ///Get the sign bit of the FP8 number.
    pub fn get_sign_bit(&self) -> u8 {
        self.byte >> 7
    }

    ///Get the exponent bits of the FP8 number.
    pub fn get_exponent_bits(&self) -> u8 {
        (self.byte & 0b0111_1000) >> 3
    }

    ///Get the mantissa bits of the FP8 number.
    pub fn get_mantissa_bits(&self) -> u8 {
        self.byte & 0b0000_0111
    }

    ///Get the exponent value (with added bias) of the FP8 number.
    ///
    ///Returns `None` if the number is zero or NaN.
    pub fn get_exponent_value(&self) -> Option<i8> {
        match State::get(self) {
            State::Zero => {
                None
            },

            State::NaN => {
                None
            },

            State::Subnormal => {
                Some(-6)
            },

            State::Normal => {
                Some(self.get_exponent_bits() as i8 - 7)
            }
        }
    }

    ///Get the mantissa value (as a regular float) of the FP8 number.
    ///
    ///Returns `None` if the number is NaN.
    pub fn get_mantissa_value(&self) -> Option<f32> {
        match State::get(self) {
            State::Zero => {
                Some(0.0)
            },

            State::NaN => {
                None
            },

            State::Subnormal => {
                Some(
                    0.5 * ((self.get_mantissa_bits() & 4) >> 2) as f32 +
                    0.25 * ((self.get_mantissa_bits() & 2) >> 1) as f32 +
                    0.125 * (self.get_mantissa_bits() & 1) as f32
                )
            },

            State::Normal => {
                Some(
                    1.0 +
                    0.5 * ((self.get_mantissa_bits() & 4) >> 2) as f32 +
                    0.25 * ((self.get_mantissa_bits() & 2) >> 1) as f32 +
                    0.125 * (self.get_mantissa_bits() & 1) as f32
                )
            }
        }
    }
}

impl Debug for Fp8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mantissa = self.get_mantissa_value();
        let exponent = self.get_exponent_value();

        return {
            if let None = mantissa && None == exponent {
                if self.get_sign_bit() == 1 {
                    write!(f, "-NaN")
                }
                else {
                    write!(f, "NaN")
                }
            }
            else if let Some(_) = mantissa && let None = exponent {
                if self.get_sign_bit() == 1 {
                    write!(f, "-0.0")
                }
                else {
                    write!(f, "0.0")
                }
            }
            else if let None = mantissa && let Some(_) = exponent {
                unreachable!()
            }
            else {
                if self.get_sign_bit() == 1 {
                    write!(
                        f,
                        "-{} x 2 ^ {}",
                        self.get_mantissa_value().unwrap(),
                        self.get_exponent_value().unwrap()
                    )
                }
                else {
                    write!(
                        f,
                        "{} x 2 ^ {}",
                        self.get_mantissa_value().unwrap(),
                        self.get_exponent_value().unwrap()
                    )
                }
            }
        };
    }
}

impl From<u8> for Fp8 {
    fn from(value: u8) -> Self {
        Self { byte: value }
    }
}