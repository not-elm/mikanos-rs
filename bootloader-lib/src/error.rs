use core::fmt::{Display, Formatter};
use uefi::fs;
use uefi::fs::Error;

pub type LibResult<T = ()> = Result<T, BootLoaderError>;

#[derive(Debug)]
pub enum BootLoaderError {
    FailedToAllocatePages(u64),
    FailedFileOperation(fs::Error),
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct LibError<Data: core::fmt::Debug = ()> {
//     data: Data,
// }
//
// impl<Data: core::fmt::Debug + core::fmt::Display> core::fmt::Display for
// LibError<Data> {     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) ->
// core::fmt::Result {         write!(f, "LibsError={}", self.data)
//     }
// }
//
impl Display for BootLoaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            BootLoaderError::FailedToAllocatePages(phys_addr) => {
                write!(f, "FailedToAllocatePages PhysicalAddr={}", phys_addr)
            }
            BootLoaderError::FailedFileOperation(file) => {
                write!(f, "Failed file operation {}", file)
            }
        }
    }
}

impl core::error::Error for BootLoaderError {}
//
// #[cfg(feature = "unstable")]
// impl<Data: Debug + Display> core::error::Error for LibError<Data> {}


impl From<fs::Error> for BootLoaderError {
    fn from(value: Error) -> Self {
        Self::FailedFileOperation(value)
    }
}
