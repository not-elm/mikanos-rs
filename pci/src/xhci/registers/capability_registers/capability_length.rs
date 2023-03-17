use macros::Volatile;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u8)]
pub struct CapabilityLength(usize);

impl CapabilityLength {
    pub fn new_with_check(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let cap_length = CapabilityLength::new_non_zero(mmio_addr.addr())
            .ok_or(PciError::ZeroRegister("cap_length"))?;
        if cap_length.read_volatile() < 0x20 {
            Err(PciError::CapLengthInvalid(cap_length.read_volatile()))
        } else {
            Ok(cap_length)
        }
    }
}
