use std::time::Instant;

use soft_fp8::{addition::add, subtraction::subtract};
use vfp8_driver::{U128, Vfp8Accelerator, Vfp8Operation};
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
                subtract(&ax[i][0].into(), &bx[i][0].into()).0.into(),
                subtract(&ax[i][1].into(), &bx[i][1].into()).0.into(),
                subtract(&ax[i][2].into(), &bx[i][2].into()).0.into(),
                subtract(&ax[i][3].into(), &bx[i][3].into()).0.into(),
                subtract(&ax[i][4].into(), &bx[i][4].into()).0.into(),
                subtract(&ax[i][5].into(), &bx[i][5].into()).0.into(),
                subtract(&ax[i][6].into(), &bx[i][6].into()).0.into(),
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
    println!("Soft Impl. took {} ms.", t2.as_millis());

    println!("\n{:?}\n", rx[200]);


    let mut device = Vfp8Accelerator::take().unwrap();

    let t1 = Instant::now();
    for i in 0..rx.capacity() {
        rx[i] = device.compute(
            Vfp8Operation::Subtract,
            [
                (ax[i][0].into(), bx[i][0].into()),
                (ax[i][1].into(), bx[i][1].into()),
                (ax[i][2].into(), bx[i][2].into()),
                (ax[i][3].into(), bx[i][3].into()),
                (ax[i][4].into(), bx[i][4].into()),
                (ax[i][5].into(), bx[i][5].into()),
                (ax[i][6].into(), bx[i][6].into())
            ]
        ).unwrap().unwrap()
    }
    let t2 = t1.elapsed();
    println!("Hard Impl. took {} ms.", t2.as_millis());

    println!("\n{:?}\n", rx[200]);
}