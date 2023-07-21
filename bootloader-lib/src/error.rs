use thiserror_no_std::Error;

use common_lib::error::CommonError;

pub type BootLoaderResult<T = ()> = Result<T, BootLoaderError>;


#[derive(Debug, Error)]
pub enum BootLoaderError {
    #[error(transparent)]
    Common(#[from] CommonError),
    
    #[error(transparent)]
    FailedFileOperation(#[from] uefi::Error),
}


