use std::{f32, fmt::Debug};

use crate::state::State;

pub mod state;

pub mod addition;
pub mod subtraction;

///Represents an 8 bit, E4M3 floating
///point number.
///
///This format is defined by Nvidia, Arm and Intel for ML use.
///
///**NOTE**: No subnormal arithmetic support.
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

    ///Creates +0.0.
    pub fn zero() -> Self {
        Self { byte: 0 }
    }

    ///Creates +NaN.
    pub fn nan() -> Self {
        Self { byte: 0b0111_1111 }
    }

    ///Flip the sign bit of the FP8 number.
    /// 
    ///Valid for all states, including NaN and zero.
    pub fn flip_sign(self) -> Self {
        Self {
            byte: self.byte ^ 0b1000_0000
        }
    }

    ///Get the sign bit of the FP8 number.
    pub fn sign_bit(&self) -> u8 {
        self.byte >> 7
    }

    ///Returns true if the number is positive.
    ///
    ///Valid for all states, including NaN and zero.
    pub fn is_positive(&self) -> bool {
        return if self.sign_bit() == 0 {
            true
        } else {
            false
        }
    }

    ///Get the exponent bits of the FP8 number.
    pub fn exponent_bits(&self) -> u8 {
        (self.byte & 0b0111_1000) >> 3
    }

    ///Get the mantissa bits of the FP8 number.
    pub fn mantissa_bits(&self) -> u8 {
        self.byte & 0b0000_0111
    }

    ///Get the exponent value (with bias) of the FP8 number.
    ///
    ///Returns `None` if the number is zero or NaN.
    pub fn exponent_value(&self) -> Option<i8> {
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
                Some(self.exponent_bits() as i8 - 7)
            }
        }
    }

    ///Get the (absolute) mantissa value of the FP8 number.
    ///
    ///For Normals, the result includes
    ///the implicit bit, 1 at bit index 3.
    /// 
    ///Returns `None` if the number is NaN.
    pub fn mantissa_value(&self) -> Option<u8> {
        match State::get(self) {
            State::Zero => {
                Some(0)
            },

            State::NaN => {
                None
            },

            State::Subnormal => {
                Some(self.mantissa_bits())
            },

            State::Normal => {
                Some(8 | self.mantissa_bits())
            }
        }
    }

    ///Get the (signed) mantissa value of the FP8 number as a float.
    ///
    ///Returns `None` if the number is NaN.
    pub fn mantissa_value_as_float(&self) -> Option<f32> {
        match State::get(self) {
            State::Zero => {
                if self.is_positive() {
                    Some(0.0)
                } else {
                    Some(-0.0)
                }
            },

            State::NaN => {
                None
            },

            State::Subnormal => {
                if self.is_positive() {
                    Some(
                        0.5 * ((self.mantissa_bits() & 4) >> 2) as f32 +
                        0.25 * ((self.mantissa_bits() & 2) >> 1) as f32 +
                        0.125 * (self.mantissa_bits() & 1) as f32
                    )
                } else {
                    Some(
                        -(
                            0.5 * ((self.mantissa_bits() & 4) >> 2) as f32 +
                            0.25 * ((self.mantissa_bits() & 2) >> 1) as f32 +
                            0.125 * (self.mantissa_bits() & 1) as f32
                        )
                    )
                }
            },

            State::Normal => {
                if self.is_positive() {
                    Some(
                        1.0 +
                        0.5 * ((self.mantissa_bits() & 4) >> 2) as f32 +
                        0.25 * ((self.mantissa_bits() & 2) >> 1) as f32 +
                        0.125 * (self.mantissa_bits() & 1) as f32
                    )
                } else {
                    Some(
                        -(
                            1.0 +
                            0.5 * ((self.mantissa_bits() & 4) >> 2) as f32 +
                            0.25 * ((self.mantissa_bits() & 2) >> 1) as f32 +
                            0.125 * (self.mantissa_bits() & 1) as f32
                        )
                    )
                }
            }
        }
    }

    ///Gets the FP8 number as (exponent, signed mantissa)
    ///
    ///SAFETY: The number MUST NOT BE 0 or NaN
    pub(crate) unsafe fn as_components(&self) -> (i8, u8) {
        match State::get(self) {
            State::Normal => (
                self.exponent_value().unwrap(),
                self.mantissa_value().unwrap()
            ),

            State::Subnormal => (-6, self.mantissa_value().unwrap()),

            _ => unreachable!(),
        }
    }

    pub fn print_differs(
        percent_tolerance: f32,

        f_impl: fn(&Fp8, &Fp8) -> (Fp8, State),
        f_fpu: fn(f32, f32) -> f32
    ) {
        for i in 0b0000_0000..=0b1111_1111 {
            let a = Fp8::from(i);

            for j in 0b0000_0000..=0b1111_1111 {
                let b = Fp8::from(j);

                let (r, s) = f_impl(&a, &b);

                let fpu_r = f_fpu(Into::<f32>::into(a), Into::<f32>::into(b));
                let e = percent_tolerance * fpu_r / 100.0;

                let r_c: f32 = r.into();

                let in_range = f32::abs(fpu_r) - f32::abs(e) < f32::abs(r_c) &&
                    f32::abs(r_c) < f32::abs(fpu_r) + f32::abs(e);
                let equals = r_c == fpu_r;

                if !(in_range || equals) && s != State::NaN {
                    println!(
                        "{}, {}, {}, FPU: {}",
                        Into::<f32>::into(a),
                        Into::<f32>::into(b),
                        Into::<f32>::into(r),
                        f_fpu(Into::<f32>::into(a), Into::<f32>::into(b))
                    );
                }
            }
        }
    }

    pub fn print_as_csv(f: fn(&Fp8, &Fp8) -> (Fp8, State)) {
        for i in 0..=255 {
            for j in 0..=255 {
                println!(
                    "{:08b},{:08b},{:08b}",
                    i,
                    j,
                    Into::<u8>::into(f(&Fp8::from(i), &Fp8::from(j)).0)
                );
            }
        }
    }
}

