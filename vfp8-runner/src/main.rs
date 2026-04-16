use std::time::Instant;

use soft_fp8::addition::add;
use vfp8_driver::{U128, Vfp8Accelerator};
use vfp8_runner::fill_randoms;

fn main() {
    let mut ax = Vec::<U128>::with_capacity(100000);
    fill_randoms(&mut ax);
    let mut bx = Vec::<U128>::with_capacity(100000);
    fill_randoms(&mut bx);

    let mut rx = Vec::<U128>::with_capacity(100000);

    let t1 = Instant::now();
    for i in 0..rx.capacity() {
        rx.push(
            [
                add(&ax[i][0].into(), &bx[i][0].into()).0.into(),
                add(&ax[i][1].into(), &bx[i][1].into()).0.into(),
                add(&ax[i][2].into(), &bx[i][2].into()).0.into(),
                add(&ax[i][3].into(), &bx[i][3].into()).0.into(),
                add(&ax[i][4].into(), &bx[i][4].into()).0.into(),
                add(&ax[i][5].into(), &bx[i][5].into()).0.into(),
                add(&ax[i][6].into(), &bx[i][6].into()).0.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into()
            ]
        );
    }
    let t2 = t1.elapsed();
    println!("Soft Impl. took {} micros.", t2.as_millis());


    let mut device = Vfp8Accelerator::take().unwrap();

    let mut dummy = [1; 16];
    dummy[0] = 5;

    let _ = device.write_reg_at(0, dummy);
    let store = device.read_reg_at(0);

    println!("Sent {:?}", dummy);
    println!("Got {:?}!", store.unwrap());
}