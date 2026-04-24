use soft_fp8::Fp8;

use crate::{OPCODE_REGISTER, RESULT_REGISTER, Vfp8Accelerator, Vfp8Operator, ffi::ComputeResult, ops::OPCODE_CACHE};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compute_inverse(
    device: *mut Vfp8Accelerator,
    
    a0: Fp8,
    a1: Fp8,
    a2: Fp8,
    a3: Fp8,
    a4: Fp8,
    a5: Fp8,
    a6: Fp8,
    a7: Fp8,
    a8: Fp8,
    a9: Fp8,
    a10: Fp8,
    a11: Fp8,
    a12: Fp8,
    a13: Fp8,
    a14: Fp8,
    a15: Fp8
) -> ComputeResult {
    unsafe {
        if OPCODE_CACHE != Vfp8Operator::Inverse {
            OPCODE_CACHE = Vfp8Operator::Inverse;

            let mut opcr = [0; 16];
            opcr[15] = Vfp8Operator::Inverse.into();

            device
                .as_mut()
                .unwrap()
                .write_to(OPCODE_REGISTER, opcr);
        }

        let oprr = [
            a0.into(), a1.into(), a2.into(), a3.into(),
            a4.into(), a5.into(), a6.into(), a7.into(),
            a8.into(), a9.into(), a10.into(), a11.into(),
            a12.into(), a13.into(), a14.into(), a15.into()
        ];

        device
            .as_mut()
            .unwrap()
            .write_to(OPCODE_REGISTER, oprr);

        let res = device
            .as_ref()
            .unwrap()
            .read_from(RESULT_REGISTER);

        ComputeResult {
            res0: res[0].into(), res1: res[1].into(),
            res2: res[2].into(), res3: res[3].into(),
            res4: res[4].into(), res5: res[5].into(),
            res6: res[6].into(), res7: res[7].into(),
            res8: res[8].into(), res9: res[9].into(),
            res10: res[10].into(), res11: res[11].into(),
            res12: res[12].into(), res13: res[13].into(),
            res14: res[14].into(), res15: res[15].into()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn compute(
    device: *mut Vfp8Accelerator,
    operator: Vfp8Operator,

    a0: Fp8, b0: Fp8,
    a1: Fp8, b1: Fp8,
    a2: Fp8, b2: Fp8,
    a3: Fp8, b3: Fp8,
    a4: Fp8, b4: Fp8,
    a5: Fp8, b5: Fp8,
    a6: Fp8, b6: Fp8,
    a7: Fp8, b7: Fp8
) -> ComputeResult {
    unsafe {
        if OPCODE_CACHE != operator {
            OPCODE_CACHE = operator;

            let mut opcr = [0; 16];
            opcr[15] = operator.into();

            device
                .as_mut()
                .unwrap()
                .write_to(OPCODE_REGISTER, opcr);
        }

        let oprr = [
            b0.into(), a0.into(),
            b1.into(), a1.into(),
            b2.into(), a2.into(),
            b3.into(), a3.into(),
            b4.into(), a4.into(),
            b5.into(), a5.into(),
            b6.into(), a6.into(),
            b7.into(), a7.into()
        ];

        device
            .as_mut()
            .unwrap()
            .write_to(OPCODE_REGISTER, oprr);

        let res = device
            .as_ref()
            .unwrap()
            .read_from(RESULT_REGISTER);

        ComputeResult {
            res0: res[0].into(), res1: res[1].into(),
            res2: res[2].into(), res3: res[3].into(),
            res4: res[4].into(), res5: res[5].into(),
            res6: res[6].into(), res7: res[7].into(),
            res8: 0.into(), res9: 0.into(),
            res10: 0.into(), res11: 0.into(),
            res12: 0.into(), res13: 0.into(),
            res14: 0.into(), res15: 0.into()
        }
    }
}