use crate::xhc::registers::capability_registers::capability_length::CapabilityLength;
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::VolatileAccessible;

/// Address: MemoryMappedAddress + CapLength
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct OperationalRegistersOffset(usize);

impl OperationalRegistersOffset {
    pub fn new(mmio_base_addr: MemoryMappedAddr, cap_length: &CapabilityLength) -> Self {
        Self(mmio_base_addr.addr() + cap_length.read_volatile() as usize)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
