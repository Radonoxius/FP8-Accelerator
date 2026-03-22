use soft_fp8::{Fp8, subtraction::subtract};

fn main() {
    Fp8::print_as_csv(
        subtract
    );
}