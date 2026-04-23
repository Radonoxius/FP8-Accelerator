use std::{error::Error, fmt::{Debug, Display}};

use crate::Vfp8Operator;

///Represents a driver error that can occur during vfp8 accelerator operations.
#[derive(Debug)]
pub enum DriverError<'a> {
    MemFdError,
    MmapError,

    InvalidExpressionOperator(Vfp8Operator, &'a str)
}

impl<'a> Display for DriverError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::MemFdError =>
                f.write_str(
                    "An error occured while opening /dev/mem as read-write. Try running the program as root."
                ),

            Self::MmapError =>
                f.write_str(
                    "An error occured while obtaining virtual address of the bus. Try running the program as root."
                ),

            Self::InvalidExpressionOperator(op, s) =>
                write!(f, "{:?} is incompatible with {} expression!", op, s)
        }
    }
}

impl<'a> Error for DriverError<'a> {
    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}