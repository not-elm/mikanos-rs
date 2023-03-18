pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    NotSingleFunction,
    NotGeneralHeader,
    ZeroRegister(&'static str),
    CapLengthInvalid(u8),
    DoorbellOffsetInvalid(u32),
    RuntimeOffsetInvalid(u32),
    XhcStopping,
    XhcRunning,
    WriteFailedDeviceContextArrayAddrToDCBAAP(u64),
    FailedAllocate,
    FailedWroteSetMaxSlotsEn(u8),
    HostControllerNotHalted,
    HostControllerResetInvalid,
    InvalidControllerNotReadyRegister,
    InvalidRingCycleState,
}
