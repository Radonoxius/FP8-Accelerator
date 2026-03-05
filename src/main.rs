use soft_fp8::Fp8;

fn main() {
    let n = Fp8::new(0b0_1111_000);

    println!("{:?}", n);
}
