use std::f32;

use soft_fp8::{Fp8, addition::add, state::State};

fn main() {
    for i in 0b0000_0000..=0b1111_1111 {
        let a = Fp8::from(i);

        for j in 0b0000_0000..=0b1111_1111 {
            let b = Fp8::from(j);

            let (r, s) = add(&a, &b);

            if Into::<f32>::into(r) != (
                Into::<f32>::into(a) + Into::<f32>::into(b)
            ) && s != State::NaN {
                println!(
                    "Differed at: {} + {} = {}, But FP unit says: {}",
                    Into::<f32>::into(a),
                    Into::<f32>::into(b),
                    Into::<f32>::into(r),
                    Into::<f32>::into(a) + Into::<f32>::into(b)
                );
            }
        }
    }
}