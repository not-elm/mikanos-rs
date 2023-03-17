use bitfield_struct::bitfield;

pub type PciResult<T = ()> = Result<T, PciError>;

#[bitfield]
#[derive(Debug)]
pub enum PciError {
    NotSingleFunction,
    NotGeneralHeader,
    ZeroRegister(&'static str),
    CapLengthInvalid(u8),
    DoorbellOffsetInvalid(u32),
    RuntimeOffsetInvalid(u32),
    XhcStoped,
    HostControllerNotHalted,
    HostControllerResetInvalid,
}
