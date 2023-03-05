use core::num::TryFromIntError;

pub type KernelResult<T = ()> = core::result::Result<T, KernelError>;

/// Errors emitted from kernel-lib
#[derive(Debug)]
pub enum KernelError {
    ExceededFrameBufferSize,
    NotSupportCharacter,
    TryFromIntError(TryFromIntError),
}
