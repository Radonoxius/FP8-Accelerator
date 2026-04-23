use std::time::Instant;

use soft_fp8::{multiplication::multiply};
use vfp8_driver::{FpReg, Vfp8Accelerator, Vfp8Operator, ops::DoubleOperandExpr};
use vfp8_runner::fill_randoms;

fn main() {
    let mut ax = Vec::<FpReg>::with_capacity(100000);
    fill_randoms(&mut ax);
    let mut bx = Vec::<FpReg>::with_capacity(100000);
    fill_randoms(&mut bx);

    let mut rx = Vec::<FpReg>::with_capacity(100000);

    let t1 = Instant::now();
    for i in 0..rx.capacity() {
        rx.push(
            [
                multiply(&ax[i][0].into(), &bx[i][0].into()).0.into(),
                multiply(&ax[i][1].into(), &bx[i][1].into()).0.into(),
                multiply(&ax[i][2].into(), &bx[i][2].into()).0.into(),
                multiply(&ax[i][3].into(), &bx[i][3].into()).0.into(),
                multiply(&ax[i][4].into(), &bx[i][4].into()).0.into(),
                multiply(&ax[i][5].into(), &bx[i][5].into()).0.into(),
                multiply(&ax[i][6].into(), &bx[i][6].into()).0.into(),
                multiply(&ax[i][7].into(), &bx[i][7].into()).0.into(),
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

    println!("\n{:?}", rx[0]);
    println!("{:?}", rx[1]);
    println!("{:?}\n", rx[2]);


    let mut device = Vfp8Accelerator::take().unwrap();

    let t1 = Instant::now();
    for i in 0..rx.capacity() {
        rx[i] = device.compute(
            &DoubleOperandExpr::construct(
                Vfp8Operator::Subtract,
                ax[i][0].into(), bx[i][0].into(),
                ax[i][1].into(), bx[i][1].into(),
                ax[i][2].into(), bx[i][2].into(),
                ax[i][3].into(), bx[i][3].into(),
                ax[i][4].into(), bx[i][4].into(),
                ax[i][5].into(), bx[i][5].into(),
                ax[i][6].into(), bx[i][6].into(),
                ax[i][7].into(), bx[i][7].into()
            ).unwrap()
        ).unwrap()
    }
    let t2 = t1.elapsed();
    println!("Hard Impl. took {} ms.", t2.as_millis());

    println!("\n{:?}", rx[0]);
    println!("{:?}", rx[1]);
    println!("{:?}\n", rx[2]);
}