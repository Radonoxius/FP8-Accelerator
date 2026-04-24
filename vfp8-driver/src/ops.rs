use soft_fp8::Fp8;

use crate::{OPCODE_REGISTER, OPERAND_REGISTER, RESULT_REGISTER, FpReg, Vfp8Accelerator, Vfp8Operator, errors::DriverError};

static mut OPCODE_CACHE: Vfp8Operator = Vfp8Operator::Idle;

///Represents a floating point arithmetic expression that can be computed by the accelerator.
///
///SAFETY: The implementor must ensure that the operator and operands provided are compatible.
///The operands must be properly ordered, in accordance with the accelerator to prevent UB
unsafe trait Vfp8Expression {
    ///Gets the operator associated with the given expression
    fn operator(&self) -> Vfp8Operator;

    ///Converts the given expression into a register pair
    fn as_raw(&self) -> [FpReg; 2];
}

///Represents a single-operand expression
pub struct SingleOperandExpr {
    ax: FpReg,
    operator: Vfp8Operator
}

impl SingleOperandExpr {
    ///Constructs a single-operand expression.
    ///Possible choices for `operator` depends on accelerator support
    pub fn construct<'a>(
        operator: Vfp8Operator,
        a0: Fp8, a1: Fp8, a2: Fp8, a3: Fp8,
        a4: Fp8, a5: Fp8, a6: Fp8, a7: Fp8,
        a8: Fp8, a9: Fp8, a10: Fp8, a11: Fp8,
        a12: Fp8, a13: Fp8, a14: Fp8, a15: Fp8
    ) -> Result<Self, DriverError<'a>> {
        match operator {
            Vfp8Operator::Inverse => Ok(
                Self {
                    ax: [
                        a0.into(), a1.into(), a2.into(), a3.into(),
                        a4.into(), a5.into(), a6.into(), a7.into(),
                        a8.into(), a9.into(), a10.into(), a11.into(),
                        a12.into(), a13.into(), a14.into(), a15.into()
                    ],
                    operator 
                }
            ),

            Vfp8Operator::Idle => Ok(
                Self {
                    ax: [0; 16],
                    operator
                }
            ),

            _ => Err(
                DriverError::InvalidExpressionOperator(
                    operator,
                    "SingleOperandExpr"
                )
            )
        }
    }
}

unsafe impl Vfp8Expression for SingleOperandExpr {
    fn operator(&self) -> Vfp8Operator {
        self.operator
    }

    fn as_raw(&self) -> [FpReg; 2] {
        let mut opcode_r = [0; 16];
        opcode_r[15] = self.operator.into();

        [
            self.ax,
            opcode_r
        ]
    }
}

///Represents a two-operand expression, in the form `a op b`
pub struct DoubleOperandExpr {
    abx: FpReg,
    operator: Vfp8Operator
}

impl DoubleOperandExpr {
    ///Constructs a two-operand expression.
    ///Possible choices for `operator` depends on accelerator support
    pub fn construct<'a>(
        operator: Vfp8Operator,
        a0: Fp8, b0: Fp8,
        a1: Fp8, b1: Fp8,
        a2: Fp8, b2: Fp8,
        a3: Fp8, b3: Fp8,
        a4: Fp8, b4: Fp8,
        a5: Fp8, b5: Fp8,
        a6: Fp8, b6: Fp8,
        a7: Fp8, b7: Fp8
    ) -> Result<Self, DriverError<'a>> {
        match operator {
            Vfp8Operator::Add |
            Vfp8Operator::Subtract |
            Vfp8Operator::Multiply |
            Vfp8Operator::Divide => Ok(
                Self {
                    abx: [
                        b0.into(), a0.into(),
                        b1.into(), a1.into(),
                        b2.into(), a2.into(),
                        b3.into(), a3.into(),
                        b4.into(), a4.into(),
                        b5.into(), a5.into(),
                        b6.into(), a6.into(),
                        b7.into(), a7.into()
                    ],
                    operator
                }
            ),

            Vfp8Operator::Idle => Ok(
                Self {
                    abx: [0; 16],
                    operator
                }
            ),

            _ => Err(
                DriverError::InvalidExpressionOperator(
                    operator,
                    "DoubleOperandExpr"
                )
            )
        }
    }
}

unsafe impl Vfp8Expression for DoubleOperandExpr {
    fn operator(&self) -> Vfp8Operator {
        self.operator
    }

    fn as_raw(&self) -> [FpReg; 2] {
        let mut opcode_r = [0; 16];
        opcode_r[15] = self.operator.into();

        [
            self.abx,
            opcode_r
        ]
    }
}

///Represents a three-operand expression, in the form `(a opx b) opy c`
pub struct TripleOperandExpr {
    abcx: FpReg,
    operator: Vfp8Operator
}

impl TripleOperandExpr {
    ///Constructs a three-operand expression.
    ///Possible choices for `operator` depends on accelerator support
    pub fn construct<'a>(
        operator: Vfp8Operator,
        a0: Fp8, b0: Fp8, c0: Fp8,
        a1: Fp8, b1: Fp8, c1: Fp8,
        a2: Fp8, b2: Fp8, c2: Fp8,
        a3: Fp8, b3: Fp8, c3: Fp8,
        a4: Fp8, b4: Fp8, c4: Fp8
    ) -> Result<Self, DriverError<'a>> {
        match operator {
            Vfp8Operator::Fma => Ok(
                Self {
                    abcx: [
                        c0.into(), b0.into(), a0.into(),
                        c1.into(), b1.into(), a1.into(),
                        c2.into(), b2.into(), a2.into(),
                        c3.into(), b3.into(), a3.into(),
                        c4.into(), b4.into(), a4.into(),
                        0
                    ],
                    operator
                }
            ),

            Vfp8Operator::Idle => Ok(
                Self {
                    abcx: [0; 16],
                    operator
                }
            ),

            _ => Err(
                DriverError::InvalidExpressionOperator(
                    operator,
                    "TripleOperandExpr"
                )
            )
        }
    }
}

unsafe impl Vfp8Expression for TripleOperandExpr {
    fn operator(&self) -> Vfp8Operator {
        self.operator
    }

    fn as_raw(&self) -> [FpReg; 2] {
        let mut opcode_r = [0; 16];
        opcode_r[15] = self.operator.into();

        [
            self.abcx,
            opcode_r
        ]
    }
}

impl Vfp8Accelerator {
    ///Dispatches the expression to the vfp8 accelerator, evaluates it and returns the result.
    #[allow(private_bounds)]
    pub fn compute(&mut self, expr: impl Vfp8Expression) -> Option<FpReg> {
        unsafe {
            let reg_pair = expr.as_raw();

            return match expr.operator() {
                Vfp8Operator::Idle => None,

                _ => {
                    if expr.operator() != OPCODE_CACHE {
                        OPCODE_CACHE = expr.operator();
                        self.write_to(OPCODE_REGISTER, reg_pair[1]);
                    }
                    
                    self.write_to(OPERAND_REGISTER, reg_pair[0]);
                    Some(self.read_from(RESULT_REGISTER))
                }
            }
        }
    }
}