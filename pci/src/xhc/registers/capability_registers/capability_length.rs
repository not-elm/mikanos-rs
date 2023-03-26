use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;

/// CAP LENGTH
///
/// Offset: 0
///
/// MemoryMappedAddress+CAP LengthでOperationalRegistersのベースアドレスになります。
#[derive(VolatileBits)]
#[volatile_type(u8)]
pub struct CapabilityLength(usize);

impl CapabilityLength {
    pub fn new_check_length(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let cap_length = CapabilityLength::new_uncheck(mmio_addr.addr());
        if cap_length.read_volatile() < 0x20 {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::ToSmallCapLength(cap_length.read_volatile()),
            ))
        } else {
            Ok(cap_length)
        }
    }
}
