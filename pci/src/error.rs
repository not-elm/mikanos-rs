use core::fmt::Debug;

pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    InvalidHeaderType(HeaderTypeReason),
    InvalidFunction(FunctionReason),
    FailedAllocate(AllocateReason),
    FailedOperateToRegister(OperationReason),
    InvalidRegister(InvalidRegisterReason),
}

#[derive(Debug)]
pub enum FunctionReason {
    NotSingleFunction,
}

#[derive(Debug)]
pub enum HeaderTypeReason {
    NotGeneralHeader,
}

#[derive(Debug)]
pub enum AllocateReason {
    NotAlignedAddress { expect_align_size: usize },
    NotEnoughMemory,
}

#[derive(Debug)]
pub enum OperationReason {
    MustBeCommandRingStopped,
    HostControllerNotHalted,
    FailedAllocate,
    NotReflectedValue { value: usize },
    XhcRunning,
    ExceedsEventRingSegmentTableMax { max: u32, value: u16 },
}

#[derive(Debug)]
pub enum InvalidRegisterReason {
    IllegalBitFlag { expect: bool },
    HostControllerNotHalted,
    InvalidAddress { specified_address: usize },
    ToSmallCapLength(u8),
}
