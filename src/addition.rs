use crate::{Fp8, state::State};

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

    // Get (exponent, mantissa) for each operand.
    // Normal:    implicit leading 1, so mantissa = 0b1_xxx (value 8..15)
    // Subnormal: no implicit 1,      so mantissa = 0b0_xxx (value 1..7)
    // Both cases use effective exponent -6 at the boundary.
    let (a_exp, a_mant) = unsafe { a.as_components() };
    let (b_exp, b_mant) = unsafe { b.as_components() };

    // Align mantissas to the larger exponent
    let (mut result_exp, sum) = if a_exp >= b_exp {
        let shift = a_exp - b_exp;
        (a_exp, a_mant + b_mant.unbounded_shr(shift as u32))
    } else {
        let shift = b_exp - a_exp;
        (b_exp, a_mant.unbounded_shr(shift as u32) + b_mant)
    };

    if sum == 0 {
        return (Fp8::zero(), State::Zero);
    }

    let result_sign = if sum < 0 { 1u8 } else { 0u8 };
    let mut abs_mant = sum.unsigned_abs();

    // Shift right on carry overflow (e.g. 1.111 + 1.111 = 11.110)
    if abs_mant >= 16 {
        abs_mant >>= 1;
        result_exp += 1;
    }

    // Shift left to normalise, but stop at the subnormal boundary (exp = -6)
    while abs_mant < 8 && result_exp > -6 {
        abs_mant <<= 1;
        result_exp -= 1;
    }

    // Overflow: E4M3 has no infinity, saturate to largest finite value
    if result_exp > 8 {
        return (Fp8::nan(), State::NaN);
    }

    // Underflow
    if result_exp < -6 {
        return (Fp8::zero(), State::Zero);
    }

    let mantissa_bits = (abs_mant & 0b111) as u8;

    // Subnormal result (implicit bit is gone, exp field = 0)
    if result_exp == -6 && abs_mant < 8 {
        return (Fp8::new(result_sign, 0, mantissa_bits), State::Subnormal);
    }

    let exp_bits = (result_exp + 7) as u8;

    // Avoid accidentally encoding NaN (0_1111_111); saturate to max normal instead
    if exp_bits == 15 && mantissa_bits == 7 {
        return (Fp8::nan(), State::NaN);
    }

    (Fp8::new(result_sign, exp_bits, mantissa_bits), State::Normal)
}