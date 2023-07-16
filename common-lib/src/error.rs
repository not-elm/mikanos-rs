use thiserror_no_std::Error;

pub type CommonResult<T = ()> = Result<T, CommonError>;


#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Failed allocate pages: physical addr is {0:X}")]
    FailedToAllocatePages(u64),
}
