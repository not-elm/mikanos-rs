pub type KernelResult<T = ()> = core::result::Result<T, KernelError>;

/// Errors emitted from kernel-lib
#[derive(Debug)]
pub enum KernelError {
    ExceededFrameBufferSize,
}
