use std::{fs::OpenOptions, io::Read};

use soft_fp8::Fp8;

pub fn generate_randoms() -> [Fp8; 16] {
    let mut r = [0; 16];

    let mut urandom_file = OpenOptions::new().read(true).open("/dev/urandom").unwrap();
    urandom_file.read_exact(&mut r).unwrap();

    r.map(|e| Into::<Fp8>::into(e))
}

pub fn fill_randoms(vec: &mut Vec<[Fp8; 16]>) {
    for _ in 0..vec.capacity() {
        vec.push(generate_randoms());
    }
}