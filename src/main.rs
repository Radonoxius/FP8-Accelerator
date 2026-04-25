use soft_fp8::{Fp8, division::{divide, inverse_divide}};

fn main() {
    Fp8::print_as_csv1(
        inverse_divide
    );
}