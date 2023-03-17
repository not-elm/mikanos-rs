pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    NotSingleFunction,
    NotGeneralHeader,
    ZeroRegister(&'static str),
    CapLengthInvalid(u8),
    DoorbellOffsetInvalid(u32),
}
