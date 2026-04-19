use soft_fp8::{Fp8, multiplication::fma};

fn main() {
    Fp8::print_as_csv3(
        fma
    );
}