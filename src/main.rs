use soft_fp8::{Fp8, addition::add};

fn main() {
    Fp8::dump_csv(
        |a, b| add(a, b)
    );
}