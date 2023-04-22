use core::num::TryFromIntError;

pub type KernelResult<T = ()> = Result<T, KernelError>;

/// Errors emitted from kernel-lib
#[derive(Debug)]
pub enum KernelError {
    ExceededFrameBufferSize,
    NotSupportCharacter,
    FailedCast,
    NumSizeOver,
    FailedOperateLayer(LayerReason),
    FailedAllocate(AllocateReason),
    TryFromIntError(TryFromIntError),
}


#[derive(Debug)]
pub enum LayerReason {
    FailedInititialize,
}


#[derive(Debug)]
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
