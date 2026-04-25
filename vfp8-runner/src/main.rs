use std::time::Instant;

use soft_fp8::division::inverse_divide;
use vfp8_driver::{FpReg, Vfp8Accelerator, Vfp8Operator, ops::SingleOperandExpr};
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
                inverse_divide(&ax[i][0].into()).0.into(),
                inverse_divide(&ax[i][1].into()).0.into(),
                inverse_divide(&ax[i][2].into()).0.into(),
                inverse_divide(&ax[i][3].into()).0.into(),
                inverse_divide(&ax[i][4].into()).0.into(),
                inverse_divide(&ax[i][5].into()).0.into(),
                inverse_divide(&ax[i][6].into()).0.into(),
                inverse_divide(&ax[i][7].into()).0.into(),
                inverse_divide(&ax[i][8].into()).0.into(),
                inverse_divide(&ax[i][9].into()).0.into(),
                inverse_divide(&ax[i][10].into()).0.into(),
                inverse_divide(&ax[i][11].into()).0.into(),
                inverse_divide(&ax[i][12].into()).0.into(),
                inverse_divide(&ax[i][13].into()).0.into(),
                inverse_divide(&ax[i][14].into()).0.into(),
                inverse_divide(&ax[i][15].into()).0.into(),
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
            SingleOperandExpr::construct(
                Vfp8Operator::Inverse,
                ax[i][0].into(), ax[i][1].into(),
                ax[i][2].into(), ax[i][3].into(),
                ax[i][4].into(), ax[i][5].into(),
                ax[i][6].into(), ax[i][7].into(),
                ax[i][8].into(), ax[i][9].into(),
                ax[i][10].into(), ax[i][11].into(),
                ax[i][12].into(), ax[i][13].into(),
                ax[i][14].into(), ax[i][15].into(),
            ).unwrap()
        ).unwrap()
    }
    let t2 = t1.elapsed();
    println!("Hard Impl. took {} ms.", t2.as_millis());

    println!("\n{:?}", rx[0]);
    println!("{:?}", rx[1]);
    println!("{:?}\n", rx[2]);
}