impl Debug for Fp8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mantissa = self.mantissa_value_as_float();
        let exponent = self.exponent_value();

        return if let None = mantissa && None == exponent {
            if self.is_positive() {
                write!(f, "NaN")
            }
            else {
                write!(f, "-NaN")
            }
        } else if let Some(_) = mantissa && let None = exponent {
            if self.is_positive() {
                write!(f, "0.0")
            }
            else {
                write!(f, "-0.0")
            }
        } else if let None = mantissa && let Some(_) = exponent {
            unreachable!()
        } else {
            write!(
                f,
                "{} x 2 ^ {}",
                self.mantissa_value_as_float().unwrap(),
                self.exponent_value().unwrap()
            )
        };
    }
}

impl From<u8> for Fp8 {
    ///Converts a raw byte into an FP8 variable.
    fn from(value: u8) -> Self {
        Self { byte: value }
    }
}

impl Into<u8> for Fp8 {
    ///Returns the raw byte representation of the
    ///FP8 number.
    fn into(self) -> u8 {
        self.byte
    }
}

impl Into<f32> for Fp8 {
    ///Converts the FP8 variable into a regular float.
    ///
    ///NOTE: f32 only supports +NaN
    fn into(self) -> f32 {
        let exp = self.exponent_value();
        let mantissa = self.mantissa_value_as_float();

        return if exp == None && mantissa == None {
            f32::NAN
        } else if exp == None && mantissa != None {
            if self.is_positive() {
                0.0
            }
            else {
                -0.0
            }
        } else if exp != None && mantissa == None {
            unreachable!()
        } else {
            let exp = exp.unwrap();
            let mantissa = mantissa.unwrap();

            mantissa * f32::powi(2.0, exp as i32)
        };
    }
}