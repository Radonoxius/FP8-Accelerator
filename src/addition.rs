use std::ops;

use crate::{Fp8, state::State};

///Returns a + b and the state of the result.
pub fn add(a: &Fp8, b: &Fp8) -> (Fp8, State) {
    let a_state = State::get(a);
    let b_state = State::get(b);

    if a_state == State::NaN && b_state == State::NaN {
        return (Fp8::nan(), State::NaN);
    } else if a_state == State::NaN {
        return (*a, State::NaN);
    } else if b_state == State::NaN {
        return (*b, State::NaN);
    }
    
    else if a_state == State::Zero && b_state == State::Zero {
        return (Fp8::zero(), State::Zero);
    } else if a_state == State::Zero {
        return (*b, b_state);
    } else if b_state == State::Zero {
        return (*a, a_state);
    }

    //Get (exponent, mantissa) for each operand.
    //Normal:    implicit leading 1, so mantissa = 0b1_xxx (value 8..15)
    //Subnormal: no implicit 1,      so mantissa = 0b0_xxx (value 1..7)
    //Both cases use effective exponent -6 at the boundary.
    let (a_exp, a_mant) = unsafe { a.as_components() };
    let (b_exp, b_mant) = unsafe { b.as_components() };

    let a_mant = a_mant << 3;
    let b_mant = b_mant << 3;

    let result_sign;

    //Fixed Point Add logic
    let (mut result_exp, sum) = if a_exp > b_exp {
        let shift = a_exp - b_exp;

        if a.is_positive() && b.is_positive() {
            result_sign = 0;
            (a_exp, a_mant + b_mant.unbounded_shr(shift as u32))
        } else if a.is_positive() && !b.is_positive() {
            result_sign = 0;
            (a_exp, a_mant - b_mant.unbounded_shr(shift as u32))
        } else if !a.is_positive() && b.is_positive() {
            result_sign = 1;
            (a_exp, a_mant - b_mant.unbounded_shr(shift as u32))
        } else {
            result_sign = 1;
            (a_exp, a_mant + b_mant.unbounded_shr(shift as u32))
        }
    } else if a_exp == b_exp {
        if a_mant >= b_mant {
            if a.is_positive() && b.is_positive() {
                result_sign = 0;
                (a_exp, a_mant + b_mant)
            } else if a.is_positive() && !b.is_positive() {
                result_sign = 0;
                (a_exp, a_mant - b_mant)
            } else if !a.is_positive() && b.is_positive() {
                result_sign = 1;
                (a_exp, a_mant - b_mant)
            } else {
                result_sign = 1;
                (a_exp, a_mant + b_mant)
            }
        } else {
            if b.is_positive() && a.is_positive() {
                result_sign = 0;
                (a_exp, a_mant + b_mant)
            } else if b.is_positive() && !a.is_positive() {
                result_sign = 0;
                (a_exp, b_mant - a_mant)
            } else if !b.is_positive() && a.is_positive() {
                result_sign = 1;
                (a_exp, b_mant - a_mant)
            } else {
                result_sign = 1;
                (a_exp, a_mant + b_mant)
            }
        }
    } else {
        let shift = b_exp - a_exp;

        if b.is_positive() && a.is_positive() {
            result_sign = 0;
            (b_exp, b_mant + a_mant.unbounded_shr(shift as u32))
        } else if b.is_positive() && !a.is_positive() {
            result_sign = 0;
            (b_exp, b_mant - a_mant.unbounded_shr(shift as u32))
        } else if !b.is_positive() && a.is_positive() {
            result_sign = 1;
            (b_exp, b_mant - a_mant.unbounded_shr(shift as u32))
        } else {
            result_sign = 1;
            (b_exp, b_mant + a_mant.unbounded_shr(shift as u32))
        }
    };

    //Perform Rount-to-Nearest, Ties to Even
    let guard  = (sum >> 2) & 1;
    let sticky = sum & 0b11;
    let truncated = sum >> 3;

    let round_up = guard == 1 && (sticky != 0 || (truncated & 1) == 1);
    let mut abs_mant = truncated + if round_up { 1 } else { 0 };

    if sum == 0 {
        return (Fp8::zero(), State::Zero);
    }

    //Renormalise results
    if abs_mant >= 16 {
        abs_mant >>= 1;
        result_exp += 1;
    }
    while abs_mant < 8 && result_exp > -6 {
        abs_mant <<= 1;
        result_exp -= 1;
    }

    //Return NaN on overflow
    if result_exp > 8 {
        return (Fp8::nan(), State::NaN);
    }
    //Return 0 on underflow
    if result_exp < -6 {
        return (Fp8::zero(), State::Zero);
    }

    let mantissa_bits = (abs_mant & 0b111) as u8;

    //Subnormal result (implicit bit is gone, exp field = 0)
    if result_exp == -6 && abs_mant < 8 {
        return (Fp8::new(result_sign, 0, mantissa_bits), State::Subnormal);
    }

    let exp_bits = (result_exp + 7) as u8;

    //Avoid accidentally encoding NaN (0_1111_111); saturate to max normal instead
    if exp_bits == 15 && mantissa_bits == 7 {
        return (
            Fp8::new(result_sign, 15, 7),
            State::Normal
        );
    }

    (Fp8::new(result_sign, exp_bits, mantissa_bits), State::Normal)
}

impl ops::Add for Fp8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        add(&self, &rhs).0
    }
}