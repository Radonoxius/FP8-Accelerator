use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug)]
pub enum DriverError {
    MemFdError,
    MmapError
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
                )
        }
    }
}

impl Error for DriverError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}