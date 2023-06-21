use core::num::TryFromIntError;

use common_lib::math::rectangle::Rectangle;

pub type KernelResult<T = ()> = Result<T, KernelError>;


/// Errors emitted from kernel-lib
#[derive(Debug)]
pub enum KernelError {
    Anyhow(anyhow::Error),
    ExceededFrameBufferSize,
    NotSupportCharacter,
    FailedCast,
    NumSizeOver,
    FailedOperateLayer(LayerReason),
    FailedAllocate(AllocateReason),
    TryFromIntError(TryFromIntError),
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


#[derive(Debug, PartialEq)]
pub enum LayerReason {
    FailedInitialize,
    NotExistsKey,
    IllegalLayer,
    WindowSizeOver(Rectangle<usize>),
}


#[derive(Debug, PartialEq)]
pub enum AllocateReason {
    InitializeGlobalAllocator,
    OverFrame {
        max_frame_id: usize,
        frame_id: usize,
    },
    OverAddress {
        address: u64,
    },
}
