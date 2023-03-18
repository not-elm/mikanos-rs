use macros::VolatileBits;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

#[derive(VolatileBits)]
#[volatile_type(u8)]
pub struct CapabilityLength(usize);

impl CapabilityLength {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let cap_length = CapabilityLength::new_uncheck(mmio_addr.addr());
        if cap_length.read_volatile() < 0x20 {
            Err(PciError::CapLengthInvalid(cap_length.read_volatile()))
        } else {
            Ok(cap_length)
        }
    }
}
