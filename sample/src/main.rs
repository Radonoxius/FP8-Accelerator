use soft_fp8::Fp8;
use vfp8_driver::{Vfp8Accelerator, Vfp8Operator, ops::DoubleOperandExpr};

fn main() {
    //Initialize array A as an array of 1.0s
    let ax = [Fp8::one(); 8];
    //Initialize array B as an array of 1.0s
    let bx = [Fp8::one(); 8];

    println!("Adding 1.0 + 1.0 on the VFP8 accelerator!");

    //Take ownership/Initialize the VFP8 accelerator.
    let mut vfp8 = Vfp8Accelerator::take().unwrap();

    //Dispatch an addition operation to the accelerator and
    //get back the results.
    let results = vfp8.compute(
        //Construct and mathematical expression with
        //two operands, a and b.
        DoubleOperandExpr::construct(
            Vfp8Operator::Add, 
            ax[0], bx[0], 
            ax[1], bx[1], 
            ax[2], bx[2], 
            ax[3], bx[3], 
            ax[4], bx[4], 
            ax[5], bx[5], 
            ax[6], bx[6], 
            ax[7], bx[7]
        ).unwrap()
    ).unwrap();

    println!("Results:");
    pretty_print(results);
}

//Who doesnt love pretty printing?
fn pretty_print(arr: [Fp8; 16]) {
    print!("[");
    for i in 0..15 {
        print!(
            "{}, ",
            Into::<f32>::into(arr[i])
        )
    }

    println!(
        "{}]",
        Into::<f32>::into(arr[15])
    );
}