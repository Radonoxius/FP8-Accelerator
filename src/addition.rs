use crate::{Fp8, state::State};

pub fn add(a: &Fp8, b: &Fp8) -> (Fp8, State) {
    let a_state = State::get(a);
    let b_state = State::get(b);

    if a_state == State::NaN && b_state == State::NaN {
        return (Fp8::nan(), State::NaN);
    }
    else if a_state == State::NaN {
        return (*a, State::NaN);
    }
    else if b_state == State::NaN {
        return (*b, State::NaN);
    }

    else if a_state == State::Zero && b_state == State::Zero {
        return (Fp8::zero(), State::Zero);
    }
    else if a_state == State::Zero {
        return (*b, b_state);
    }
    else if b_state == State::Zero {
        return (*a, a_state);
    }

    else if a_state == State::Normal && b_state == State::Normal {
        todo!()
    }
    else {
        todo!()
    }
}