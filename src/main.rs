use soft_fp8::Fp8;

fn main() {
    for i in 0b1000_0000..=0b1111_1111 {
        let mut n = Fp8::from(i);
        n.flip_sign();

        println!("{:?}", n);
    }
}
