use soft_fp8::{Fp8, subtraction::subtract};

fn main() {
    Fp8::print_differs(
        25.0,
        |a, b| subtract(a, b),
        |af, bf| af - bf
    );
}