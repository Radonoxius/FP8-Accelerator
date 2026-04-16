use std::{fs::OpenOptions, io::Read};

use vfp8_driver::U128;

pub fn generate_randoms() -> U128 {
    let mut r = [0; 16];

    let mut urandom_file = OpenOptions::new().read(true).open("/dev/urandom").unwrap();
    urandom_file.read_exact(&mut r).unwrap();

    r
}

pub fn fill_randoms(vec: &mut Vec<U128>) {
    for _ in 0..vec.capacity() {
        vec.push(generate_randoms());
    }
}