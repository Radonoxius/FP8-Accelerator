use std::ops;

use crate::{Fp8, state::State};

///Returns 1 / n.
pub fn reciprocal(n: &Fp8) -> Fp8 {
    let one = Fp8::one();
    let n_state = State::get(n);

    if n.byte == 0b0_0111_000 ||
        n.byte == 0b1_0111_000 {
        return one.xor_signed(&one, n);
    }

    //Ready to witness this Hell?
    if n_state == State::NaN {
        return *n;
    } else if n_state == State::Zero {
        return Fp8::nan().xor_signed(&one, n);
    } else if n_state == State::Subnormal {
        let result_sign = n.sign_bit();
        if n.mantissa_bits() == 1 {
            return Fp8::new(result_sign, 15, 6)
        }
        let recip_bits = 128 - (
            (n.mantissa_bits() << 2) -
            (n.mantissa_bits() >> 2)
        ) - if n.mantissa_bits() > 4 { 0 } else { 1 };

        Fp8 { byte: recip_bits | result_sign << 7 }
    } else {
        let result_sign = n.sign_bit();
        let recip_bits;

        if n.byte & 0b0111_1111 < 106 {
            recip_bits = 111 -
                (n.exponent_bits() << 3) -
                n.mantissa_bits();
        } else {
            recip_bits = 32 -
            (n.exponent_bits() << 1) -
            (n.mantissa_bits() >> 2);
        }

        Fp8 { byte: recip_bits | result_sign << 7 }
    }
}

///Returns a / b and the state of the result.
pub fn divide(a: &Fp8, b: &Fp8) -> (Fp8, State) {
    let a_state = State::get(a);
    let b_state = State::get(b);

    if a_state == State::NaN && b_state == State::NaN {
        return (Fp8::nan(), State::NaN);
    } else if a_state == State::NaN {
        return (a.xor_signed(a, b), State::NaN);
    } else if b_state == State::NaN {
        return (b.xor_signed(a, b), State::NaN);
    }
    
    else if a_state == State::Zero && b_state == State::Zero {
        return (Fp8::nan(), State::NaN);
    } else if a_state == State::Zero {
        return (a.xor_signed(a, b), State::Zero);
    } else if b_state == State::Zero {
        return (
            (&Fp8::nan()).xor_signed(a, b),
            State::NaN
        );
    }

    //Get (exponent, mantissa) for each operand.
    //Normal:    implicit leading 1, so mantissa = 0b1_xxx (value 8..15)
    //Subnormal: no implicit 1,      so mantissa = 0b0_xxx (value 1..7)
    //Both cases use effective exponent -6 at the boundary.
    let (a_exp, a_mant) = unsafe { a.as_components() };
    let (b_exp, b_mant) = unsafe { b.as_components() };

    let a_mant = (a_mant as u16) << 12;

    let result_sign = a.sign_bit() ^ b.sign_bit();
    let mut result_exp = a_exp - b_exp;

    let full_mant = a_mant / b_mant as u16;

    //Perform Rount-to-Nearest, Ties to Even
    let guard  = (full_mant >> 8) & 1;
    let sticky = (full_mant & 0b11000000) >> 6;
    let truncated = full_mant.unbounded_shr(9);

    let round_up = guard == 1 && (sticky != 0 || (truncated & 1) == 1);
    let mut abs_mant = truncated + if round_up { 1 } else { 0 };

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
        return (
            Fp8::new(result_sign, 0, mantissa_bits),
            State::Subnormal
        );
    }

    let exp_bits = (result_exp + 7) as u8;

    //Avoid accidentally encoding NaN (0_1111_111); saturate to max normal instead
    if exp_bits == 15 && mantissa_bits == 7 {
        return (
            Fp8::new(result_sign, 15, 6),
            State::Normal
        );
    }

    (Fp8::new(result_sign, exp_bits, mantissa_bits), State::Normal)
}

impl ops::Div for Fp8 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        divide(&self, &rhs).0
    }
}