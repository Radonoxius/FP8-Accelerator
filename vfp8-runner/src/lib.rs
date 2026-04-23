use std::{fs::OpenOptions, io::Read};

use vfp8_driver::FpReg;

pub fn generate_randoms() -> FpReg {
    let mut r = [0; 16];

    let mut urandom_file = OpenOptions::new().read(true).open("/dev/urandom").unwrap();
    urandom_file.read_exact(&mut r).unwrap();

    r
}

pub fn fill_randoms(vec: &mut Vec<FpReg>) {
    for _ in 0..vec.capacity() {
        vec.push(generate_randoms());
    }
}