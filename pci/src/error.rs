use core::fmt::Debug;

pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    InvalidHeaderType(HeaderTypeReason),
    InvalidFunction(FunctionReason),
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
pub enum OperationReason {
    MustBeCommandRingStopped,
    HostControllerNotHalted,
    FailedAllocate,
    NotReflectedValue { value: usize },
    XhcRunning,
}

#[derive(Debug)]
pub enum InvalidRegisterReason {
    IllegalBitFlag { expect: bool },
    HostControllerNotHalted,
    ToSmallCapLength(u8),
}
