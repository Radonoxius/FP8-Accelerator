use crate::{OPCODE_REGISTER_OFFSET, OPERAND_REGISTER_OFFSET, OperandPair, RESULT_REGISTER_OFFSET, U128, Vfp8Accelerator, Vfp8Operation, errors::DriverError};

static mut OPCODE_CACHE: Vfp8Operation = Vfp8Operation::Halt; 

impl Vfp8Accelerator {
    pub fn compute2(&mut self, op: Vfp8Operation, operands: [OperandPair; 8]) -> Option<Result<U128, DriverError>> {
        unsafe {
            let data = [
                operands[0].1.into(),
                operands[0].0.into(),

                operands[1].1.into(),
                operands[1].0.into(),

                operands[2].1.into(),
                operands[2].0.into(),

                operands[3].1.into(),
                operands[3].0.into(),

                operands[4].1.into(),
                operands[4].0.into(),

                operands[5].1.into(),
                operands[5].0.into(),

                operands[6].1.into(),
                operands[6].0.into(),

                operands[7].1.into(),
                operands[7].0.into()
            ];

            let mut opcode = [0; 16];
            opcode[15] = op.into();

            return match op {
                Vfp8Operation::Halt => None,

                _ => {
                    if op != OPCODE_CACHE {
                        OPCODE_CACHE = op;
                        self.write_reg_at(OPCODE_REGISTER_OFFSET, opcode).unwrap();
                    }
                    
                    self.write_reg_at(OPERAND_REGISTER_OFFSET, data).unwrap();
                    Some(self.read_reg_at(RESULT_REGISTER_OFFSET))
                }
            }
        }
    }
}