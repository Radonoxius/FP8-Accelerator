use soft_fp8::{Fp8, multiplication::multiply};

fn main() {
    Fp8::print_as_csv(
        multiply
    );
}