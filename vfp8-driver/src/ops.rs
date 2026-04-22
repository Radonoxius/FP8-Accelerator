use crate::{OperandPair, U128, Vfp8Accelerator, Vfp8Operation, errors::DriverError};

impl Vfp8Accelerator {
    pub fn compute2(&mut self, op: Vfp8Operation, operands: [OperandPair; 8]) -> Option<Result<U128, DriverError>> {
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
                self.write_reg_at(0x10, opcode).unwrap();
                self.write_reg_at(0, data).unwrap();

                Some(self.read_reg_at(0x20))
            }
        }
    }
}