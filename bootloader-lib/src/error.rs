use thiserror_no_std::Error;

use common_lib::error::CommonError;
use uefi::fs;
use uefi::fs::Error;

pub type BootLoaderResult<T = ()> = Result<T, BootLoaderError>;


#[derive(Debug, Error)]
pub enum BootLoaderError {
    Common(CommonError),
    FailedFileOperation(fs::Error),
}


impl From<fs::Error> for BootLoaderError {
    fn from(value: Error) -> Self {
        Self::FailedFileOperation(value)
    }
}


impl From<CommonError> for BootLoaderError {
    fn from(value: CommonError) -> Self {
        Self::Common(value)
    }
}
