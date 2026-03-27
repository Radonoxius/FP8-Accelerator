use soft_fp8::{Fp8, division::divide};

fn main() {
    Fp8::print_as_csv(
        divide
    );
}