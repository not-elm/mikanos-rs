use core::fmt::Debug;

pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    InvalidTrb(u128),
    FailedOperateTransferRing,
    NullPointer,
    FailedOperateDeviceContext(DeviceContextReason),
    InvalidHeaderType(HeaderTypeReason),
    InvalidFunction(FunctionReason),
    FailedAllocate(AllocateReason),
    FailedOperateDevice(DeviceReason),
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
pub enum DeviceContextReason {
    NotExistsAddressingPort,
    ExceedMasSlots {
        max_slots: u8,
        specified_slot_id: u8,
    },
}
#[derive(Debug)]
pub enum DeviceReason {
    NotExistsSlot(u8),
    InvalidTargetEvent,
    InvalidPhase { current_phase: usize },
}

#[derive(Debug)]
pub enum OperationReason {
    MustBeCommandRingStopped,
    CommandRingNotRunning,
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
