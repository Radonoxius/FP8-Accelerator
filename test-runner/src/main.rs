use std::time::Instant;

use soft_fp8::{Fp8, division::inverse_divide};
use vfp8_driver::{Vfp8Accelerator, Vfp8Operator, ops::SingleOperandExpr};
use test_runner::{fill_randoms, generate_randoms};

fn main() {
    println!("Generating pseudorandom Fp8 values...");
    let mut ax = Vec::<[Fp8; 16]>::with_capacity(100000);
    fill_randoms(&mut ax);
    let mut bx = Vec::<[Fp8; 16]>::with_capacity(100000);
    fill_randoms(&mut bx);
    println!("Finished generation\n");

    let mut rx = Vec::<[Fp8; 16]>::with_capacity(100000);

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

    println!("\nChoosing 3 random test indices...\n");
    let random_indices = generate_randoms();
    let i1 = unsafe { *((&raw const random_indices as usize + 0) as *const u16) } as usize;
    let i2 = unsafe { *((&raw const random_indices as usize + 2) as *const u16) } as usize;
    let i3 = unsafe { *((&raw const random_indices as usize + 4) as *const u16) } as usize;

    println!("\n{:?}", rx[i1]);
    println!("{:?}", rx[i2]);
    println!("{:?}\n", rx[i3]);


    let mut device = Vfp8Accelerator::take().unwrap();

    let t1 = Instant::now();
    for i in 0..rx.capacity() {
        rx[i] = device.compute(
            SingleOperandExpr::construct(
                Vfp8Operator::Inverse,
                ax[i][0], ax[i][1],
                ax[i][2], ax[i][3],
                ax[i][4], ax[i][5],
                ax[i][6], ax[i][7],
                ax[i][8], ax[i][9],
                ax[i][10], ax[i][11],
                ax[i][12], ax[i][13],
                ax[i][14], ax[i][15],
            ).unwrap()
        ).unwrap()
    }
    let t2 = t1.elapsed();
    println!("Hard Impl. took {} ms.", t2.as_millis());

    println!("\n{:?}", rx[i1]);
    println!("{:?}", rx[i2]);
    println!("{:?}\n", rx[i3]);
}