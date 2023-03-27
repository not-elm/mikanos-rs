use core::fmt::Debug;

pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    InvalidTrb(u128),
    NullPointer,
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
    OverMaxDeviceSlots { max: u8, specify: u8 },
    HostControllerNotHalted,
    FailedAllocate,
    NotReflectedValue { expect: usize, value: usize },
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
