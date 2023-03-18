use crate::xhci::registers::capability_registers::capability_length::CapabilityLength;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct OperationRegistersOffset(usize);

impl OperationRegistersOffset {
    pub fn new(mmio_base_addr: MemoryMappedAddr, cap_length: CapabilityLength) -> Self {
        Self(mmio_base_addr.addr() + cap_length.read_volatile() as usize)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}