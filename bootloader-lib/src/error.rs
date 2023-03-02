use core::fmt::{Display, Formatter};

pub type LibResult<T = ()> = Result<T, LibError>;

#[derive(Debug)]
pub enum LibError {
    FailedToAllocatePages(u64)
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct LibError<Data: core::fmt::Debug = ()> {
//     data: Data,
// }
//
// impl<Data: core::fmt::Debug + core::fmt::Display> core::fmt::Display for LibError<Data> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "LibsError={}", self.data)
//     }
// }
//
impl Display for LibError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            LibError::FailedToAllocatePages(phys_addr) => write!(f, "FailedToAllocatePages PhysicalAddr={}", phys_addr)
        }
    }
}

impl core::error::Error for LibError {}
//
// #[cfg(feature = "unstable")]
// impl<Data: Debug + Display> core::error::Error for LibError<Data> {}
