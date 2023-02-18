
use crate::error::Error;

pub type Result<T = ()> = core::result::Result<T, Error>;




impl From<uefi::Error<usize>> for Error{
    fn from(value: uefi::Error<usize>) -> Self {
        Self::SfsFileWrite(*value.data())
    }
}

impl From<uefi::Error<()>> for Error{
    fn from(_value: uefi::Error<()>) -> Self {
        Self::Void
    }
}


pub fn from_sfs_write_result<T>(result: uefi::Result<T, usize>) -> crate::result::Result<T>{
    match result {
        Ok(data) => Ok(data),
        Err(uefi_error) => Err(Error::SfsFileWrite(*uefi_error.data()))
    }
}