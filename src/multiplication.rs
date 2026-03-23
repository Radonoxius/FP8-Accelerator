use crate::{Fp8, state::State};

pub fn multiply(a: &Fp8, b: &Fp8) -> (Fp8, State) {
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
        return (*a, State::Zero);
    } else if b_state == State::Zero {
        return (*b, State::Zero);
    }

    //Get (exponent, mantissa) for each operand.
    //Normal:    implicit leading 1, so mantissa = 0b1_xxx (value 8..15)
    //Subnormal: no implicit 1,      so mantissa = 0b0_xxx (value 1..7)
    //Both cases use effective exponent -6 at the boundary.
    let (a_exp, a_mant) = unsafe { a.as_components() };
    let (b_exp, b_mant) = unsafe { b.as_components() };

    let a_mant = a_mant << 3;
    let b_mant = b_mant << 3;

    let result_sign = 0;

    todo!()
}