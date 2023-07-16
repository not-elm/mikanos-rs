use core::num::TryFromIntError;

use simple_fat::error::FatError;
use thiserror_no_std::Error;

use common_lib::error::CommonError;
use common_lib::math::rectangle::Rectangle;

pub type KernelResult<T = ()> = Result<T, KernelError>;


/// Errors emitted from kernel-lib
#[derive(Debug, Error)]
pub enum KernelError {
    #[error(transparent)]
    Common(#[from] CommonError),

    #[error(transparent)]
    Fat(#[from] FatError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("Exceeded frame buffer size")]
    ExceededFrameBufferSize,

    #[error("Not supported character")]
    NotSupportCharacter,

    #[error("Failed cast")]
    FailedCast,

    #[error("Num size over")]
    NumSizeOver,

    #[error(transparent)]
    FailedOperateLayer(#[from] LayerReason),

    #[error(transparent)]
    FailedAllocate(#[from] AllocateReason),

    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
}


#[macro_export]
macro_rules! kernel_error {
    ($message: expr) => {
        $crate::error::KernelError::Anyhow(anyhow::anyhow!($message))
    };


    ($fmt: expr, $($args:tt)*) => {
        $crate::kernel_error!(alloc::format!($fmt, $($args)*))
    };
}


#[macro_export]
macro_rules! kernel_bail {
    ($message: expr) => {
        Err($crate::kernel_error!($message))
    };

    ($fmt: expr, $($args:tt)*) => {
        Err($crate::kernel_error!($fmt, $($args)*))
    };
}


#[derive(Debug, PartialEq, Error)]
pub enum LayerReason {
    #[error("Failed initialize")]
    FailedInitialize,
    #[error("Not exists key")]
    NotExistsKey,
    #[error("Illegal layer")]
    IllegalLayer,
    #[error("Window size over expected: {0:?}")]
    WindowSizeOver(Rectangle<usize>),
}


#[derive(Debug, PartialEq, Error)]
pub enum AllocateReason {
    #[error("Failed initialize global allocator")]
    InitializeGlobalAllocator,

    #[error("over frame id: max-frame-id={max_frame_id} but was={frame_id}")]
    OverFrame {
        max_frame_id: usize,
        frame_id: usize,
    },

    #[error("Over address: 0x{address:X}")]
    OverAddress { address: u64 },
}
