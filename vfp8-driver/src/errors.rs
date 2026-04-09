use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub enum DriverError {
    MemFdError,
    MmapError,

    OutOfBounds
}

impl Display for DriverError {
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

            Self::OutOfBounds =>
                f.write_str(
                    "The memory offset for read/write is out of bounds! 0x1000 is the maximum offset!"
                )
        }
    }
}

impl Error for DriverError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}