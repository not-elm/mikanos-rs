use alloc::string::String;
use core::fmt::Debug;

use anyhow::Error;

pub type PciResult<T = ()> = Result<T, PciError>;

#[macro_export]
#[doc(hidden)]
macro_rules! pci_error {
    ($($message: tt) *) => {
        $crate::error::PciError::from(anyhow::anyhow!($($message)*))
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! pci_bail {
    ($($message: tt) *) => {
        Err($crate::pci_error!($($message)*))
    };
}




#[derive(Debug)]
#[repr(transparent)]
pub struct PciError(anyhow::Error);


impl From<anyhow::Error> for PciError {
    fn from(e: Error) -> Self {
        Self(e)
    }
}


impl PciError {
    pub fn new(message: String) -> Self {
        Self(anyhow::anyhow!(message))
    }


    pub fn invalid_target_event() -> Self {
        pci_error!("Invalid target event")
    }
}


