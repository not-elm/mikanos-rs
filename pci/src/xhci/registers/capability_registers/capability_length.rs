use crate::error::{PciError, PciResult};
use macros::Volatile;

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u8)]
pub struct CapabilityLength(usize);

impl CapabilityLength {
    pub(crate) fn new_with_check(addr: usize) -> PciResult<Self> {
        let cap_length =
            CapabilityLength::new_non_zero(addr).ok_or(PciError::ZeroRegister("cap_length"))?;
        if cap_length.read_volatile() < 0x20 {
            Err(PciError::CapLengthInvalid(cap_length.read_volatile()))
        } else {
            Ok(cap_length)
        }
    }
}
