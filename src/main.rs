use soft_fp8::{Fp8, addition::add};

fn main() {
    Fp8::print_differs(
        25.0,
        |a, b| add(a, b),
        |af, bf| af + bf
    );
}