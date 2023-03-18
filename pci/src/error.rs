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
    HostControllerNotHalted,
    HostControllerResetInvalid,
}
